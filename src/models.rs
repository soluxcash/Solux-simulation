use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoluxProfile {
    pub id: String,
    pub name: String,
    pub role: String,
    pub company: String,
    pub email: String,
    pub website: String,
}
