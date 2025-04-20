use crate::item_hash::{ItemHash, hash_item_details};
use crate::item_model::ItemModel;
use crate::item_state::ItemState;
use crate::language::Language::{DE, EN};
use crate::language::{I18nString, Language};
use crate::price::Price;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ItemData {
    #[serde(rename = "itemId")]
    pub item_id: String,

    // ISO 8601: 2010-01-01T12:00:00.001+01:00
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub created: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceId", default)]
    pub source_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<ItemState>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub price: Option<Price>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty", default)]
    pub name: I18nString,

    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty", default)]
    pub description: I18nString,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "imageUrl", default)]
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
            source_id: self.source_id,
            event_id: Some(format!("{}#{}", self.item_id, created.unwrap())),
            state: self.state,
            price: self.price.map(|price| price.def_amount_in_euros()),
            category: self.category,
            name_en: self.name.get(&EN).map(|x| x.to_string()),
            description_en: self.description.get(&EN).map(|x| x.to_string()),
            name_de: self.name.get(&DE).map(|x| x.to_string()),
            description_de: self.description.get(&DE).map(|x| x.to_string()),
            url: self.url,
            image_url: self.image_url,
            hash: Some(hash_item_details(
                self.state,
                self.price.map(|price| price.def_amount_in_euros()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::item_data::ItemData;
    use crate::item_model::ItemModel;
    use crate::item_state::ItemState;
    use crate::language::Language::{DE, EN};
    use crate::price::Currency::EUR;
    use crate::price::Price;
    use std::collections::HashMap;

    #[test]
    fn should_convert_data_into_model() {
        let data = ItemData {
            item_id: "https://foo.bar#123456".to_string(),
            created: Some("2010-01-01T12:00:00.001+01:00".to_string()),
            source_id: Some("https://foo.bar".to_string()),
            state: Some(ItemState::AVAILABLE),
            price: Some(Price::new(EUR, 42f32)),
            category: Some("foo".to_string()),
            name: HashMap::from([(EN, "bar".to_string()), (DE, "balken".to_string())]),
            description: HashMap::from([(EN, "baz".to_string()), (DE, "basis".to_string())]),
            url: Some("https://foo.bar?item=123456".to_string()),
            image_url: Some("https://foo.bar?item_img=123456".to_string()),
        };
        let expected = ItemModel {
            item_id: "https://foo.bar#123456".to_string(),
            created: Some("2010-01-01T12:00:00.001+01:00".to_string()),
            source_id: Some("https://foo.bar".to_string()),
            event_id: Some("https://foo.bar#123456#2010-01-01T12:00:00.001+01:00".to_string()),
            state: Some(ItemState::AVAILABLE),
            price: Some(42f32),
            category: Some("foo".to_string()),
            name_en: Some("bar".to_string()),
            description_en: Some("baz".to_string()),
            name_de: Some("balken".to_string()),
            description_de: Some("basis".to_string()),
            url: Some("https://foo.bar?item=123456".to_string()),
            image_url: Some("https://foo.bar?item_img=123456".to_string()),
            hash: Some(
                "1d10a63438fff3ccd4877c2195c0a377a6ee0c8caad97e652b1e69c68b45557b".to_string(),
            ),
        };

        let actual: ItemModel = data.into();

        assert_eq!(actual, expected)
    }
}
