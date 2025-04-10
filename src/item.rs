use crate::currency::Currency;
use crate::itemstate::ItemState;
use crate::itemtype::ItemType;
use serde::{Deserialize, Serialize};

// DynamoDB Relations#PT
#[derive(Serialize, Deserialize)]
pub struct ItemDiff {
    // item#sourceId#itemId
    pub item_id: String,

    // item#sourceId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,

    // item#sourceId#itemId#created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<ItemType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_state: Option<ItemState>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_en: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_en: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_de: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_de: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_year: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_year: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_price: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_price: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

// DynamoDB Relations#GSI_1: SourceId-EventId-Hash-Index
#[derive(Serialize, Deserialize)]
pub struct ItemEventHash {
    // item#sourceId
    pub source_id: String,

    // item#sourceId#itemId#created
    pub event_id: String,

    pub hash: String,
}

impl ItemEventHash {
    pub fn get_item_id(&self) -> &str {
        let s = self.event_id.as_str();
        let mut count = 0;
        for (i, b) in s.bytes().enumerate() {
            if b == b'#' {
                count += 1;
                if count == 3 {
                    return &s[..i];
                }
            }
        }
        s
    }
}

pub fn hash_item_details(
    item_state: Option<ItemState>,
    currency: Option<Currency>,
    lower_price: Option<f32>,
    upper_price: Option<f32>,
    url: Option<String>,
    image_url: Option<String>,
) -> String {
    blake3::hash(
        format!(
            "{}|{}|{}|{}|{}|{}",
            item_state.map(|x| x.to_string()).unwrap_or(String::new()),
            currency.map(|x| x.to_string()).unwrap_or(String::new()),
            lower_price.map(|x| x.to_string()).unwrap_or(String::new()),
            upper_price.map(|x| x.to_string()).unwrap_or(String::new()),
            url.map(|x| x.to_string()).unwrap_or(String::new()),
            image_url.map(|x| x.to_string()).unwrap_or(String::new())
        )
        .as_bytes(),
    )
    .to_string()
}
