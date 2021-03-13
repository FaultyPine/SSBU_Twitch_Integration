use smash::app::{BattleObjectModuleAccessor, lua_bind::*, utility::*};
use smash::lib::lua_const::*;
use smash::hash40;

use crate::*;

pub fn install() {
    skyline::install_hooks!(
        get_param_int_hook,
        get_param_float_hook,
    );
}

static INT_OFFSET: isize = 0x4E19D0; // 11.0.0


#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_hook(x0: u64, x1: u64, x2 :u64) -> i32 {
    //let boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	//let fighter_kind = get_kind(&mut *boma);
	//let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let ret = original!()(x0, x1, x2);

    ret
}

#[skyline::hook(offset=INT_OFFSET+0x40)]
pub unsafe fn get_param_float_hook(x0: u64, x1: u64, x2 :u64) -> f32 {
    let boma = &mut *(*((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    //let fighter_kind = get_kind(&mut *boma);
	let id = smash_utils::gameplay::get_player_number(boma);

    let ret = original!()(x0, x1, x2);
    let votes = voting::VOTES.try_lock();
    if votes.is_err() { return ret; }
    let mut votes = votes.unwrap();

    if x2 == 0 && x1 == hash40("air_accel_y") {
        if votes.contains_key("low-grav") && votes.get_mut("low-grav").unwrap().players[id].unwrap_or_default() {
            return ret/2.0;
        }
        else if votes.contains_key("flight") && votes.get_mut("flight").unwrap().players[id].unwrap_or_default() {
            return ret/2.3;
        }
    }

    ret
}
