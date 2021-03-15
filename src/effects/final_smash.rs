use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash_utils::gameplay::get_player_number;
use crate::*;



pub unsafe fn final_smash(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut(effects::FINALSMASH);
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "final_smash" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        let mgr = smash_utils::gameplay::get_fighter_manager().unwrap();
        let entry_id = app::FighterEntryID(id as i32);
        let available_final = app::FighterAvailableFinal{ _address: 0 };
        FighterManager::set_final(mgr, entry_id, available_final, 0);
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "final_smash" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effect_struct.is_enabled = false;
        voting::init_votes(&mut vote_map);
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
    }
}