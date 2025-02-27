use actix_web::{HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::models::claim::Claim;

pub async fn login() -> impl Responder {
    let claim = Claim {
        sub: "user_id".to_owned(),
        exp: 10000000000,
    };

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .expect("Token Creation failed");

    HttpResponse::Ok().body(token)
}
