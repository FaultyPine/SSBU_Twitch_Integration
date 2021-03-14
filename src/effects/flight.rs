use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;

const FLIGHT_SPEED_X_COEFF: f32 = 1.3;
const FLIGHT_SPEED_Y_COEFF: f32 = 1.4;

const FLIGHT_DURATION_TIME: u32 = 15;

pub unsafe fn flight(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("flight");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "flight" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, true, true);
        effect_struct.activate_times[id] = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "flight" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if utils::is_time_range(effect_struct.activate_times[id], FLIGHT_DURATION_TIME) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
        else if smash_utils::externs::get_remaining_time_as_frame() % 4 == 0 {
            /* Flight logic */
            let add_y_vel = ControlModule::get_stick_y(boma) * FLIGHT_SPEED_Y_COEFF;
            let add_x_vel = ControlModule::get_stick_x(boma) * FLIGHT_SPEED_X_COEFF;
            let flight_speed = smash::phx::Vector3f { x: add_x_vel, y: add_y_vel, z: 0.0};
            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &flight_speed);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, true, false);
        effect_struct.players[id] = Some(false);
    }
}