use serde::de;

// https://stackoverflow.com/questions/70114905/how-to-deserialize-a-string-field-to-bool
pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["true", "false"])),
    }
}

pub fn opt_bool_from_string<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;

    match s {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Ok(None),
    }
}
