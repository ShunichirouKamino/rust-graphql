use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    iat: i64,
    exp: i64,
}

pub fn make_jwt(secret: &str, aud: &str) -> Result<String, String> {
    let header = Header::new(Algorithm::HS256);
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(8)).timestamp();
    let my_claims = Claims {
        aud: aud.to_owned(),
        sub: "example_system".to_owned(),
        iat,
        exp,
    };
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let token = match encode(&header, &my_claims, &encoding_key) {
        Ok(t) => t,
        Err(_) => panic!("todo panic!"),
    };

    Ok(token)
}
