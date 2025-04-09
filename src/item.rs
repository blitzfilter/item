use crate::currency::Currency;
use crate::itemstate::ItemState;
use crate::itemtype::ItemType;
use serde::{Deserialize, Serialize};

// DynamoDB Relations#PT
#[derive(Serialize, Deserialize)]
pub struct ItemDiff {
    // item#sourceId#itemId
    pub item_id: Option<String>,

    // item#sourceId
    pub source_id: Option<String>,

    // item#sourceId#itemId#created
    pub event_id: Option<String>,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    pub created: Option<String>,

    pub item_type: Option<ItemType>,
    pub item_state: Option<ItemState>,
    pub category: Option<String>,

    pub name_en: Option<String>,
    pub description_en: Option<String>,
    pub name_de: Option<String>,
    pub description_de: Option<String>,

    pub lower_year: Option<i8>,
    pub upper_year: Option<i8>,

    pub currency: Option<Currency>,
    pub lower_price: Option<f32>,
    pub upper_price: Option<f32>,

    pub url: Option<String>,
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
