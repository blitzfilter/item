use crate::item_state::ItemState;
use serde::{Deserialize, Serialize};

pub trait ItemHash {
    fn hash(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct ItemEventHash {
    // sourceId
    #[serde(
        rename = "party_id",
        serialize_with = "crate::ddb_prefix::ser_string_source_prefix",
        deserialize_with = "crate::ddb_prefix::de_string_source_prefix"
    )]
    pub source_id: String,

    // sourceId#itemId#created
    #[serde(
        serialize_with = "crate::ddb_prefix::ser_string_item_prefix",
        deserialize_with = "crate::ddb_prefix::de_string_item_prefix"
    )]
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
                if count == 2 {
                    return &s[..i];
                }
            }
        }
        s
    }
}

impl ItemHash for ItemEventHash {
    fn hash(&self) -> String {
        self.hash.clone()
    }
}

pub fn hash_item_details(item_state: Option<ItemState>, eur_price: Option<f32>) -> String {
    blake3::hash(
        format!(
            "{}|{}",
            item_state.map(|x| x.to_string()).unwrap_or(String::new()),
            eur_price.map(|x| x.to_string()).unwrap_or(String::new()),
        )
        .as_bytes(),
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::item_hash::ItemEventHash;

    #[test]
    fn should_return_item_id_for_get_item_id() {
        let item_event_hash = ItemEventHash {
            source_id: "foo".to_string(),
            event_id: "foo#bar#2025-01-01T12:00:00.001+01:00".to_string(),
            hash: "123465".to_string(),
        };

        let expected = "foo#bar";
        let actual = item_event_hash.get_item_id();

        assert_eq!(expected, actual);
    }
}
