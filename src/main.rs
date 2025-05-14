use actix_files::Files;
use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = |cfg: &mut ServiceConfig| {
        cfg.service(Files::new("/", "htmlfiles").index_file("index.html"));
    };

    Ok(config.into())
}