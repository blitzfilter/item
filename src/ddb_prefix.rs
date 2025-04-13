use crate::item_state::ItemState;
use serde::Deserialize;

#[macro_export]
macro_rules! make_opt_prefix_fns {
    (
        ser = $ser_fn:ident,
        de = $de_fn:ident,
        ty = $ty:ty,
        prefix = $prefix:expr
    ) => {
        // Option<T>
        pub fn $ser_fn<S>(val: &Option<$ty>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            match val {
                Some(v) => {
                    let s = format!("{}{}", $prefix, v); // uses Display
                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        pub fn $de_fn<'de, D>(deserializer: D) -> Result<Option<$ty>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Option<$ty>;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(concat!(
                        "optional string with prefix '",
                        $prefix,
                        "' and valid enum variant"
                    ))
                }

                fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let s: &str = Deserialize::deserialize(deserializer)?;
                    let stripped = s.strip_prefix($prefix).ok_or_else(|| {
                        serde::de::Error::custom(concat!("missing prefix '", $prefix, "'"))
                    })?;
                    stripped.parse::<$ty>().map(Some).map_err(|_| {
                        serde::de::Error::custom(concat!("failed to parse enum from string"))
                    })
                }

                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(None)
                }
            }

            deserializer.deserialize_option(Visitor)
        }
    };
}

#[macro_export]
macro_rules! make_prefix_fns {
    (
        ser = $ser_fn:ident,
        de = $de_fn:ident,
        ty = $ty:ty,
        prefix = $prefix:expr
    ) => {
        // Plain T
        pub fn $ser_fn<S>(val: &$ty, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let s = format!("{}{}", $prefix, val); // uses Display
            serializer.serialize_str(&s)
        }

        pub fn $de_fn<'de, D>(deserializer: D) -> Result<$ty, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = $ty;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(concat!(
                        "string with prefix '",
                        $prefix,
                        "' and valid enum variant"
                    ))
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    let stripped = value
                        .strip_prefix($prefix)
                        .ok_or_else(|| E::custom(concat!("missing prefix '", $prefix, "'")))?;
                    stripped
                        .parse::<$ty>()
                        .map_err(|_| E::custom("failed to parse enum from string"))
                }
            }

            deserializer.deserialize_str(Visitor)
        }
    };
}

// region macro_gen

make_prefix_fns!(
    ser = ser_string_item_prefix,
    de = de_string_item_prefix,
    ty = String,
    prefix = "item#"
);

make_opt_prefix_fns!(
    ser = ser_opt_string_item_prefix,
    de = de_opt_string_item_prefix,
    ty = String,
    prefix = "item#"
);

make_opt_prefix_fns!(
    ser = ser_opt_string_source_prefix,
    de = de_opt_string_source_prefix,
    ty = String,
    prefix = "source#"
);

make_opt_prefix_fns!(
    ser = ser_opt_item_state_item_prefix,
    de = de_opt_item_state_item_prefix,
    ty = ItemState,
    prefix = "item#"
);

// endregion
