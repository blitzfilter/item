use crate::currency::Currency;
use crate::itemstate::ItemState;
use crate::itemtype::ItemType;
use serde::{Deserialize, Serialize};

// DynamoDB Relations#PT
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

impl ItemDiff {
    pub fn new(item_id: String) -> Self {
        ItemDiff {
            item_id,
            source_id: None,
            event_id: None,
            created: None,
            item_type: None,
            item_state: None,
            category: None,
            name_en: None,
            description_en: None,
            name_de: None,
            description_de: None,
            lower_year: None,
            upper_year: None,
            currency: None,
            lower_price: None,
            upper_price: None,
            url: None,
            image_url: None,
        }
    }

    // region fluent-setter

    pub fn source_id(&mut self, source_id: String) -> &mut Self {
        self.source_id = Some(source_id);
        self
    }

    pub fn event_id(&mut self, event_id: String) -> &mut Self {
        self.event_id = Some(event_id);
        self
    }

    pub fn created(&mut self, created: String) -> &mut Self {
        self.created = Some(created);
        self
    }

    pub fn item_type(&mut self, item_type: ItemType) -> &mut Self {
        self.item_type = Some(item_type);
        self
    }
    pub fn item_state(&mut self, item_state: ItemState) -> &mut Self {
        self.item_state = Some(item_state);
        self
    }

    pub fn category(&mut self, category: String) -> &mut Self {
        self.category = Some(category);
        self
    }

    pub fn name_en(&mut self, name_en: String) -> &mut Self {
        self.name_en = Some(name_en);
        self
    }

    pub fn description_en(&mut self, description_en: String) -> &mut Self {
        self.description_en = Some(description_en);
        self
    }

    pub fn lower_year(&mut self, lower_year: i8) -> &mut Self {
        self.lower_year = Some(lower_year);
        self
    }

    pub fn upper_year(&mut self, upper_year: i8) -> &mut Self {
        self.upper_year = Some(upper_year);
        self
    }

    pub fn currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }

    pub fn lower_price(&mut self, lower_price: f32) -> &mut Self {
        self.lower_price = Some(lower_price);
        self
    }

    pub fn upper_price(&mut self, upper_price: f32) -> &mut Self {
        self.upper_price = Some(upper_price);
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn image_url(&mut self, image_url: String) -> &mut Self {
        self.image_url = Some(image_url);
        self
    }

    // endregion
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
) -> String {
    blake3::hash(
        format!(
            "{}|{}|{}|{}|{}",
            item_state.map(|x| x.to_string()).unwrap_or(String::new()),
            currency.map(|x| x.to_string()).unwrap_or(String::new()),
            lower_price.map(|x| x.to_string()).unwrap_or(String::new()),
            upper_price.map(|x| x.to_string()).unwrap_or(String::new()),
            url.map(|x| x.to_string()).unwrap_or(String::new()),
        )
        .as_bytes(),
    )
    .to_string()
}
