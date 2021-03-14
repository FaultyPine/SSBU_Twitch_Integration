use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;



pub unsafe fn curry(boma: &mut smash::app::BattleObjectModuleAccessor) {
    let id = smash_utils::gameplay::get_player_number(boma);
    let mut vote_map = voting::VOTES.lock().unwrap();
    let effect_struct = vote_map.get_mut("curry");
    if effect_struct.is_none() { return; }
    let effect_struct = effect_struct.unwrap();
    /* This block runs when we first enable "curry" */
    if !effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effects::toggle_effect_eff(boma, false, true);
        ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_CURRY), 0, 0, false, false);
        effect_struct.players[id] = Some(true);
    }
    /* This block will run once-per-frame after the first frame of "curry" being "enabled" */
    else if effect_struct.players[id].unwrap_or_default() && effect_struct.is_enabled {
        effect_struct.is_enabled = false;
        voting::init_votes(&mut vote_map);
    }
    /* This block runs when we should "disable" the effect */
    else if effect_struct.players[id].unwrap_or_default() && !effect_struct.is_enabled {
        effect_struct.players[id] = Some(false);
        effects::toggle_effect_eff(boma, false, false);
    }
}