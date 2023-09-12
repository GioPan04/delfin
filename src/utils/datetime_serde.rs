use serde::{
    de::{self, Visitor},
    Deserializer,
};
use speedate::{DateTime, TimeConfigBuilder};

struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an ISO 8601 formatted timestamp")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Jellyfin sends a lot of decimal places, truncate instead of erroring
        let config = TimeConfigBuilder::new()
            .microseconds_precision_overflow_behavior(
                speedate::MicrosecondsPrecisionOverflowBehavior::Truncate,
            )
            .build();

        match speedate::DateTime::parse_bytes_with_config(v.as_bytes(), &config) {
            Ok(datetime) => Ok(datetime),
            Err(err) => Err(de::Error::custom(err)),
        }
    }
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DateTimeVisitor)
}

pub fn deserialize_datetime_opt<'de, D>(deserializer: D) -> Result<Option<DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DateTimeVisitor).map(Some)
}
