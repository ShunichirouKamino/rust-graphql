use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
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
    let my_claims = Claims {
        iss: "example_system".to_owned(),
        aud: aud.to_owned(),
        sub: aud.to_owned(),
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

pub fn decode_jwt(secret: &str, token: &str, aud: &str) -> Result<Claims, String> {
    let mut validation = Validation::new(Algorithm::HS256);
    let decode_key = DecodingKey::from_secret(secret.as_ref());
    validation.sub = Some(aud.to_string());
    validation.set_audience(&[aud]);
    let token_data = match decode::<Claims>(&token, &decode_key, &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };

    Ok(token_data.claims)
}
