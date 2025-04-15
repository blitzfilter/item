use std::collections::HashMap;
use crate::item_hash::{ItemHash, hash_item_details};
use crate::item_model::ItemModel;
use crate::item_state::ItemState;
use crate::price::Price;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use crate::language::{I18nString, Language};
use crate::language::Language::{DE, EN};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ItemData {
    #[serde(rename = "itemId")]
    pub item_id: String,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceId")]
    pub source_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ItemState>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub name: I18nString,

    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub description: I18nString,

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
            name: HashMap::new(),
            description: HashMap::new(),
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

    pub fn name(&mut self, name: I18nString) -> &mut Self {
        self.name = name;
        self
    }

    pub fn name_lang(&mut self, name: String, lang: Language) -> &mut Self {
        self.name.insert(lang, name);
        self
    }

    pub fn description(&mut self, description: I18nString) -> &mut Self {
        self.description = description;
        self
    }

    pub fn description_lang(&mut self, description: String, lang: Language) -> &mut Self {
        self.description.insert(lang, description);
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

impl Into<ItemModel> for ItemData {
    fn into(self) -> ItemModel {
        let created = self.created.or(OffsetDateTime::now_local()
            .ok()
            .map(|now| now.to_offset(time::UtcOffset::UTC).format(&Rfc3339).ok())
            .flatten());
        ItemModel {
            item_id: self.item_id.clone(),
            created: created.clone(),
            party_id: self.source_id.clone(),
            event_id: Some(format!(
                "{}#{}#{}",
                self.source_id.unwrap(),
                self.item_id,
                created.unwrap()
            )),
            state: self.state,
            price: self.price.map(|price| price.def_amount_in_euros()),
            category: self.category,
            name_en: self.name.get(&EN).map(|x| x.to_string()),
            description_en: self.description.get(&EN).map(|x| x.to_string()),
            name_de: self.name.get(&DE).map(|x| x.to_string()),
            description_de: self.description.get(&DE).map(|x| x.to_string()),
            url: self.url,
            image_url: self.image_url,
        }
    }
}
