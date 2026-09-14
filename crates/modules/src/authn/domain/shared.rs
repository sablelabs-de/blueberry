use crate::authn::domain::{
    access_tokens::models::access_token::AccessToken,
    sessions::models::refresh_token::RefreshToken,
};

pub struct TokenPair {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
