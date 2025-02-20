use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(try_from = "FeatureConfig")]
pub(crate) struct Feature {
    pub(crate) default: bool,
    pub(crate) visible: bool,
}

impl Default for Feature {
    fn default() -> Self {
        Feature { default: false, visible: false }
    }
}

#[derive(Debug, Deserialize, Clone)]
struct FeatureConfig {
    default: bool,
    visible: bool,
}

impl TryFrom<FeatureConfig> for Feature {
    type Error = String;

    fn try_from(feature: FeatureConfig) -> Result<Self, Self::Error> {
        Ok(Feature { default: feature.default, visible: feature.visible })
    }
}
