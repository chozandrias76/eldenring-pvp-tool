use super::cfg_command::feature::Feature;
use super::cfg_command::CfgCommand;
use super::radial_menu::RadialMenu;
use super::Settings;
use hudhook::tracing::metadata::LevelFilter;
use super::indicator::Indicator;
use super::level_filter_serde::LevelFilterSerde;
use libeldenring::prelude::*;
use practice_tool_core::controller::ControllerCombination;
use practice_tool_core::widgets::Widget;
use serde::Deserialize;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) settings: Settings,
    #[serde(rename = "radial-menu")]
    pub(crate) radial_menu: Vec<RadialMenu>,
    commands: Vec<CfgCommand>,
}

impl Config {
    pub(crate) fn parse(cfg: &str) -> Result<Self, String> {
        let de = &mut toml::de::Deserializer::new(cfg);
        serde_path_to_error::deserialize(de)
            .map_err(|e| format!("TOML config error at {}: {}", e.path(), e.inner()))
    }

    pub(crate) fn make_commands(
        self,
        chains: &Pointers,
        features: &Vec<Feature>,
    ) -> Vec<Box<dyn Widget>> {
        self.commands
            .into_iter()
            .filter_map(|c| c.into_widget(&self.settings, chains, &features))
            .collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                log_level: LevelFilterSerde(LevelFilter::DEBUG),
                display: "0".parse().unwrap(),
                hide: "rshift+0".parse().ok(),
                dxgi_debug: false,
                show_console: false,
                indicators: Indicator::default_set(),
                features: Feature::default_set(),
                disable_update_prompt: false,
                radial_menu_open: ControllerCombination::try_from("l3+r3").ok(),
            },
            radial_menu: Vec::new(),
            commands: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_parse_ok() {
        println!(
            "{:?}",
            toml::from_str::<toml::Value>(include_str!("../../../jdsd_er_practice_tool.toml"))
        );
        println!("{:?}", Config::parse(include_str!("../../../jdsd_er_practice_tool.toml")));
    }

    #[test]
    fn test_parse_errors() {
        println!(
            "{:#?}",
            Config::parse(
                r#"commands = [ { boh = 3 } ]
                [settings]
                log_level = "DEBUG"
                "#
            )
        );
    }
}
