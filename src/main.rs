use actix_files::NamedFile;
use actix_web::{web::ServiceConfig, HttpRequest, Responder};
use shuttle_actix_web::ShuttleActixWeb;

async fn index(_req: HttpRequest) -> impl Responder {
    NamedFile::open("static/index.html")
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = |cfg: &mut ServiceConfig| {
        cfg.default_service(actix_web::web::to(index));
    };

    Ok(config.into())
}