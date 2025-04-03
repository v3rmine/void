// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::RwLock;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::Stream;
use http::request::Builder as RequestBuilder;
use http::{header, Request, Response};
use query::AsyncQuery;
use serde::de::DeserializeOwned;
use url::Url;

use crate::api::paged::link_header;
use crate::api::{
    query, ApiError, AsyncClient, Client, Endpoint, Pageable, Paged, Query, RestClient,
};

impl<E> Paged<E>
where
    E: Endpoint,
    E: Pageable,
{
    /// Create an iterator over the results of paginated results for with a client.
    pub fn iter<'a, C, T>(&'a self, client: &'a C) -> LazilyPagedIter<'a, E, C, T> {
        LazilyPagedIter::new(self, client)
    }
}

impl<E> Paged<E>
where
    E: Endpoint + Pageable + Sync,
{
    /// Create a stream over the results of paginated results for with a client.
    pub fn iter_async<'a, C, T>(
        &'a self,
        client: &'a C,
    ) -> impl Stream<Item = Result<T, ApiError<C::Error>>> + 'a
    where
        T: DeserializeOwned + 'static,
        C: AsyncClient + Sync,
    {
        let iter = LazilyPagedIter::new(self, client);
        futures_util::stream::unfold(iter, |mut iter| async move {
            iter.next_async().await.map(|item| (item, iter))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum KeysetPage {
    First,
    Next(Url),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Number(u64),
    Keyset(KeysetPage),
    Done,
}

impl Page {
    fn next_url(&self) -> Option<&Url> {
        if let Self::Keyset(KeysetPage::Next(url)) = self {
            Some(url)
        } else {
            None
        }
    }

    fn next_page(&mut self, next_url: Option<Url>) {
        let next_page = match *self {
            Self::Number(page) => Self::Number(page + 1),
            Self::Keyset(_) => {
                if let Some(next_url) = next_url {
                    Self::Keyset(KeysetPage::Next(next_url))
                } else {
                    Self::Done
                }
            },
            Self::Done => Self::Done,
        };

        *self = next_page;
    }

    fn apply_to(&self, pairs: &mut url::form_urlencoded::Serializer<'_, url::UrlQuery<'_>>) {
        match self {
            Self::Number(page) => {
                let page_str = page.to_string();
                pairs.append_pair("page", &page_str);
            },
            Self::Keyset(_) => {
                pairs.append_pair("pagination", "keyset");
            },
            Self::Done => {
                unreachable!("The `Done` state should not be applied to any url")
            },
        }
    }
}

struct PageState {
    total_results: usize,
    next_page: Page,
}

struct LazilyPagedState<'a, E> {
    paged: &'a Paged<E>,
    page_state: RwLock<PageState>,
}

impl<'a, E> LazilyPagedState<'a, E>
where
    E: Pageable,
{
    fn new(paged: &'a Paged<E>) -> Self {
        let next_page = if paged.endpoint.use_keyset_pagination() {
            Page::Keyset(KeysetPage::First)
        } else {
            Page::Number(1)
        };

        let page_state = PageState {
            total_results: 0,
            next_page,
        };

        Self {
            paged,
            page_state: RwLock::new(page_state),
        }
    }
}

impl<'a, E> LazilyPagedState<'a, E> {
    fn next_page(&self, last_page_size: usize, next_url: Option<Url>) {
        let mut page_state = self.page_state.write().expect("poisoned next_page");
        page_state.total_results += last_page_size;

        // Gitlab used to have issues returning paginated results; these have been fixed since, but
        // if it is needed, the bug manifests as Gitlab returning *all* results instead of just the
        // requested results. This can cause an infinite loop here if the number of total results
        // is exactly equal to `per_page`.
        if self
            .paged
            .pagination
            .is_last_page(last_page_size, page_state.total_results)
        {
            page_state.next_page = Page::Done;
        } else {
            page_state.next_page.next_page(next_url);
        }
    }
}

impl<'a, E> LazilyPagedState<'a, E>
where
    E: Endpoint,
{
    fn page_url<C>(&self, client: &C) -> Result<Option<Url>, ApiError<C::Error>>
    where
        C: RestClient,
    {
        let page_state = self.page_state.read().expect("poisoned next_page");
        let next_page = &page_state.next_page;

        if *next_page == Page::Done {
            return Ok(None);
        }

        let url = if let Some(next_url) = next_page.next_url() {
            next_url.clone()
        } else {
            let mut url = client.rest_endpoint(&self.paged.endpoint.endpoint())?;
            self.paged.endpoint.parameters().add_to_url(&mut url);

            let per_page = self.paged.pagination.page_limit();
            let per_page_str = per_page.to_string();

            {
                let mut pairs = url.query_pairs_mut();
                pairs.append_pair("per_page", &per_page_str);

                next_page.apply_to(&mut pairs);
            }

            url
        };

        Ok(Some(url))
    }

    fn build_request<C>(&self, url: Url) -> Result<(RequestBuilder, Vec<u8>), ApiError<C::Error>>
    where
        C: RestClient,
    {
        let body = self.paged.endpoint.body()?;

        let req = Request::builder()
            .method(self.paged.endpoint.method())
            .uri(query::url_to_http_uri(url));
        Ok(if let Some((mime, data)) = body.as_ref() {
            let req = req.header(header::CONTENT_TYPE, *mime);
            (req, data.clone())
        } else {
            (req, Vec::new())
        })
    }

    fn process_response<C, T>(&self, rsp: Response<Bytes>) -> Result<Vec<T>, ApiError<C::Error>>
    where
        E: Pageable,
        T: DeserializeOwned,
        C: RestClient,
    {
        let status = rsp.status();

        let next_url = if self.paged.endpoint.use_keyset_pagination() {
            link_header::next_page_from_headers(rsp.headers())?
        } else {
            None
        };

        let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
            v
        } else {
            return Err(ApiError::server_error(status, rsp.body()));
        };
        if !status.is_success() {
            return Err(ApiError::from_gitlab(v));
        }

        let page = serde_json::from_value::<Vec<T>>(v).map_err(ApiError::data_type::<Vec<T>>)?;
        self.next_page(page.len(), next_url);

        Ok(page)
    }
}

impl<'a, E, T, C> Query<Vec<T>, C> for LazilyPagedState<'a, E>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = if let Some(url) = self.page_url(client)? {
            url
        } else {
            // Just return empty data.
            // XXX: Return a new kind of PaginationError here?
            return Ok(Vec::new());
        };
        let (req, data) = self.build_request::<C>(url)?;
        let rsp = client.rest(req, data)?;
        self.process_response::<C, _>(rsp)
    }
}

