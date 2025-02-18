use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(try_from = "FeatureConfig")]
pub(crate) struct Feature {
    pub(crate) flag: Option<String>,
    pub(crate) flags: Option<Vec<String>>,
    pub(crate) group: Option<String>,
    pub(crate) default: bool,
    pub(crate) visible: bool,
}

impl Default for Feature {
    fn default() -> Self {
        Feature {
            flag: None,
            flags: None,
            group: None,
            default: false,
            visible: false,
        }
    }
}

impl Feature {
    pub fn default_set() -> Vec<Feature> {
        vec![
            Feature {
                flags: Some(vec!["show_all_graces".to_string(), "show_all_map_layers".to_string()]),
                ..Default::default()
            },
            Feature {
                flag: Some("no_damage".to_string()),
                ..Default::default()
            },
            Feature {
                flag: Some("no_stamina_consume".to_string()),
                ..Default::default()
            },
            Feature {
                flags: Some(vec![
                    "no_fp_consume".to_string(),
                    "no_ashes_of_war_fp_consume".to_string(),
                ]),
                ..Default::default()
            },
            Feature {
                flags: Some(vec!["no_goods_consume".to_string(), "no_arrows_consume".to_string()]),
                default: true,
                visible: true,
                ..Feature::default()
            },
            Feature {
                flag: Some("deathcam".to_string()),
                visible: false,
                ..Feature::default()
            },
            Feature {
                flag: Some("no_dead".to_string()),
                ..Feature::default()
            },
            Feature {
                flag: Some("one_shot".to_string()),
                ..Feature::default()
            },
            Feature {
                flag: Some("runearc".to_string()),
                visible: true,
                ..Feature::default()
            },
            Feature {
                flags: Some(vec![
                    "field_area_direction".to_string(),
                    "field_area_altimeter".to_string(),
                    "field_area_compass".to_string(),
                ]),
                ..Feature::default()
            },
            Feature {
                flag: Some("no_update_ai".to_string()),
                ..Feature::default()
            },
            Feature {
                flag: Some("no_trigger_event".to_string()),
                ..Feature::default()
            },
            Feature {
                flag: Some("gravity".to_string()),
                ..Feature::default()
            },
            Feature {
                flag: Some("torrent_gravity".to_string()),
                ..Feature::default()
            },
            Feature {
                flags: Some(vec!["collision".to_string(), "torrent_collision".to_string()]),
                ..Feature::default()
            },
            Feature {
                flag: Some("action_freeze".to_string()),
                ..Feature::default()
            },
            Feature {
                group: Some("Render flags".to_string()),
                ..Feature::default()
            },
            Feature {
                group: Some("Position storage".to_string()),
                visible: false,
                ..Feature::default()
            },
        ]
    }
}

#[derive(Debug, Deserialize, Clone)]
struct FeatureConfig {
    flags: Option<Vec<String>>,
    flag: Option<String>,
    group: Option<String>,
    default: bool,
    visible: bool,
}

impl TryFrom<FeatureConfig> for Feature {
    type Error = String;

    fn try_from(feature: FeatureConfig) -> Result<Self, Self::Error> {
        Ok(Feature {
            flag: feature.flag,
            flags: feature.flags,
            default: feature.default,
            visible: feature.visible,
            group: feature.group,
        })
    }
}
