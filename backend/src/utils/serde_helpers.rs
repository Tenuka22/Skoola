use serde::{Deserialize, Deserializer};

/// Deserializer for `Option<Option<T>>` to distinguish between a missing field,
/// an explicit null, and a value.
///
/// - Missing field (with `#[serde(default)]`): `None`
/// - Explicit null: `Some(None)`
/// - Explicit value: `Some(Some(T))`
pub fn deserialize_option_option<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}
