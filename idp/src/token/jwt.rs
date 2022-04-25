use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // Issuer , this idp itself.
    aud: String, // Audience, idp user.
    sub: String, // User identifier.
    iat: i64,    // Timing of issue
    exp: i64,    // expiration time
}

pub fn make_jwt(secret: &str, aud: &str) -> Result<String, String> {
    let header = Header::new(Algorithm::HS256);
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::hours(8)).timestamp();
    let sub = Uuid::new_v4();
    let my_claims = Claims {
        iss: "example_system".to_owned(),
        aud: aud.to_owned(),
        sub: sub.to_string(),
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
