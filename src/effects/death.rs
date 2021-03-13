use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;



pub unsafe fn death(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("death");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "death" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_DEAD, true);
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "death" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effect_struct.is_enabled = false;
        voting::init_votes(&mut vote_map);
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
    }

}