mod cfg_command;
mod flag_spec;
pub mod indicator;
mod level_filter_serde;
mod multi_flag_spec;
pub mod config;
pub mod radial_menu;
use cfg_command::feature::Feature;
use practice_tool_core::controller::ControllerCombination;
use practice_tool_core::key::Key;
use serde::Deserialize;

use indicator::Indicator;
use level_filter_serde::LevelFilterSerde;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Settings {
    pub(crate) log_level: LevelFilterSerde,
    pub(crate) display: Key,
    pub(crate) hide: Option<Key>,
    #[serde(default)]
    pub(crate) dxgi_debug: bool,
    #[serde(default)]
    pub(crate) show_console: bool,
    #[serde(default)]
    pub(crate) disable_update_prompt: bool,
    #[serde(default = "Indicator::default_set")]
    pub(crate) indicators: Vec<Indicator>,
    pub(crate) radial_menu_open: Option<ControllerCombination>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            log_level: LevelFilterSerde::default(),
            display: Key::try_from("F2").unwrap(),
            hide: "rshift+f2".parse().ok(),
            dxgi_debug: false,
            show_console: false,
            disable_update_prompt: false,
            indicators: Indicator::default_set(),
            radial_menu_open: ControllerCombination::try_from("l3+r3").ok(),
        }
    }
}