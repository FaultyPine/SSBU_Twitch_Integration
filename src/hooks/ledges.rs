use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;

pub fn install() {
    skyline::install_hooks!(
        can_entry_cliff_hook
    );
}



#[skyline::hook(replace = GroundModule::can_entry_cliff)]
unsafe fn can_entry_cliff_hook(boma: &mut app::BattleObjectModuleAccessor) -> u64 {
    let ret = original!()(boma);
    let id = smash_utils::gameplay::get_player_number(boma);
    let votes = voting::VOTES.try_lock();
    if votes.is_err() { return ret; }
    let mut votes = votes.unwrap();

    if votes.contains_key(effects::NOLEDGES) && votes.get_mut(effects::NOLEDGES).unwrap().players[id].unwrap_or_default() {
        return 0;
    }
    
    ret
}