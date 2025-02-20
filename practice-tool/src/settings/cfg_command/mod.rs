pub mod feature;

use feature::Feature;
use hudhook::tracing::error;
use libeldenring::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use serde::Deserialize;

use super::flag_spec::FlagSpec;
use super::multi_flag_spec::MultiFlagSpec;
use super::Settings;
use crate::widgets::action_freeze::action_freeze;
use crate::widgets::character_stats::character_stats_edit;
use crate::widgets::cycle_color::cycle_color;
use crate::widgets::cycle_speed::cycle_speed;
use crate::widgets::deathcam::deathcam;
use crate::widgets::flag::flag_widget;
use crate::widgets::group::group;
use crate::widgets::item_spawn::ItemSpawner;
use crate::widgets::label::label_widget;
use crate::widgets::multiflag::multi_flag;
use crate::widgets::none::NoneWidget;
use crate::widgets::nudge_pos::nudge_position;
use crate::widgets::position::save_position;
use crate::widgets::quitout::quitout;
use crate::widgets::runes::runes;
use crate::widgets::savefile_manager::savefile_manager;
use crate::widgets::target::Target;
use crate::widgets::warp::Warp;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
#[serde(untagged)]
pub enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: PlaceholderOption<Key>,
        feature: Feature,
    },
    ItemSpawner {
        #[serde(rename = "item_spawner")]
        hotkey_load: PlaceholderOption<Key>,
        feature: Feature,
    },
    Flag {
        flag: FlagSpec,
        hotkey: Option<Key>,
        feature: Feature,
    },
    MultiFlag {
        flag: MultiFlagSpec,
        hotkey: Option<Key>,
        feature: Feature,
    },
    SpecialFlag {
        flag: String,
        hotkey: Option<Key>,
        feature: Feature,
    },
    MultiFlagUser {
        flags: Vec<FlagSpec>,
        hotkey: Option<Key>,
        label: String,
        feature: Feature,
    },
    Label {
        #[serde(rename = "label")]
        label: String,
        feature: Feature,
    },
    Position {
        position: PlaceholderOption<Key>,
        save: Option<Key>,
        feature: Feature,
    },
    NudgePosition {
        nudge: f32,
        nudge_up: Option<Key>,
        nudge_down: Option<Key>,
        feature: Feature,
    },
    CycleSpeed {
        #[serde(rename = "cycle_speed")]
        cycle_speed: Vec<f32>,
        hotkey: Option<Key>,
        feature: Feature,
    },
    CycleColor {
        #[serde(rename = "cycle_color")]
        cycle_color: Vec<i32>,
        hotkey: Option<Key>,
        feature: Feature,
    },
    CharacterStats {
        #[serde(rename = "character_stats")]
        hotkey_open: PlaceholderOption<Key>,
        feature: Feature,
    },
    Runes {
        #[serde(rename = "runes")]
        amount: u32,
        hotkey: Option<Key>,
        feature: Feature,
    },
    Target {
        #[serde(rename = "target")]
        hotkey: PlaceholderOption<Key>,
        feature: Feature,
    },
    Warp {
        #[serde(rename = "warp")]
        _warp: bool,
        feature: Feature,
    },
    Group {
        #[serde(rename = "group")]
        label: String,
        commands: Vec<CfgCommand>,
        feature: Feature,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: PlaceholderOption<Key>,
    },
}

