use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString, EnumIter, Eq, PartialEq, Debug)]
pub enum ItemState {
    LISTED,
    AVAILABLE,
    RESERVED,
    SOLD,
    REMOVED,
}
