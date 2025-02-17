pub mod feature;
pub mod group;

use hudhook::tracing::error;
use libeldenring::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::Widget;
use serde::Deserialize;

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

use feature::Feature;
use super::flag_spec::FlagSpec;
use super::multi_flag_spec::MultiFlagSpec;
use super::Settings;

#[cfg_attr(test, derive(Debug))]
#[derive(Deserialize)]
#[serde(untagged)]
pub enum CfgCommand {
    SavefileManager {
        #[serde(rename = "savefile_manager")]
        hotkey_load: PlaceholderOption<Key>,
    },
    ItemSpawner {
        #[serde(rename = "item_spawner")]
        hotkey_load: PlaceholderOption<Key>,
    },
    Flag {
        flag: FlagSpec,
        hotkey: Option<Key>,
    },
    MultiFlag {
        flag: MultiFlagSpec,
        hotkey: Option<Key>,
    },
    SpecialFlag {
        flag: String,
        hotkey: Option<Key>,
    },
    MultiFlagUser {
        flags: Vec<FlagSpec>,
        hotkey: Option<Key>,
        label: String,
    },
    Label {
        #[serde(rename = "label")]
        label: String,
    },
    Position {
        position: PlaceholderOption<Key>,
        save: Option<Key>,
    },
    NudgePosition {
        nudge: f32,
        nudge_up: Option<Key>,
        nudge_down: Option<Key>,
    },
    CycleSpeed {
        #[serde(rename = "cycle_speed")]
        cycle_speed: Vec<f32>,
        hotkey: Option<Key>,
    },
    CycleColor {
        #[serde(rename = "cycle_color")]
        cycle_color: Vec<i32>,
        hotkey: Option<Key>,
    },
    CharacterStats {
        #[serde(rename = "character_stats")]
        hotkey_open: PlaceholderOption<Key>,
    },
    Runes {
        #[serde(rename = "runes")]
        amount: u32,
        hotkey: Option<Key>,
    },
    Target {
        #[serde(rename = "target")]
        hotkey: PlaceholderOption<Key>,
    },
    Warp {
        #[serde(rename = "warp")]
        _warp: bool,
    },
    Group {
        #[serde(rename = "group")]
        label: String,
        commands: Vec<CfgCommand>,
    },
    Quitout {
        #[serde(rename = "quitout")]
        hotkey: PlaceholderOption<Key>,
    },
}

impl CfgCommand {
    pub fn into_widget(
        self,
        settings: &Settings,
        chains: &Pointers,
    ) -> Option<Box<dyn Widget>> {
        let mut exit = false;
        let widget = match self {
            CfgCommand::Flag { flag, hotkey } => {
                &settings.features.iter().filter(|f| !f.visible).for_each(|f| {
                    if !exit {
                        if let Some(fl) = f.flag.as_ref() {
                            if let Ok(flag_spec) = FlagSpec::try_from(fl.clone()) {
                                exit = flag_spec.label == flag.label;
                            }
                        }
                    }
                });

                if exit {
                    return Some(Box::new(NoneWidget {}));
                }
                flag_widget(&flag.label, (flag.getter)(chains).clone(), hotkey)
            },
            CfgCommand::MultiFlag { flag, hotkey } => {
                multi_flag(
                    &flag.label,
                    flag.items.iter().map(|flag| flag(chains).clone()).collect(),
                    hotkey,
                )
            },
            CfgCommand::MultiFlagUser { flags, hotkey, label } => {
                settings.features.iter().filter(|f| !f.visible).for_each(|f| {
                    if !exit {
                        if let Some(fls) = f.flags.as_ref() {
                            fls.iter().for_each(|flag| {
                                exit = flags.iter().any(|f| f.label == *flag);
                            });
                        }
                    }
                });
                if exit {
                    return Some(Box::new(NoneWidget {}));
                }
                multi_flag(
                label.as_str(),
                flags.iter().map(|flag| (flag.getter)(chains).clone()).collect(),
                hotkey,
                )
            },
            CfgCommand::SpecialFlag { flag, hotkey } if flag == "deathcam" => deathcam(
                chains.deathcam.0.clone(),
                chains.deathcam.1.clone(),
                chains.deathcam.2.clone(),
                hotkey,
            ),
            CfgCommand::SpecialFlag { flag, hotkey } if flag == "action_freeze" => action_freeze(
                chains.func_dbg_action_force.clone(),
                chains.func_dbg_action_force_state_values,
                hotkey,
            ),
            CfgCommand::SpecialFlag { flag, hotkey: _ } => {
                error!("Invalid flag {}", flag);
                return None;
            },
            CfgCommand::Label { label } => label_widget(label.as_str()),
            CfgCommand::SavefileManager { hotkey_load } => {
                savefile_manager(hotkey_load.into_option(), settings.display)
            },
            CfgCommand::ItemSpawner { hotkey_load } => Box::new(ItemSpawner::new(
                chains.func_item_inject,
                chains.base_addresses.map_item_man,
                chains.gravity.clone(),
                hotkey_load.into_option(),
                settings.display,
            )),
            CfgCommand::Position { position, save } => {
                save_position(
                chains.global_position.clone(),
                chains.chunk_position.clone(),
                chains.torrent_chunk_position.clone(),
                position.into_option(),
                save,
                )
            },
            CfgCommand::NudgePosition { nudge, nudge_up, nudge_down } => nudge_position(
                chains.global_position.clone(),
                chains.chunk_position.clone(),
                chains.torrent_chunk_position.clone(),
                nudge,
                nudge_up,
                nudge_down,
            ),
            CfgCommand::CycleSpeed { cycle_speed: values, hotkey } => cycle_speed(
                values.as_slice(),
                [chains.animation_speed.clone(), chains.torrent_animation_speed.clone()],
                hotkey,
            ),
            CfgCommand::CycleColor { cycle_color: values, hotkey } => {
                cycle_color(values.as_slice(), chains.mesh_color.clone(), hotkey)
            },
            CfgCommand::CharacterStats { hotkey_open } => character_stats_edit(
                chains.character_stats.clone(),
                chains.character_points.clone(),
                chains.character_blessings.clone(),
                hotkey_open.into_option(),
                settings.display,
            ),
            CfgCommand::Runes { amount, hotkey } => runes(amount, chains.runes.clone(), hotkey),
            CfgCommand::Warp { .. } => Box::new(Warp::new(
                chains.func_warp,
                chains.warp1.clone(),
                chains.warp2.clone(),
                settings.display,
            )),
            CfgCommand::Target { hotkey } => Box::new(Target::new(
                chains.current_target.clone(),
                chains.chunk_position.clone(),
                hotkey.into_option(),
            )),
            CfgCommand::Quitout { hotkey } => quitout(chains.quitout.clone(), hotkey.into_option()),
            CfgCommand::Group { label, commands } => group(
                label.as_str(),
                commands
                    .into_iter()
                    .filter_map(|c| c.into_widget(settings, chains))
                    .collect(),
                settings.display,
            ),
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