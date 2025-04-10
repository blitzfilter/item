use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString, Eq, PartialEq, Debug)]
pub enum Currency {
    EUR,
    GBP,
    USD,
    AUD,
    CAD
}
