/* use serde::de::{self, Deserialize, Deserializer, Unexpected};

pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "true" => Ok(true),
        "false" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"true or false",
        )),
    }
} */