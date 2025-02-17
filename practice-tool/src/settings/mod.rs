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
    #[serde(default = "Feature::default_set")]
    pub(crate) features: Vec<Feature>,
    pub(crate) radial_menu_open: Option<ControllerCombination>,
}