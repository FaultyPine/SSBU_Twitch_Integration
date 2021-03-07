use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


static mut TRIP_ACTIVATE_TIME: u32 = 0;


const TRIP_DURATION_SECS: u32 = 15;

pub unsafe fn trip(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("trip");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "trip" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        TRIP_ACTIVATE_TIME = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "trip" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::status_kind(boma)) && MotionModule::frame(boma) <= 1.0 {
            if app::sv_math::rand(smash::hash40("fighter"), 3) == 0 { // 1/3 chance to trip
                StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_SLIP, true);
            }
        }

        if utils::is_time_range(TRIP_ACTIVATE_TIME, TRIP_DURATION_SECS) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
    }
}