use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


static mut SLOWMO_ACTIVATE_TIME: u32 = 0;


const SLOWMO_DURATION_SECS: u32 = 15;

pub unsafe fn slow_mo(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("slow-mo");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "slow_mo" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        SlowModule::set_whole(boma, 4, 0); // first arg is "intensity" of slowdown i think
        SLOWMO_ACTIVATE_TIME = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "slow_mo" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if utils::is_time_range(SLOWMO_ACTIVATE_TIME, SLOWMO_DURATION_SECS) {
            SlowModule::clear_whole(boma);
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
    }
}