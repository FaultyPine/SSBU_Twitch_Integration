use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


static mut REVERSE_DIR_ACTIVATE_TIME: u32 = 0;

const REVERSE_DIR_DURATION: u32 = 15;
const REVERSE_STICK_FLICK_FRAMES: i32 = 1;
const REVERSE_STICK_FLICK_SENS: f32 = 0.5;

pub unsafe fn reverse_dir(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("reverse");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "reverse" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effects::toggle_effect_eff(boma);
        REVERSE_DIR_ACTIVATE_TIME = utils::get_remaining_time_as_seconds();
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "reverse" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        if ControlModule::get_flick_x(boma) < REVERSE_STICK_FLICK_FRAMES && ControlModule::get_stick_x(boma).abs() >= REVERSE_STICK_FLICK_SENS {
            println!("Flick");
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
            ControlModule::reset_main_stick(boma);
        }

        if utils::is_time_range(REVERSE_DIR_ACTIVATE_TIME, REVERSE_DIR_DURATION) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effects::toggle_effect_eff(boma);
        effect_struct.players[id] = Some(false);
    }
}