impl CfgCommand {
    pub fn into_widget(self, settings: &Settings, chains: &Pointers) -> Option<Box<dyn Widget>> {
        let mut exit = false;
        let widget = match self {
            CfgCommand::Flag { flag, hotkey, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                flag_widget(&flag.label, (flag.getter)(chains).clone(), hotkey)
            },
            CfgCommand::MultiFlag { flag, hotkey, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                multi_flag(
                    &flag.label,
                    flag.items.iter().map(|flag| flag(chains).clone()).collect(),
                    hotkey,
                )
            },
            CfgCommand::MultiFlagUser { flags, hotkey, label, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                multi_flag(
                    label.as_str(),
                    flags.iter().map(|flag| (flag.getter)(chains).clone()).collect(),
                    hotkey,
                )
            },
            CfgCommand::SpecialFlag { flag: special_flag, hotkey, feature }
                if special_flag == "deathcam" =>
            {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                deathcam(
                    chains.deathcam.0.clone(),
                    chains.deathcam.1.clone(),
                    chains.deathcam.2.clone(),
                    hotkey,
                )
            },
            CfgCommand::SpecialFlag { flag: special_flag, hotkey, feature }
                if special_flag == "action_freeze" =>
            {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                action_freeze(
                    chains.func_dbg_action_force.clone(),
                    chains.func_dbg_action_force_state_values,
                    hotkey,
                )
            },
            CfgCommand::SpecialFlag { flag: special_flag, hotkey: _, feature: _ } => {
                error!("Invalid flag {}", special_flag);
                return None;
            },
            CfgCommand::Label { label, feature: _ } => label_widget(label.as_str()),
            CfgCommand::SavefileManager { hotkey_load, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                savefile_manager(hotkey_load.into_option(), settings.display)
            },
            CfgCommand::ItemSpawner { hotkey_load, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                Box::new(ItemSpawner::new(
                    chains.func_item_inject,
                    chains.base_addresses.map_item_man,
                    chains.gravity.clone(),
                    hotkey_load.into_option(),
                    settings.display,
                ))
            },
            CfgCommand::Position { position, save, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                save_position(
                    chains.global_position.clone(),
                    chains.chunk_position.clone(),
                    chains.torrent_chunk_position.clone(),
                    position.into_option(),
                    save,
                )
            },
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}));
                }
                nudge_position(
                    chains.global_position.clone(),
                    chains.chunk_position.clone(),
                    chains.torrent_chunk_position.clone(),
                    nudge,
                    nudge_up,
                    nudge_down,
                )
            },
            CfgCommand::CycleSpeed { cycle_speed: values, hotkey , feature} => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }

                cycle_speed(
                    values.as_slice(),
                    [chains.animation_speed.clone(), chains.torrent_animation_speed.clone()],
                    hotkey,
                )
            },
            CfgCommand::CycleColor { cycle_color: values, hotkey , feature} => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }
                cycle_color(values.as_slice(), chains.mesh_color.clone(), hotkey)
            },
            CfgCommand::CharacterStats { hotkey_open , feature} => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }
                character_stats_edit(
                    chains.character_stats.clone(),
                    chains.character_points.clone(),
                    chains.character_blessings.clone(),
                    hotkey_open.into_option(),
                    settings.display,
                )
            },
            CfgCommand::Runes { amount, hotkey , feature} => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }
                runes(amount, chains.runes.clone(), hotkey)
            },
            CfgCommand::Warp { feature,.. } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }
                Box::new(Warp::new(
                    chains.func_warp,
                    chains.warp1.clone(),
                    chains.warp2.clone(),
                    settings.display,
                ))
            },
            CfgCommand::Target { hotkey, feature } => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }
                Box::new(Target::new(
                    chains.current_target.clone(),
                    chains.chunk_position.clone(),
                    hotkey.into_option(),
                ))
            },
            CfgCommand::Quitout { hotkey } => quitout(chains.quitout.clone(), hotkey.into_option()),
            CfgCommand::Group { label, commands , feature} => {
                if !feature.visible {
                    return Some(Box::new(NoneWidget {}))
                }
                group(
                    label.as_str(),
                    commands.into_iter().filter_map(|c| c.into_widget(settings, chains)).collect(),
                    settings.display,
                )
            },
        };

        Some(widget)
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PlaceholderOption<T> {
    Data(T),
    #[allow(dead_code)]
    Placeholder(bool),
}

impl<T> PlaceholderOption<T> {
    fn into_option(self) -> Option<T> {
        match self {
            PlaceholderOption::Data(d) => Some(d),
            PlaceholderOption::Placeholder(_) => None,
        }
    }
}
