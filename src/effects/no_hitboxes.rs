use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;

/*
static mut NO_HITBOXES_ACTIVATE_TIME: u32 = 0;

const NO_HITBOXES_DURATION: u32 = 15;

pub unsafe fn no_hitboxes(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("no-hitboxes").unwrap();
    /* This block runs when we first enable "no_hitboxes" */
    if !IS_NO_HITBOXES && effect_struct.is_enabled {
        NO_HITBOXES_ACTIVATE_TIME = utils::get_remaining_time_as_seconds();
        IS_NO_HITBOXES = true;
    }
    /* This block will run once-per-frame after the first frame of "no_hitboxes" being "enabled" */
    else if IS_NO_HITBOXES && effect_struct.is_enabled {
        if utils::is_time_range(NO_HITBOXES_ACTIVATE_TIME, NO_HITBOXES_DURATION) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
        
    }
    /* This block runs when we should "disable" the effect */
    else if IS_NO_HITBOXES && !effect_struct.is_enabled {
        IS_NO_HITBOXES = false;
    }
}
*/