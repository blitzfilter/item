use crate::item_hash::{ItemHash, hash_item_details};
use crate::item_state::ItemState;
use crate::price::Price;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ItemData {

    #[serde(rename = "itemId")]
    pub item_id: String,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    #[serde(skip_serializing_if = "Option::is_none",)]
    pub created: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceId")]
    pub source_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ItemState>,

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

    #[serde(skip_serializing_if = "Option::is_none", rename = "imageUrl")]
    pub image_url: Option<String>,
}

impl ItemData {
    pub fn new(item_id: String) -> Self {
        ItemData {
            item_id,
            source_id: None,
            created: None,
            state: None,
            price: None,
            category: None,
            name: None,
            description: None,
            url: None,
            image_url: None,
        }
    }

    // region fluent-setter

    pub fn source_id(&mut self, source_id: String) -> &mut Self {
        self.source_id = Some(source_id);
        self
    }

    pub fn created(&mut self, created: String) -> &mut Self {
        self.created = Some(created);
        self
    }

    pub fn state(&mut self, state: ItemState) -> &mut Self {
        self.state = Some(state);
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
            self.state,
            self.price.map(|price| price.def_amount_in_euros()),
        )
    }
}
