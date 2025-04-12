use crate::item_hash::{ItemHash, hash_item_details};
use crate::item_state::ItemState;
use crate::price::Price;
use serde::{Deserialize, Serialize};

// TODO: (de)serialized Fields names for API!
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ItemData {
    // item#sourceId#itemId
    pub item_id: String,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    // item#sourceId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_state: Option<ItemState>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl ItemData {
    pub fn new(item_id: String) -> Self {
        ItemData {
            item_id,
            party_id: None,
            created: None,
            item_state: None,
            price: None,
            category: None,
            name: None,
            description: None,
            url: None,
            image_url: None,
        }
    }

    // region fluent-setter

    pub fn party_id(&mut self, source_id: String) -> &mut Self {
        self.party_id = Some(source_id);
        self
    }

    pub fn created(&mut self, created: String) -> &mut Self {
        self.created = Some(created);
        self
    }

    pub fn item_state(&mut self, item_state: ItemState) -> &mut Self {
        self.item_state = Some(item_state);
        self
    }

    pub fn price(&mut self, price: Price) -> &mut Self {
        self.price = Some(price);
        self
    }

    pub fn category(&mut self, category: String) -> &mut Self {
        self.category = Some(category);
        self
    }

    pub fn name(&mut self, name_en: String) -> &mut Self {
        self.name = Some(name_en);
        self
    }

    pub fn description(&mut self, description_en: String) -> &mut Self {
        self.description = Some(description_en);
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

impl ItemHash for ItemData {
    fn hash(&self) -> String {
        hash_item_details(
            self.item_state,
            self.price.map(|price| price.def_amount_in_euros()),
        )
    }
}
