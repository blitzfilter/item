use crate::item_hash::{ItemHash, hash_item_details};
use crate::item_state::ItemState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ItemModel {
    // sourceId#itemId
    #[serde(
        rename = "pk",
        serialize_with = "crate::ddb_prefix::ser_string_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_string_item_prefix"
    )]
    pub item_id: String,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    #[serde(
        rename = "sk",
        serialize_with = "crate::ddb_prefix::ser_opt_string_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_string_item_prefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<String>,

    #[serde(
        serialize_with = "crate::ddb_prefix::ser_opt_string_source_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_string_source_prefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub party_id: Option<String>,

    // sourceId#itemId#created
    #[serde(
        serialize_with = "crate::ddb_prefix::ser_opt_string_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_string_item_prefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub event_id: Option<String>,

    #[serde(
        serialize_with = "crate::ddb_prefix::ser_opt_item_state_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_item_state_item_prefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub state: Option<ItemState>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,

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
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

impl ItemModel {
    pub fn new(item_id: String) -> Self {
        ItemModel {
            item_id,
            party_id: None,
            created: None,
            event_id: None,
            state: None,
            price: None,
            category: None,
            name_en: None,
            description_en: None,
            name_de: None,
            description_de: None,
            url: None,
            image_url: None,
        }
    }

    // region fluent_setter

    pub fn party_id(&mut self, source_id: String) -> &mut Self {
        self.party_id = Some(source_id);
        self
    }

    pub fn created(&mut self, created: String) -> &mut Self {
        self.created = Some(created);
        self
    }

    pub fn event_id(&mut self, event_id: String) -> &mut Self {
        self.event_id = Some(event_id);
        self
    }

    pub fn state(&mut self, state: ItemState) -> &mut Self {
        self.state = Some(state);
        self
    }

    pub fn price(&mut self, upper_price: f32) -> &mut Self {
        self.price = Some(upper_price);
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

    pub fn name_de(&mut self, name_en: String) -> &mut Self {
        self.name_de = Some(name_en);
        self
    }

    pub fn description_de(&mut self, description_en: String) -> &mut Self {
        self.description_de = Some(description_en);
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

impl ItemHash for ItemModel {
    fn hash(&self) -> String {
        hash_item_details(self.state, self.price)
    }
}
