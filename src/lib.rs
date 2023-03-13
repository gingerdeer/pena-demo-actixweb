use actix_web::{get, web::ServiceConfig};
use actix_files::NamedFile;
use shuttle_service::ShuttleActixWeb;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;


#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello (Rust) World!"
}

#[get("/test")]
async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "../build/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[shuttle_service::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Sync + Send + Clone + 'static> {
    Ok(move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(index);
    })
}
