use jwks_client::keyset::KeyStore;
use jwks_client::jwt::Payload;
use failure::Error;
use crate::auth::JWTError::{InvalidJWTFormat, InvalidSignature};
use crate::graphql::context::User;
use serde_json::Value;
use serde::Serialize;

/// Authentication validator
///
/// This allows validation of use authentication tokens
pub struct JWTAuth {
    jwks_store: KeyStore,
    issuer: String,
    audience: String
}

#[derive(Debug, Fail, Serialize)]
#[allow(clippy::enum_variant_names)]
pub enum JWTError {
    #[fail(display = "JWT Token failed to validate")]
    InvalidSignature,
    #[fail(display = "Invalid request: {}", reason)]
    InvalidRequestFormat {reason: String},
    #[fail(display = "Invalid JWT format: {}", reason)]
    InvalidJWTFormat {reason: String}
}


fn audience_valid(aud: &String, payload: &Payload) -> Result<bool, Error> {
    let audiences = payload.get_array("aud");
    match audiences {
        Some(audiences) => {
            let audiences: Result<Vec<String>, JWTError> = audiences.iter()
                .map(|x| {
                    let result = x.as_str()
                        .ok_or(InvalidJWTFormat { reason: "Audiences array contains non-strings".to_string() })
                        .map(|s| s.to_string());
                    result
                })
                .collect();
            Ok(audiences?.contains(aud))
        },
        None => {
            let audience = payload.aud().ok_or(InvalidJWTFormat {reason: "Missing audience from JWT".to_string()})?;
            Ok(audience.eq(aud))
        }
    }
}

fn parse_user(payload: &Payload) -> Result<User, Error> {
    let id = payload.sub()
        .ok_or(InvalidJWTFormat {reason: "Missing subject from JWT".to_string()})?
        .to_string();
    let empty = Vec::<Value>::new();
    let permissions = payload
        .get_array("permissions")
        .unwrap_or_else(|| &empty)
        .iter()
        .map(|x| x.as_str().expect("Can't convert permission to string").to_string())
        .collect();
    Ok(User {
        id,
        permissions
    })
}

impl JWTAuth {
    /// Creates a new JWT authentication validator
    ///
    /// # Parameters
    ///
    /// * `jwks_store` - A JWKS keystore used to validate the token
    /// * `issuer` - The issuer (iss) expected to be on the token
    /// * `audience` - The audience (aud) expected to be on the token
    ///
    /// # Returns
    ///
    /// An instance of the JWT authentication validator
    ///
    /// # Example
    ///
    /// ```
    /// # use jwks_client::keyset::KeyStore;
    /// use backend_rust::auth::JWTAuth;
    /// let jwks_keys = KeyStore::new();
    ///
    /// let issuer = "test_iss".to_string();
    /// let audience = "test_aud".to_string();
    ///
    /// let auth = JWTAuth::new(jwks_keys, issuer, audience);
    ///
    /// let invalid_token = auth.validate("asd".to_string());
    ///
    /// assert!(invalid_token.is_err());
    /// ```
    pub fn new(jwks_store: KeyStore, issuer: String, audience: String) -> Self {
        JWTAuth {
            jwks_store,
            issuer,
            audience
        }
    }

    /// Validate a user's JWT token
    ///
    /// # Parameters
    ///
    /// * `token` - The JWT token passed by the user
    ///
    /// # Returns
    ///
    /// The parsed user if the token was valid, or an error if something went wrong
    ///
    /// # Example
    ///
    /// ```
    /// # use jwks_client::keyset::KeyStore;
    /// use backend_rust::auth::JWTAuth;
    /// let jwks_keys = KeyStore::new();
    ///
    /// let issuer = "test_iss".to_string();
    /// let audience = "test_aud".to_string();
    ///
    /// let auth = JWTAuth::new(jwks_keys, issuer, audience);
    ///
    /// let invalid_token = auth.validate("asd".to_string());
    ///
    /// assert!(invalid_token.is_err());
    /// ```
    pub fn validate(&self, token: String) -> Result<User, Error> {
        let decoded = self.jwks_store.verify(token.as_str()).map_err(|_| InvalidSignature)?;
        let audience_valid = audience_valid(&self.audience, decoded.payload())?;
        let issuer_valid = decoded.payload().iss().ok_or(InvalidJWTFormat {reason: "Missing issuer from JWT".to_string()})? == self.issuer;
        let valid = audience_valid && issuer_valid;

        match valid {
            true => parse_user(decoded.payload()),
            false => Err(InvalidJWTFormat {reason: "Token wasn't issued for this service!".to_string()}.into())
        }
    }
}