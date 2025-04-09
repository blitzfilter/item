use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ItemType {
    ORIGINAL,
    REPLICA,
}
