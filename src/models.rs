// src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
  pub id: Uuid,
  pub name: String,
  pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUpdateItem {
    pub name: String,
    pub description: String,
}