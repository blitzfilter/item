use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

// ISO 639-1
#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString, EnumIter, Eq, PartialEq, Debug, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    DE,
    EN,
    FR,
    ES
}

pub type I18nString = HashMap<Language, String>;

#[cfg(test)]
mod tests {
    use crate::language::Language;
    use rstest::rstest;

    #[rstest]
    #[case(Language::DE, "\"de\"")]
    #[case(Language::EN, "\"en\"")]
    #[case(Language::FR, "\"fr\"")]
    #[case(Language::ES, "\"es\"")]
    fn should_serialize_language_according_to_iso_639(
        #[case] language: Language,
        #[case] expected: &str,
    ) {
        let actual = serde_json::to_string(&language).unwrap();
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("\"de\"", Language::DE)]
    #[case("\"en\"", Language::EN)]
    #[case("\"fr\"", Language::FR)]
    #[case("\"es\"", Language::ES)]
    fn should_deserialize_language_according_to_iso_639(
        #[case] language: &str,
        #[case] expected: Language,
    ) {
        let actual = serde_json::from_str::<Language>(language).unwrap();
        assert_eq!(actual, expected);
    }
}
