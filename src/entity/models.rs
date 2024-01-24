
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub(crate) struct GroundDetails {
    pub(crate) ground_id: String,
    pub(crate) ground_name: String,
    pub(crate) ground_address: String,
}
