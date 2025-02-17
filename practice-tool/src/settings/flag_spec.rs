use libeldenring::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct FlagSpec {
    pub label: String,
    pub getter: fn(&Pointers) -> &Bitflag<u8>,
}

impl std::fmt::Debug for FlagSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlagSpec {{ label: {:?} }}", self.label)
    }
}

impl FlagSpec {
    fn new(label: &str, getter: fn(&Pointers) -> &Bitflag<u8>) -> FlagSpec {
        FlagSpec { label: label.to_string(), getter }
    }
}

impl TryFrom<String> for FlagSpec {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        macro_rules! flag_spec {
          ($x:expr, [ $( ($flag_name:ident, $flag_label:expr), )* ]) => {
              match $x {
                  $(stringify!($flag_name) => Ok(FlagSpec::new($flag_label, |c| &c.$flag_name)),)*
                  e => Err(format!("\"{}\" is not a valid flag specifier", e)),
              }
          }
      }
        flag_spec!(value.as_str(), [
            (one_shot, "One shot"),
            (no_damage, "All no damage"),
            (no_dead, "No death"),
            (no_hit, "No hit"),
            (no_goods_consume, "Inf Consumables"),
            (no_stamina_consume, "Inf Stamina"),
            (no_fp_consume, "Inf Focus"),
            (no_ashes_of_war_fp_consume, "Inf Focus (AoW)"),
            (no_arrows_consume, "Inf arrows"),
            (no_attack, "No attack"),
            (no_move, "No move"),
            (no_update_ai, "No update AI"),
            (no_trigger_event, "No trigger events"),
            (runearc, "Rune Arc"),
            (gravity, "No Gravity"),
            (torrent_gravity, "No Gravity (Torrent)"),
            (collision, "No Collision"),
            (torrent_collision, "No Collision (Torrent)"),
            (display_stable_pos, "Show stable pos"),
            (weapon_hitbox1, "Weapon hitbox #1"),
            (weapon_hitbox2, "Weapon hitbox #2"),
            (weapon_hitbox3, "Weapon hitbox #3"),
            (hitbox_high, "High world hitbox"),
            (hitbox_low, "Low world hitbox"),
            (hitbox_f, "Walls hitbox"),
            (hitbox_character, "Character hitbox"),
            (hitbox_event, "Event hitbox"),
            (field_area_direction, "Direction HUD"),
            (field_area_altimeter, "Altimeter HUD"),
            (field_area_compass, "Compass HUD"),
            // (show_map, "Show/hide map"),
            (show_chr, "Show/hide character"),
            (show_all_map_layers, "Show all map layers"),
            (show_all_graces, "Show all graces"),
        ])
    }
}
