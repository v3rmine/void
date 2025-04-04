use crate::library::methods::add_config;
use crate::library::reponses::{Response, ResponseTypes};
use crate::library::requests::CreateConf;
use crate::library::structs::RcloneConfig;
use actix_web::{web::Json, HttpResponse};
use actix_web_codegen::post;

#[post("/conf")]
pub fn post_conf(params: Json<CreateConf>) -> actix_web::Result<HttpResponse> {
    let unc_client_id: String = decrypt!(params.client_id.clone()).map_err(|_| ())?;
    let unc_client_secret: String = decrypt!(params.client_secret.clone()).map_err(|_| ())?;
    let unc_service_account_file: String =
        decrypt!(params.service_account_file.clone()).map_err(|_| ())?;
    println!("{}", unc_client_id);
    println!("{}", unc_client_secret);
    println!("{}", unc_service_account_file);
    Ok(
        match add_config(
            RcloneConfig {
                r#type: params.r#type.clone().unwrap_or("drive".to_owned()),
                sa_id: params.sa_id.clone(),
                client_id: unc_client_id,
                client_secret: unc_client_secret,
                scope: params.scope.clone().unwrap_or("drive".to_owned()),
                service_account_file: unc_service_account_file,
                trashed_only: params.trashed_only.clone().unwrap_or(false),
                use_trash: params.use_trash.clone().unwrap_or(true),
                shared_with_me: params.shared_with_me.clone().unwrap_or(false),
                list_chunk: params.list_chunk.clone().unwrap_or(500),
                chunk_size: params.chunk_size.clone().unwrap_or(2),
                pacer_burst: params.pacer_burst.clone().unwrap_or(250),
                team_drive: params.team_drive.clone().unwrap_or("0APOJta3pYSKaUk9PVA".to_owned())
            },
            false,
        ) {
            Ok(_) => response!(
                Ok,
                Response::ok(
                    200,
                    ResponseTypes::PlainText("Le SA à été ajouté".to_owned())
                )
            ),
            Err(_) => response!(
                InternalServerError,
                Response::err(500, "Internal Server Error", "Erreur d'ajout de config")
            ),
        },
    )
}
