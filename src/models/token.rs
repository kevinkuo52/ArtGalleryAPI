use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleClaims {
    // sub: String,
    // iss: String,
    exp: i32,
    email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JWK {
    pub kid: Option<String>,
    pub n: String,
    pub e: String,
    pub alg: Option<String>,
    pub kty: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JWKS {
    keys: Vec<JWK>
}

impl JWKS {
    /// Attempt to find a JWK by its key ID.
    pub fn find(&self, kid: &str) -> Option<&JWK> {
        self.keys.iter().find(|jwk| jwk.kid == Some(kid.into()))
    }
}