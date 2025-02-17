use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(try_from = "FeatureConfig")]
pub(crate) struct Feature {
    pub(crate) flag: Option<String>,
    pub(crate) flags: Option<Vec<String>>,
    pub(crate) default: bool,
    pub(crate) visible: bool,
}

impl Feature {
    pub fn default_set() -> Vec<Feature> {
        vec![
            Feature {
                flag: Some("show_all_map_layers".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: None,
                flags: Some(vec!["show_all_graces".to_string(), "show_all_map_layers".to_string()]),
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("show_map".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("show_chr".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: Some("display_stable_pos".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("hitbox_high".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: Some("hitbox_low".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: Some("hitbox_f".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: Some("hitbox_character".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: Some("hitbox_event".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: None,
                flags: Some(vec![
                    "weapon_hitbox1".to_string(),
                    "weapon_hitbox2".to_string(),
                    "weapon_hitbox3".to_string(),
                ]),
                default: false,
                visible: true,
            },
            Feature {
                flag: Some("no_damage".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("no_stamina_consume".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: None,
                flags: Some(vec![
                    "no_fp_consume".to_string(),
                    "no_ashes_of_war_fp_consume".to_string(),
                ]),
                default: false,
                visible: false,
            },
            Feature {
                flag: None,
                flags: Some(vec!["no_goods_consume".to_string(), "no_arrows_consume".to_string()]),
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("deathcam".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("no_dead".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("one_shot".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("runearc".to_string()),
                flags: None,
                default: false,
                visible: true,
            },
            Feature {
                flag: None,
                flags: Some(vec![
                    "field_area_direction".to_string(),
                    "field_area_altimeter".to_string(),
                    "field_area_compass".to_string(),
                ]),
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("no_update_ai".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("no_trigger_event".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("gravity".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("torrent_gravity".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
            Feature {
                flag: None,
                flags: Some(vec!["collision".to_string(), "torrent_collision".to_string()]),
                default: false,
                visible: false,
            },
            Feature {
                flag: Some("action_freeze".to_string()),
                flags: None,
                default: false,
                visible: false,
            },
        ]
    }
}

#[derive(Debug, Deserialize, Clone)]
struct FeatureConfig {
    flags: Option<Vec<String>>,
    flag: Option<String>,
    default: bool,
    visible: bool,
}

impl TryFrom<FeatureConfig> for Feature {
    type Error = String;

    fn try_from(feature: FeatureConfig) -> Result<Self, Self::Error> {
        match (feature.flag, feature.flags) {
            (Some(flag), None) => Ok(Feature {
                flag: Some(flag),
                flags: None,
                default: feature.default,
                visible: feature.visible,
            }),
            (None, Some(flags)) => Ok(Feature {
                flag: None,
                flags: Some(flags),
                default: feature.default,
                visible: feature.visible,
            }),
            _ => Err("Invalid feature config".to_string()),
        }
    }
}
