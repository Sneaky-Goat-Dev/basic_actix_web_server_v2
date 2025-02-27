use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
}
