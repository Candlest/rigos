
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RigosConfig {
    pub title : String,
    pub description : String,
    pub author : String,
    pub theme : String,
}