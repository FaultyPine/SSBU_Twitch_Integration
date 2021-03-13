use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


static mut JUMPS_ACTIVATE_TIME: u32 = 0;

const JUMPS_DURATION_SECS: u32 = 15;

pub unsafe fn double_jumps(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("jumps");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "jumps" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, true);
        JUMPS_ACTIVATE_TIME = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "jumps" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if utils::is_time_range(JUMPS_ACTIVATE_TIME, JUMPS_DURATION_SECS) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
        else {
            WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, false);
        effect_struct.players[id] = Some(false);
    }
}