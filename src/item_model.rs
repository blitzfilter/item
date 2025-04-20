use crate::item_data::ItemData;
use crate::item_hash::{ItemHash, hash_item_details};
use crate::item_state::ItemState;
use crate::language::I18nString;
use crate::language::Language::{DE, EN};
use crate::price::Currency::EUR;
use crate::price::Price;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub created: Option<String>,

    #[serde(
        rename = "party_id",
        serialize_with = "crate::ddb_prefix::ser_opt_string_source_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_string_source_prefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub source_id: Option<String>,

    // sourceId#itemId#created
    #[serde(
        serialize_with = "crate::ddb_prefix::ser_opt_string_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_string_item_prefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub event_id: Option<String>,

    #[serde(
        serialize_with = "crate::ddb_prefix::ser_opt_item_state_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_opt_item_state_item_prefix",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub state: Option<ItemState>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub price: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name_en: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description_en: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name_de: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description_de: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub hash: Option<String>,
}

impl ItemModel {
    pub fn new(item_id: String) -> Self {
        ItemModel {
            item_id,
            source_id: None,
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
            hash: None,
        }
    }

    // region fluent_setter

    pub fn source_id(&mut self, source_id: String) -> &mut Self {
        self.source_id = Some(source_id);
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

impl Into<ItemData> for ItemModel {
    fn into(self) -> ItemData {
        ItemData {
            item_id: self.item_id,
            created: self.created,
            source_id: self.source_id,
            state: self.state,
            price: self.price.map(|price| Price::new(EUR, price)),
            category: self.category,
            name: {
                let mut name: I18nString = HashMap::new();
                if self.name_en.is_some() {
                    name.insert(EN, self.name_en.unwrap());
                }
                if self.name_de.is_some() {
                    name.insert(DE, self.name_de.unwrap());
                }
                name
            },
            description: {
                let mut description: I18nString = HashMap::new();
                if self.description_en.is_some() {
                    description.insert(EN, self.description_en.unwrap());
                }
                if self.description_de.is_some() {
                    description.insert(DE, self.description_de.unwrap());
                }
                description
            },
            url: self.url,
            image_url: self.image_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn should_serialize_item_id_as_pk_with_prefix_item() {
        let item = ItemModel::new("123456".to_string());
        let expected = r#""pk":"item#123456""#;

        let actual = serde_json::to_string(&item).unwrap();
        
        assert!(actual.contains(expected));
    }

    #[test]
    fn should_deserialize_item_id_as_pk_with_prefix_item() {
        let item_json = r#"{"pk":"item#123456"}"#;
        let expected = ItemModel::new("123456".to_string());
        
        let actual = serde_json::from_str::<ItemModel>(item_json).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_serialize_created_as_sk_with_prefix_item() {
        let item = ItemModel::new("123456".to_string()).created("abcdef".to_string()).to_owned();
        let expected = r#""sk":"item#abcdef""#;

        let actual = serde_json::to_string(&item).unwrap();

        assert!(actual.contains(&expected));
    }

    #[test]
    fn should_deserialize_created_as_sk_with_prefix_item() {
        let item_json = r#"{"pk":"item#123456", "sk":"item#abcdef"}"#;
        let expected = ItemModel::new("123456".to_string()).created("abcdef".to_string()).to_owned();

        let actual = serde_json::from_str::<ItemModel>(item_json).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_serialize_source_id_as_party_id_with_prefix_source() {
        let item = ItemModel::new("123456".to_string()).source_id("abcdef".to_string()).to_owned();
        let expected = r#""party_id":"source#abcdef""#;

        let actual = serde_json::to_string(&item).unwrap();

        assert!(actual.contains(&expected));
    }

    #[test]
    fn should_deserialize_source_id_as_party_id_with_prefix_source() {
        let item_json = r#"{"pk":"item#123456", "party_id":"source#abcdef"}"#;
        let expected = ItemModel::new("123456".to_string()).source_id("abcdef".to_string()).to_owned();

        let actual = serde_json::from_str::<ItemModel>(item_json).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_serialize_event_id_as_event_id_with_prefix_item() {
        let item = ItemModel::new("123456".to_string()).event_id("abcdef".to_string()).to_owned();
        let expected = r#""event_id":"item#abcdef""#;

        let actual = serde_json::to_string(&item).unwrap();

        assert!(actual.contains(&expected));
    }

    #[test]
    fn should_deserialize_event_id_as_event_id_with_prefix_item() {
        let item_json = r#"{"pk":"item#123456", "event_id":"item#abcdef"}"#;
        let expected = ItemModel::new("123456".to_string()).event_id("abcdef".to_string()).to_owned();

        let actual = serde_json::from_str::<ItemModel>(item_json).unwrap();

        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(ItemState::LISTED, "LISTED")]
    #[case(ItemState::AVAILABLE, "AVAILABLE")]
    #[case(ItemState::RESERVED, "RESERVED")]
    #[case(ItemState::SOLD, "SOLD")]
    #[case(ItemState::REMOVED, "REMOVED")]
    fn should_serialize_state_as_state_with_prefix_item(#[case] state: ItemState, #[case] expected: &str) {
        let item = ItemModel::new("123456".to_string()).state(state).to_owned();
        let expected = format!(r#""state":"item#{expected}""#);

        let actual = serde_json::to_string(&item).unwrap();

        assert!(actual.contains(&expected));
    }

    #[rstest]
    #[case("LISTED", ItemState::LISTED)]
    #[case("AVAILABLE", ItemState::AVAILABLE)]
    #[case("RESERVED", ItemState::RESERVED)]
    #[case("SOLD", ItemState::SOLD)]
    #[case("REMOVED", ItemState::REMOVED)]
    fn should_deserialize_state_as_state_with_prefix_item(#[case] state: &str, #[case] expected: ItemState) {
        let item_json = format!(r#"{{"pk":"item#123456", "state":"item#{state}"}}"#);
        let expected = ItemModel::new("123456".to_string()).state(expected).to_owned();

        let actual = serde_json::from_str::<ItemModel>(&item_json).unwrap();

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn should_convert_model_into_data() {
        let model = ItemModel {
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
            hash: Some("1d10a63438fff3ccd4877c2195c0a377a6ee0c8caad97e652b1e69c68b45557b".to_string()),
        };
        let expected = ItemData {
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
        
        let actual: ItemData = model.into();
        
        assert_eq!(actual, expected)
    }
}