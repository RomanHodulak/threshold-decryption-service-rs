use actix_web::web::{Bytes, Data};
use actix_web::{post, HttpResponse, Responder, ResponseError};
use anyhow::Error;
use rsa::pkcs8::DecodePrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct EncryptedMessage(Arc<RwLock<Bytes>>);

impl EncryptedMessage {
    pub fn empty() -> Self {
        Self(Arc::new(RwLock::new(Bytes::new())))
    }

    pub fn new(content: impl Into<Bytes>) -> Self {
        Self(Arc::new(RwLock::new(content.into())))
    }

    pub fn replace(&self, content: Bytes) {
        *self.0.write().unwrap() = content;
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.read().unwrap().to_vec()
    }
}

#[derive(Clone, Debug)]
pub struct Shares(Arc<RwLock<Vec<Bytes>>>);

impl Shares {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn add(&self, content: Bytes) {
        self.0.write().unwrap().push(content);
    }

    pub fn clear(&self) {
        self.0.write().unwrap().clear();
    }

    pub fn count(&self) -> usize {
        self.0.read().unwrap().len()
    }

    pub fn collect(&self) -> Vec<Vec<u8>> {
        self.0
            .read()
            .unwrap()
            .iter()
            .map(|share| share.to_vec())
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Threshold(u8);

impl Threshold {
    pub fn new(number: u8) -> Self {
        Self(number)
    }
}

#[post("/decrypt/start")]
async fn start_decryption(
    encrypted: Bytes,
    message: Data<EncryptedMessage>,
    shares: Data<Shares>,
) -> impl Responder {
    message.replace(encrypted);
    shares.clear();

    HttpResponse::NoContent()
}

#[post("/decrypt/add")]
async fn add_private_key_share(key: Bytes, shares: Data<Shares>) -> impl Responder {
    shares.add(key);

    HttpResponse::Ok().body(format!("{}", shares.count()))
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct DecryptionError(anyhow::Error);

impl From<anyhow::Error> for DecryptionError {
    fn from(value: Error) -> Self {
        DecryptionError(value)
    }
}

impl ResponseError for DecryptionError {}

#[post("/decrypt/read")]
async fn finish_decryption(
    threshold: Data<Threshold>,
    message: Data<EncryptedMessage>,
    shares: Data<Shares>,
) -> impl Responder {
    match shamir::SecretData::recover_secret(threshold.0, shares.collect()) {
        Some(private_key) => {
            let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key)
                .map_err(anyhow::Error::from)
                .map_err(DecryptionError::from)?;
            let decrypted = private_key
                .decrypt(Pkcs1v15Encrypt, &message.to_vec())
                .map_err(anyhow::Error::from)
                .map_err(DecryptionError::from)?;

            Ok(
                if let Ok(decrypted) = String::from_utf8(decrypted.clone()) {
                    HttpResponse::Ok().body(decrypted)
                } else {
                    HttpResponse::Ok().body(decrypted)
                },
            )
        }
        None => HttpResponse::BadRequest().await.into(),
    }
}
