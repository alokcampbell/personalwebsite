use actix_files::NamedFile;
use actix_web::{web::ServiceConfig, HttpRequest, Responder};
use shuttle_actix_web::ShuttleActixWeb;
use std::path::PathBuf;
use actix_web::get;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open(PathBuf::from("static/index.html"))
}


#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            actix_web::web::scope("")
                .service(index)
                .service(actix_files::Files::new("/static", "static/").index_file("index.html"))
                .wrap(actix_web::middleware::Logger::default()),
        );
    };

    Ok(config.into())
}