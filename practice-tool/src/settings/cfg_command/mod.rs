pub mod feature;
pub mod group;

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
    pub fn into_widget(self, settings: &Settings, chains: &Pointers) -> Option<Box<dyn Widget>> {
        let mut exit = false;
        let widget = match self {
            CfgCommand::Flag { flag, hotkey } => {
                if let Some(skip_widget) = skipped_widget(
                    settings.features.clone(),
                    &mut exit,
                    &Flag::FlagSpecFlag(flag.clone()),
                ) {
                    return skip_widget;
                }
                flag_widget(&flag.label, (flag.getter)(chains).clone(), hotkey)
            },
            CfgCommand::MultiFlag { flag, hotkey } => {
                // return Some(Box::new(NoneWidget {}));
                multi_flag(
                    &flag.label,
                    flag.items.iter().map(|flag| flag(chains).clone()).collect(),
                    hotkey,
                )
            },
            CfgCommand::MultiFlagUser { flags, hotkey, label } => {
                settings.features.iter().filter(|feature| !feature.visible).for_each(|feature| {
                    if !exit {
                        if let Some(feature_flags) = feature.flags.as_ref() {
                            let feature_flags_into_names: Vec<String> = feature_flags
                                .iter()
                                .map(|flag| FlagSpec::try_from(flag.clone()).unwrap().label)
                                .collect();
                            let command_flags = flags
                                .iter()
                                .map(|flag: &FlagSpec| flag.label.clone())
                                .collect::<Vec<String>>();
                            let mut sorted_command_flags = command_flags.clone();
                            sorted_command_flags.sort();
                            let mut sorted_feature_flags = feature_flags_into_names.clone();
                            sorted_feature_flags.sort();

                            exit = sorted_command_flags == sorted_feature_flags;
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
            CfgCommand::SpecialFlag { flag: special_flag, hotkey }
                if special_flag == "deathcam" =>
            {
                if let Some(skip_widget) = skipped_widget(
                    settings.features.clone(),
                    &mut exit,
                    &Flag::StringFlag(special_flag),
                ) {
                    return skip_widget;
                }
                deathcam(
                    chains.deathcam.0.clone(),
                    chains.deathcam.1.clone(),
                    chains.deathcam.2.clone(),
                    hotkey,
                )
            },
            CfgCommand::SpecialFlag { flag: special_flag, hotkey }
                if special_flag == "action_freeze" =>
            {
                if let Some(skip_widget) = skipped_widget(
                    settings.features.clone(),
                    &mut exit,
                    &Flag::StringFlag(special_flag),
                ) {
                    return skip_widget;
                }
                action_freeze(
                    chains.func_dbg_action_force.clone(),
                    chains.func_dbg_action_force_state_values,
                    hotkey,
                )
            },
            CfgCommand::SpecialFlag { flag: special_flag, hotkey: _ } => {
                error!("Invalid flag {}", special_flag);
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
                // if let Some(skip_widget) = skipped_group_widget(settings.features.clone(),
                // exit, &label) {     return skip_widget;
                // }
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
            CfgCommand::CycleSpeed { cycle_speed: values, hotkey } => {
                return Some(Box::new(NoneWidget {}));

                cycle_speed(
                    values.as_slice(),
                    [chains.animation_speed.clone(), chains.torrent_animation_speed.clone()],
                    hotkey,
                )
            },
            CfgCommand::CycleColor { cycle_color: values, hotkey } => {
                return Some(Box::new(NoneWidget {}));
                cycle_color(values.as_slice(), chains.mesh_color.clone(), hotkey)
            },
            CfgCommand::CharacterStats { hotkey_open } => {
                return Some(Box::new(NoneWidget {}));
                character_stats_edit(
                    chains.character_stats.clone(),
                    chains.character_points.clone(),
                    chains.character_blessings.clone(),
                    hotkey_open.into_option(),
                    settings.display,
                )
            },
            CfgCommand::Runes { amount, hotkey } => {
                return Some(Box::new(NoneWidget {}));
                runes(amount, chains.runes.clone(), hotkey)
            },
            CfgCommand::Warp { .. } => {
                return Some(Box::new(NoneWidget {}));
                Box::new(Warp::new(
                    chains.func_warp,
                    chains.warp1.clone(),
                    chains.warp2.clone(),
                    settings.display,
                ))
            },
            CfgCommand::Target { hotkey } => {
                return Some(Box::new(NoneWidget {}));
                Box::new(Target::new(
                    chains.current_target.clone(),
                    chains.chunk_position.clone(),
                    hotkey.into_option(),
                ))
            },
            CfgCommand::Quitout { hotkey } => quitout(chains.quitout.clone(), hotkey.into_option()),
            CfgCommand::Group { label, commands } => {
                if let Some(skip_widget) =
                    skipped_group_widget(settings.features.clone(), exit, &label)
                {
                    return skip_widget;
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

fn skipped_group_widget(
    features: Vec<feature::Feature>,
    mut exit: bool,
    label: &String,
) -> Option<Option<Box<dyn Widget>>> {
    features.iter().filter(|feature| !feature.visible).for_each(|feature| {
        if !exit {
            if let Some(feature_group) = feature.group.as_ref() {
                exit = feature_group == label;
            }
        }
    });

    if exit {
        return Some(Some(Box::new(NoneWidget {})));
    }
    None
}

enum Flag {
    StringFlag(String),
    FlagSpecFlag(FlagSpec),
}

fn skipped_widget(
    features: Vec<feature::Feature>,
    exit: &mut bool,
    flag: &Flag,
) -> Option<Option<Box<dyn Widget>>> {
    features.iter().filter(|feature| !feature.visible).for_each(|feature| {
        if !*exit {
            if let Some(feature_flag) = feature.flag.as_ref() {
                if let Ok(feature_flag_spec) = FlagSpec::try_from(feature_flag.clone()) {
                    match flag {
                        Flag::StringFlag(string_flag) => {
                            if let Ok(widget_flag_spec) = FlagSpec::try_from(string_flag.clone()) {
                                *exit = feature_flag_spec.label == widget_flag_spec.label;
                            }
                        },
                        Flag::FlagSpecFlag(widget_flag_spec) => {
                            *exit = feature_flag_spec.label == widget_flag_spec.label;
                        },
                    }
                } else {
                    match flag {
                        Flag::StringFlag(string_flag) => {
                            *exit = feature_flag == string_flag;
                        },
                        Flag::FlagSpecFlag(widget_flag_spec) => {
                           println!("Something unexpected happened when trying to skip a widget. Here's the data for widget and feature flags: {:?}, {:?}", widget_flag_spec, feature_flag);
                        },
                    }
                }
            }
        }
    });

    if *exit {
        return Some(Some(Box::new(NoneWidget {})));
    }
    None
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
