use actix_web::HttpResponse;
use actix_web_codegen::get;

/**
 * @todo Create the method in daemon-hume
 */
#[get("/sa")]
pub fn sync_get_sa() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
