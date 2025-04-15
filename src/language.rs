use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

// ISO 639-1
#[derive(Serialize, Deserialize, Copy, Clone, Display, EnumString, Eq, PartialEq, Debug, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    DE,
    EN,
    FR,
    ES
}

pub type I18nString = HashMap<Language, String>;