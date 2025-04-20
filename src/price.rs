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
    NZD,
}

pub type EuroConversionRate = fn(Currency) -> f32;

pub const DEFAULT_CONVERSION_RATE: EuroConversionRate = |currency| match currency {
    Currency::EUR => 1.0,
    Currency::GBP => 1.17,
    Currency::USD => 0.9,
    Currency::AUD => 0.58,
    Currency::CAD => 0.67,
    Currency::NZD => 0.53,
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

#[cfg(test)]
mod tests {
    use crate::price::Currency;
    use rstest::rstest;

    #[rstest]
    #[case(Currency::EUR, "\"EUR\"")]
    #[case(Currency::GBP, "\"GBP\"")]
    #[case(Currency::USD, "\"USD\"")]
    #[case(Currency::AUD, "\"AUD\"")]
    #[case(Currency::CAD, "\"CAD\"")]
    #[case(Currency::NZD, "\"NZD\"")]
    fn should_serialize_currency_according_to_iso_4217(
        #[case] currency: Currency,
        #[case] expected: &str,
    ) {
        let actual = serde_json::to_string(&currency).unwrap();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("\"EUR\"", Currency::EUR)]
    #[case("\"GBP\"", Currency::GBP)]
    #[case("\"USD\"", Currency::USD)]
    #[case("\"AUD\"", Currency::AUD)]
    #[case("\"CAD\"", Currency::CAD)]
    #[case("\"NZD\"", Currency::NZD)]
    fn should_deserialize_currency_according_to_iso_4217(
        #[case] currency: &str,
        #[case] expected: Currency,
    ) {
        let actual = serde_json::from_str::<Currency>(&currency).unwrap();
        assert_eq!(actual, expected);
    }
}
