use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;

const NO_LEDGES_DURATION: u32 = 15;

pub unsafe fn no_ledges(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut(effects::NOLEDGES);
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "no_ledges" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effect_struct.activate_times[id] = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true); // see hooks/ledges.rs
        effects::toggle_effect_eff(boma, true, true);
    }
    /* This block will run once-per-frame after the first frame of "no_ledges" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if utils::is_time_range(effect_struct.activate_times[id], NO_LEDGES_DURATION) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
        effects::toggle_effect_eff(boma, true, false);
    }
}