#[async_trait]
impl<'a, E, T, C> AsyncQuery<Vec<T>, C> for LazilyPagedState<'a, E>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<Vec<T>, ApiError<C::Error>> {
        let url = if let Some(url) = self.page_url(client)? {
            url
        } else {
            // Just return empty data.
            // XXX: Return a new kind of PaginationError here?
            return Ok(Vec::new());
        };
        let (req, data) = self.build_request::<C>(url)?;
        let rsp = client.rest_async(req, data).await?;
        self.process_response::<C, _>(rsp)
    }
}

/// An iterator which yields items from a paginated result.
///
/// The pages are fetched lazily, so endpoints not using keyset pagination may observe duplicate or
/// missing items (depending on sorting) if new objects are created or removed while iterating.
pub struct LazilyPagedIter<'a, E, C, T> {
    client: &'a C,
    state: LazilyPagedState<'a, E>,
    current_page: Vec<T>,
}

impl<'a, E, C, T> LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint,
    E: Pageable,
{
    fn new(paged: &'a Paged<E>, client: &'a C) -> Self {
        let state = LazilyPagedState::new(paged);

        Self {
            client,
            state,
            current_page: Vec::new(),
        }
    }
}

impl<'a, E, C, T> Iterator for LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint,
    E: Pageable,
    T: DeserializeOwned,
    C: Client,
{
    type Item = Result<T, ApiError<C::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query(self.client) {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };

            // Reverse the page order so that `.pop()` works.
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

// Instead of implementing Stream directly, we implement this "async" next method and use it with
// `stream::unfold` to return an anonymous Stream impl.
impl<'a, E, C, T> LazilyPagedIter<'a, E, C, T>
where
    E: Endpoint + Pageable + Sync,
    T: DeserializeOwned + 'static,
    C: AsyncClient + Sync,
{
    async fn next_async(&mut self) -> Option<Result<T, ApiError<C::Error>>> {
        if self.current_page.is_empty() {
            self.current_page = match self.state.query_async(self.client).await {
                Ok(data) => data,
                Err(err) => return Some(Err(err)),
            };

            // Reverse the page order so that `.pop()` works.
            self.current_page.reverse();
        }

        self.current_page.pop().map(Ok)
    }
}

#[cfg(test)]
mod tests {
    use futures_util::TryStreamExt;
    use http::StatusCode;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::api::endpoint_prelude::*;
    use crate::api::{self, ApiError, Pagination};
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::OK);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[tokio::test]
    async fn test_gitlab_non_json_response_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "not json");
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::GitlabService { status, .. } = err {
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[tokio::test]
    async fn test_gitlab_error_bad_json_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("page", "1"), ("per_page", "100")])
            .status(StatusCode::NOT_FOUND)
            .build()
            .unwrap();
        let client = SingleTestClient::new_raw(endpoint, "");
        let endpoint = Dummy::default();

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[tokio::test]
    async fn test_gitlab_error_detection_async() {
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::Gitlab { msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[tokio::test]
    async fn test_gitlab_error_detection_legacy_async() {
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter(&client)
            .collect();
        let err = res.unwrap_err();
        if let ApiError::GitlabUnrecognized { obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[tokio::test]
    async fn test_gitlab_error_detection_unknown_async() {
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

        let res: Result<Vec<DummyResult>, _> = api::paged(endpoint, Pagination::All)
            .iter_async(&client)
            .try_collect()
            .await;
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
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
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
            .iter_async(&client)
            .try_collect()
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

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
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
            .iter_async(&client)
            .try_collect()
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
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(res.len(), 25);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn test_keyset_pagination_limit_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy { with_keyset: true };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(25))
            .iter_async(&client)
            .try_collect()
            .await
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

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn test_keyset_pagination_all_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .paginated(true)
            .build()
            .unwrap();
        let client =
            PagedTestClient::new_raw(endpoint, (0..=255).map(|value| DummyResult { value }));
        let query = Dummy { with_keyset: true };

        let res: Vec<DummyResult> = api::paged(query, Pagination::All)
            .iter_async(&client)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
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
            .iter(&client)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }

    #[tokio::test]
    async fn test_keyset_pagination_missing_header_async() {
        let endpoint = ExpectedUrl::builder()
            .endpoint("paged_dummy")
            .add_query_params(&[("pagination", "keyset"), ("per_page", "100")])
            .build()
            .unwrap();
        let data: Vec<_> = (0..=255).map(|value| DummyResult { value }).collect();
        let client = SingleTestClient::new_raw(endpoint, serde_json::to_vec(&data).unwrap());
        let query = Dummy { with_keyset: true };

        let res: Vec<DummyResult> = api::paged(query, Pagination::Limit(300))
            .iter_async(&client)
            .try_collect()
            .await
            .unwrap();
        assert_eq!(res.len(), 256);
        for (i, value) in res.iter().enumerate() {
            assert_eq!(value.value, i as u8);
        }
    }
}
