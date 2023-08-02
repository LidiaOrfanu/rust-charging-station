use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ChargingStation {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub availability: bool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CreateChargingStation {
    pub name: String,
    pub location: String,
    pub availability: bool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UpdateChargingStation {
    pub name: Option<String>,
    pub location: Option<String>,
    pub availability: Option<bool>,
}

#[derive(Serialize)]
pub struct CreatedResponse {
    pub id: String,
}
