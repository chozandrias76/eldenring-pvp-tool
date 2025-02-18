use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "IndicatorConfig")]
pub(crate) struct Indicator {
    pub(crate) indicator: IndicatorType,
    pub(crate) default: bool,
    pub(crate) visible: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum IndicatorType {
    Igt,
    Position,
    PositionChange,
    GameVersion,
    ImguiDebug,
    Fps,
    FrameCount,
    Animation,
}

impl Indicator {
    pub fn default_set() -> Vec<Indicator> {
        vec![
            Indicator { indicator: IndicatorType::GameVersion, default: true, visible: false },
            Indicator { indicator: IndicatorType::Igt, default: true, visible: false },
            Indicator { indicator: IndicatorType::Position, default: false, visible: false },
            Indicator { indicator: IndicatorType::PositionChange, default: false, visible: false },
            Indicator { indicator: IndicatorType::Animation, default: false, visible: false },
            Indicator { indicator: IndicatorType::Fps, default: false, visible: false },
            Indicator { indicator: IndicatorType::FrameCount, default: false, visible: true },
            Indicator { indicator: IndicatorType::ImguiDebug, default: false, visible: false },
        ]
    }
}

#[derive(Debug, Deserialize, Clone)]
struct IndicatorConfig {
    indicator: String,
    default: bool,
    visible: bool,
}

impl TryFrom<IndicatorConfig> for Indicator {
    type Error = String;

    fn try_from(indicator: IndicatorConfig) -> Result<Self, Self::Error> {
        match indicator.indicator.as_str() {
            "igt" => Ok(Indicator {
                indicator: IndicatorType::Igt,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "position" => Ok(Indicator {
                indicator: IndicatorType::Position,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "position_change" => Ok(Indicator {
                indicator: IndicatorType::PositionChange,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "animation" => Ok(Indicator {
                indicator: IndicatorType::Animation,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "game_version" => Ok(Indicator {
                indicator: IndicatorType::GameVersion,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "fps" => Ok(Indicator {
                indicator: IndicatorType::Fps,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "framecount" => Ok(Indicator {
                indicator: IndicatorType::FrameCount,
                default: indicator.default,
                visible: indicator.visible,
            }),
            "imgui_debug" => Ok(Indicator {
                indicator: IndicatorType::ImguiDebug,
                default: indicator.default,
                visible: indicator.visible,
            }),
            value => Err(format!("Unrecognized indicator: {value}")),
        }
    }
}
