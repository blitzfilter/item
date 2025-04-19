use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

// ISO 4217
#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString, Eq, PartialEq, Debug)]
pub enum Currency {
    EUR,
    GBP,
    USD,
    AUD,
    CAD,
}

pub type EuroConversionRate = fn(Currency) -> f32;

pub const DEFAULT_CONVERSION_RATE: EuroConversionRate = |currency| match currency {
    Currency::EUR => 1.0,
    Currency::GBP => 1.17,
    Currency::USD => 0.9,
    Currency::AUD => 0.58,
    Currency::CAD => 0.67,
};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct Price {
    pub currency: Currency,
    pub amount: f32,
}

impl Price {
    pub fn new(currency: Currency, amount: f32) -> Self {
        Self { currency, amount }
    }

    pub fn def_amount_in_euros(&self) -> f32 {
        DEFAULT_CONVERSION_RATE(self.currency) * self.amount
    }

    pub fn amount_in_euros(&self, conversion_rate: EuroConversionRate) -> f32 {
        conversion_rate(self.currency) * self.amount
    }
}
