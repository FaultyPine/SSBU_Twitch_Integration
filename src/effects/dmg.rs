use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;


const DMG_AMNT: f32 = 15.0; // amount to either heal or damage

pub unsafe fn dmg(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("dmg-or-heal");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "dmg" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        let eff_handle = effects::toggle_effect_eff(boma, true);
        if app::sv_math::rand(smash::hash40("fighter"), 2) == 0 {
            EffectModule::set_rgb(boma, eff_handle, 0.0, 1.0, 0.1);
            DamageModule::heal(boma, DMG_AMNT, 0);
        }
        else {
            EffectModule::set_rgb(boma, eff_handle, 1.0, 0.1, 0.0);
            DamageModule::heal(boma, -DMG_AMNT, 0);
        }
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "dmg" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effect_struct.is_enabled = false;
        voting::init_votes(&mut vote_map);
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
        effects::toggle_effect_eff(boma, false);
    }
}