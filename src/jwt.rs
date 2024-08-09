use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: Uuid,
    pub aud: String,
    pub exp: u64,
    pub nbf: u64,
    pub iat: u64,
    pub jti: Uuid,
}

pub struct JwtSigner {
    key: EncodingKey,
    kid: String,
    aud: String,
    iss: String,
}

impl JwtSigner {
    pub fn new(key: String, kid: String, aud: String, iss: String) -> Self {
        JwtSigner {
            key: EncodingKey::from_secret(key.as_bytes()),
            kid,
            aud,
            iss,
        }
    }

    pub fn sign(&self, user_id: Uuid, ttl: Duration) -> Result<(String, u64), anyhow::Error> {
        let now = SystemTime::now();

        // If this fails, it means system is in terrible condition, or
        // you are traveling with the Doctor in the TARDIS.
        let iat = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = (now + ttl).duration_since(UNIX_EPOCH).unwrap().as_secs();

        let header = Header {
            alg: Algorithm::HS256,
            kid: Some(self.kid.clone()),
            ..Default::default()
        };

        let claims = Claims {
            iss: self.iss.clone(),
            sub: user_id,
            aud: self.aud.clone(),
            exp,
            nbf: iat,
            iat,
            jti: Default::default(),
        };

        let token = encode(&header, &claims, &self.key)?;

        Ok((token, exp))
    }
}
