use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ChargingStation {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub availability: bool
}

#[derive(Serialize)]
pub struct CreatedResponse {
    pub id: String
}
