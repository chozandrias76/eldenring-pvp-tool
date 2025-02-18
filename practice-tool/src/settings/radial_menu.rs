use practice_tool_core::key::Key;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RadialMenu {
    pub key: Key,
    pub label: String,
}

impl Default for RadialMenu {
    fn default() -> Self {
        Self {
            key: Key::try_from("l3+r3").unwrap(),
            label: "Radial Menu".to_string(),
        }
    }
}