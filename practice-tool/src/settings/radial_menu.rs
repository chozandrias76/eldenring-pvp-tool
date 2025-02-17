use practice_tool_core::key::Key;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RadialMenu {
    pub key: Key,
    pub label: String,
}