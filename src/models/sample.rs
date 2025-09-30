use serde::{Deserialize, Serialize};

// Domain model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sample {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub created_by: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleInput {
    pub name: String,
    pub description: Option<String>,
    pub status: String,
}
