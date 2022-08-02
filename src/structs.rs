use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlugPayload {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlugValue {
    pub url: String,
    pub creator: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserPayload {
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserValueResponse {
    pub username: String,
    pub token: String,
}
