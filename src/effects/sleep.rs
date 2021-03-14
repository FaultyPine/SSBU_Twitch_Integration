use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


const SLEEP_MAX_DURATION_IN_SECS: i32 = 5; // max amount of sleep (seconds)

pub unsafe fn sleep(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("sleep");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "sleep" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, true);
        }
        else {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL, true);
        }
        effect_struct.activate_times[id] = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "sleep" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        let sleep_duration: u32 = (smash::app::sv_math::rand(smash::hash40("fighter"), SLEEP_MAX_DURATION_IN_SECS)+1) as u32;
        if utils::is_time_range(effect_struct.activate_times[id], sleep_duration) || ![*FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL].contains(&StatusModule::status_kind(boma)) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
        else {
            if MotionModule::frame(boma) > 50.0 {
                MotionModule::set_frame(boma, 30.0, true);
            }
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
    }

}