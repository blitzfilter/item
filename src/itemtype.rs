use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString, PartialEq, Debug, Eq)]
pub enum ItemType {
    ORIGINAL,
    REPLICA,
}
