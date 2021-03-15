use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::hash40;
use crate::*;
use smash_utils::DEFAULT_VEC3;

pub const TURBO_ACTIVE_EFFECT_STR: &str = "sys_revenge_aura";
pub const TURBO_ACTIVATE_EFFECT_OFFSET_FROM_TOP: Vector3f = Vector3f {x: 0.0, y: 7.0, z: 0.0};
const TURBO_DURATION: u32 = 20;

pub unsafe fn turbo(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut(effects::TURBO);
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "turbo" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effect_struct.activate_times[id] = utils::get_remaining_time_as_seconds();
        turbo_activate(boma);
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "turbo" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        turbo_mode(boma);
        /* Once TURBO_DURATION seconds have elapsed since turbo was activated, disable it */
        if utils::is_time_range(effect_struct.activate_times[id], TURBO_DURATION) {
            effect_struct.is_enabled = false;
            voting::init_votes(&mut vote_map);
        }
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        reset_turbo_mode(boma);
        effect_struct.players[id] = Some(false);
    }
}


//called once per frame while turbo mode is on
unsafe fn turbo_mode(boma: &mut app::BattleObjectModuleAccessor) {
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
        CancelModule::enable_cancel(boma);
    }
}

//called once when turbo should be "turned on"
pub unsafe fn turbo_activate(boma: &mut app::BattleObjectModuleAccessor) {
    let eff_size = WorkModule::get_param_float(boma, hash40("shield_radius"), 0) / 7.8;
    EffectModule::req_follow(boma, Hash40::new(TURBO_ACTIVE_EFFECT_STR), Hash40::new("top"), &TURBO_ACTIVATE_EFFECT_OFFSET_FROM_TOP, &DEFAULT_VEC3, eff_size, false, 0, 0, 0, 0, 0, false, false);
    effects::toggle_effect_eff(boma, false, true);
}

//called once when turbo should be "turned off"
unsafe fn reset_turbo_mode(boma: &mut app::BattleObjectModuleAccessor) {
    effects::toggle_effect_eff(boma, false, false);
    EffectModule::kill_kind(boma, Hash40::new(TURBO_ACTIVE_EFFECT_STR), false, true);
}