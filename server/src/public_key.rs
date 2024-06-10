use actix_files::NamedFile;
use actix_web::web::Data;
use actix_web::{get, Responder};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct PublicKey(PathBuf);

impl PublicKey {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }
}

#[get("/public_key")]
async fn public_key(path: Data<PublicKey>) -> impl Responder {
    NamedFile::open_async(&path.0).await
}
