use jwks_client::{
    keyset::KeyStore,
    jwt::Payload
};
use std::{error::Error, fmt};
use crate::auth::JWTError::{InvalidJWTFormat, InvalidSignature, InvalidMetadata};
use liquidity::context::User;

/// Authentication validator
///
/// This allows validation of use authentication tokens
pub struct JWTAuth {
    jwks_store: KeyStore,
    issuer: String,
    audience: String
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum JWTError {
    InvalidSignature(jwks_client::error::Error),
    InvalidJWTFormat(String),
    InvalidMetadata(String),
    NotAToken
}

impl fmt::Display for JWTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JWTError::InvalidSignature(e) => write!(f, "Invalid JWT signature: {}", e.msg),
            JWTError::InvalidJWTFormat(e) => write!(f, "Invalid JWT format: {}", e),
            JWTError::InvalidMetadata(e) => write!(f, "Invalid JWT data: {}", e),
            JWTError::NotAToken => write!(f, "Authorization header is not a JWT token")
        }
    }
}

impl Error for JWTError {}

fn audience_valid(aud: &String, payload: &Payload) -> Result<(), JWTError> {
    let audiences = payload.get_array("aud");
    let valid = match audiences {
        Some(audiences) => {
            let audiences: Result<Vec<String>, JWTError> = audiences.iter()
                .map(|x| {
                    let result = x.as_str()
                        .ok_or(InvalidJWTFormat("Audiences array contains non-strings".to_string()))
                        .map(|s| s.to_string());
                    result
                })
                .collect();
            audiences?.contains(aud)
        },
        None => {
            let audience = payload.aud().ok_or(InvalidJWTFormat("Missing audience from JWT".to_string()))?;
            audience.eq(aud)
        }
    };
    match valid {
        true => Ok(()),
        false => Err(InvalidMetadata("Token wasn't issued for this service".to_string()))
    }
}

fn issuer_valid(iss: &String, payload: &Payload) -> Result<(), JWTError> {
    let issuer = payload.iss().ok_or(InvalidJWTFormat("Missing issuer from JWT".to_string()))?;
    match issuer.eq(iss) {
        true => Ok(()),
        false => Err(InvalidMetadata("Token wasn't issued by a trusted party".to_string()))
    }
}

fn parse_user(payload: &Payload) -> Result<User, JWTError> {
    let id = payload.sub()
        .ok_or(InvalidJWTFormat("Missing subject from JWT".to_string()))?
        .to_string();
    let empty = Vec::new();
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
    #[instrument]
    pub fn validate(&self, token: String) -> Result<User, JWTError> {
        if !token.starts_with("Bearer ") { return Err(JWTError::NotAToken) }
        let token = token.replace("Bearer ", "");
        let decoded = {
            let span = trace_span!("verify_token");
            let _enter = span.enter();
            let res = self.jwks_store.verify(token.as_str()).map_err(InvalidSignature);
            if let Err(e) = &res { error!("{:?}", e) }
            res
        }?;
        audience_valid(&self.audience, decoded.payload())?;
        issuer_valid(&self.issuer, decoded.payload())?;

        parse_user(decoded.payload())
    }
}

impl fmt::Debug for JWTAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "aud: {}, iss: {}", self.audience, self.issuer)
    }
}