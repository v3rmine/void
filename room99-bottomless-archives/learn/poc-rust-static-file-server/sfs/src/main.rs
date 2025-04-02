use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{
    http::header, http::Method, http::StatusCode, middleware, web, App, HttpRequest, HttpResponse,
    HttpServer, ResponseError,
};

async fn index(req: HttpRequest) -> Result<fs::NamedFile, ServerErrors> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    if path.starts_with("/") {
        return Err(ServerErrors::AbsolutePath);
    } else if path.starts_with(".") {
        return Err(ServerErrors::RelativeParentAccess);
    }

    let file =
        fs::NamedFile::open(path).map_err(|source| ServerErrors::CannotAccessFile { source })?;
    let resp = file.use_etag(true).use_last_modified(true);

    if req.headers().get("Range").is_some() || req.method() == Method::HEAD {
        Ok(resp.set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
    } else {
        Ok(resp)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cpus = num_cpus::get();

    println!("Server starting on 0.0.0.0:80 and {} threads", cpus);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(web::resource("/{filename:.*}").to(index))
    })
    .bind("0.0.0.0:80")?
    .workers(cpus)
    .run()
    .await
}

/* === errors part === */

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum ServerErrors {
    AbsolutePath,
    RelativeParentAccess,
    CannotAccessFile { source: std::io::Error },
}

impl Display for ServerErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServerErrors::AbsolutePath => "Absolute path is forbidden",
                ServerErrors::RelativeParentAccess => "Relative access to parent is forbidden",
                ServerErrors::CannotAccessFile { source: _ } => "404 not found",
            }
        )
    }
}

impl Error for ServerErrors {}

impl ResponseError for ServerErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerErrors::AbsolutePath => StatusCode::FORBIDDEN,
            ServerErrors::RelativeParentAccess => StatusCode::FORBIDDEN,
            ServerErrors::CannotAccessFile { source: _ } => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        actix_web::dev::HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
}
