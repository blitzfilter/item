use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString)]
pub enum Currency {
    EUR,
    GBP,
    USD,
    AUD,
    CAD
}
