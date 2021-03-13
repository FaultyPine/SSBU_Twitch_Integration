use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


static mut LOW_GRAV_ACTIVATE_TIME: u32 = 0;

const LOW_GRAV_DURATION_SECS: u32 = 15;

pub unsafe fn low_grav(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("low-grav");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "low_grav" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, true);
        LOW_GRAV_ACTIVATE_TIME = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true); // see hooks/get_param.rs
    }
    /* This block will run once-per-frame after the first frame of "low_grav" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if utils::is_time_range(LOW_GRAV_ACTIVATE_TIME, LOW_GRAV_DURATION_SECS) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, false);
        effect_struct.players[id] = Some(false);
    }
}