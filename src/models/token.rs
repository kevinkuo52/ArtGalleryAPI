use serde::{Serialize, Deserialize};
use chrono::prelude::*;

mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    #[serde(with = "jwt_numeric_date")]
    pub iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    pub exp: DateTime<Utc>,
}
impl Claims {
    /// If a token should always be equal to its representation after serializing and deserializing
    /// again, this function must be used for construction. `DateTime` contains a microsecond field
    /// but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes the
    /// timestamps.
    pub fn new(sub: String, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        let exp = exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);
        Self { sub, iat, exp }
    }
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