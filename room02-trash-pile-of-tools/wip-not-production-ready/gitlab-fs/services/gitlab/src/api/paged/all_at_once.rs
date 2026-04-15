// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use http::{header, Request};
use serde::de::DeserializeOwned;

use crate::api::paged::link_header;
use crate::api::{
    query, ApiError, AsyncClient, AsyncQuery, Client, Endpoint, Pageable, Pagination, Query,
};

/// A query modifier that paginates an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(in crate::api::paged) endpoint: E,
    pub(in crate::api::paged) pagination: Pagination,
}

/// Collect data from a paged endpoint.
pub fn paged<E>(endpoint: E, pagination: Pagination) -> Paged<E> {
    Paged {
        endpoint,
        pagination,
    }
}

impl<E, T, C> Query<Vec<T>, C> for Paged<E>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        self.iter(client).collect()
    }
}

#[async_trait]
impl<E, T, C> AsyncQuery<Vec<T>, C> for Paged<E>
where
    E: Endpoint + Sync,
    E: Pageable,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = {
            let mut url = client.rest_endpoint(&self.endpoint.endpoint())?;
            self.endpoint.parameters().add_to_url(&mut url);
            url
        };

        let mut page_num = 1;
        let per_page = self.pagination.page_limit();
        let per_page_str = per_page.to_string();

        let results = Arc::new(Mutex::new(Vec::new()));
        let mut next_url = None;
        let use_keyset_pagination = self.endpoint.use_keyset_pagination();

        let body = self.endpoint.body()?;

        loop {
            let page_url = if let Some(url) = next_url.take() {
                url
            } else {
                let page_str = page_num.to_string();
                let mut page_url = url.clone();

                {
                    let mut pairs = page_url.query_pairs_mut();
                    pairs.append_pair("per_page", &per_page_str);

                    if use_keyset_pagination {
                        pairs.append_pair("pagination", "keyset");
                    } else {
                        pairs.append_pair("page", &page_str);
                    }
                }

                page_url
            };

            let req = Request::builder()
                .method(self.endpoint.method())
                .uri(query::url_to_http_uri(page_url));
            let (req, data) = if let Some((mime, data)) = body.as_ref() {
                let req = req.header(header::CONTENT_TYPE, *mime);
                (req, data.clone())
            } else {
                (req, Vec::new())
            };
            let rsp = client.rest_async(req, data).await?;
            let status = rsp.status();

            if use_keyset_pagination {
                next_url = link_header::next_page_from_headers(rsp.headers())?;
            }

            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(status, rsp.body()));
            };
            if !status.is_success() {
                return Err(ApiError::from_gitlab(v));
            }

            let page =
                serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;
            let page_len = page.len();

            // Gitlab used to have issues returning paginated results; these have been fixed since,
            // but if it is needed, the bug manifests as Gitlab returning *all* results instead of
            // just the requested results. This can cause an infinite loop here if the number of
            // total results is exactly equal to `per_page`.
            let is_last_page = {
                let mut locked_results = results.lock().expect("poisoned results");
                locked_results.extend(page);
                self.pagination.is_last_page(page_len, locked_results.len())
            };
            if is_last_page {
                break;
            }

            if use_keyset_pagination {
                if next_url.is_none() {
                    break;
                }
            } else {
                page_num += 1;
            }
        }

        let mut locked_results = results.lock().expect("poisoned results");
        Ok(std::mem::take(&mut locked_results))
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::api::endpoint_prelude::*;
    use crate::api::{self, ApiError, AsyncQuery, Pagination, Query};
    use crate::test::client::{ExpectedUrl, PagedTestClient, SingleTestClient};

    #[derive(Debug, Default)]
    struct Dummy {
        with_keyset: bool,
    }

    impl Endpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "paged_dummy".into()
        }
    }

    impl Pageable for Dummy {
        fn use_keyset_pagination(&self) -> bool {
            self.with_keyset
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct DummyResult {
        value: u8,
    }

    #[test]
    fn test_gitlab_non_json_response() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_bad_json() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "message": "dummy error message",
            }),
        );
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection_legacy() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_json(
            endpoint,
            &json!({
                "error": "dummy error message",
            }),
        );
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_gitlab_error_detection_unknown() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let err_obj = json!({
            "bogus": "dummy error message",
        });
        let client = SingleTestClient::new_json(endpoint, &err_obj);
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All).query(&client);
        let err = res.unwrap_err();
        if let ApiError::GitlabUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_pagination_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy { with_keyset: false };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query(&client)
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn test_pagination_limit_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy { with_keyset: false };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[test]
    fn test_pagination_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy::default();

        let res: Vec<DummyResult> = api::paged(query, Pagination::All).query(&client).unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn test_pagination_all_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy::default();

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .query_async(&client)
            .await
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[test]
    fn test_keyset_pagination_limit() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy { with_keyset: true };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .query(&client)
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[test]
    fn test_keyset_pagination_all() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy { with_keyset: true };

        let res: Vec<DummyResult> = api::paged(query, Pagination::All).query(&client).unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[test]
    fn test_keyset_pagination_missing_header() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("pagination", "keyset"), ("per_page", "100")])
            .build()
            .unwrap();
        let data: Vec<_> = (0..=255).map(|value| DummyResult { value }).collect();
        let client = SingleTestClient::new_raw(endpoint, serde_json::to_vec(&data).unwrap());
        let query = Dummy { with_keyset: true };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(300))
            .query(&client)
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }
}
