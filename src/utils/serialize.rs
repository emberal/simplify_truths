use serde::{de, Deserialize, Deserializer};

pub(crate) const fn ret_true() -> bool {
    true
}


pub(crate) fn deserialize_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["true", "false"])),
    }
}