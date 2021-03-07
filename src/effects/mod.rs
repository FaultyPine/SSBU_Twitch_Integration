use crate::*;

/*
TODO:

 Add more effects:
 - Increased hitstun
 - chaos (multiple effects at random)
 - poison/flower
 - crit hits at random
 - low friction
 - wind
 - randomly scale up/down
 - metal effect + controller off for random amnt of time

Instead of picking random effect and applying to all chars,
Pick effect at random and allow audience to select which player it goes to

*/

/* 
I know that doing it this way isn't the most efficient. I could simply have the relevant effect functions run once-per-frame
and could also probably create some sorta effect object that would have func pointers to relevant ones so there isn't as much copy-paste
code for each effect, but making things into a single-object type situation wouldn't allow for multiple effects to be running on top of each other.
If someone sets the voting timer to 1 second or something like that I want effects to be able to overlap each other. So this does the trick for now at least
*/

pub mod turbo;
pub mod curry;
pub mod death;
pub mod sleep;
pub mod low_grav;
pub mod slow_mo;
pub mod flight;
pub mod trip;
pub mod final_smash;
pub mod reverse_dir;
pub mod double_jumps;
pub mod dmg;
pub mod no_hitboxes;
pub mod no_ledges;

pub unsafe fn once_per_frame(boma: &mut smash::app::BattleObjectModuleAccessor) {
    match config::CONFIG.clone().unwrap().mode {
        GameModes::ChooseEffect => {
            all_effects(boma);
        }
        GameModes::ChoosePlayer => {
            if voting::PLAYER_EFFECT_NUMBER.is_some() && smash_utils::gameplay::get_player_number(boma) == voting::PLAYER_EFFECT_NUMBER.unwrap() {
                all_effects(boma);
            }
        }
    }

}

unsafe fn all_effects(boma: &mut smash::app::BattleObjectModuleAccessor) {
    turbo::turbo(boma);
    curry::curry(boma);
    death::death(boma);
    sleep::sleep(boma);
    low_grav::low_grav(boma);
    slow_mo::slow_mo(boma);
    flight::flight(boma);
    trip::trip(boma);
    final_smash::final_smash(boma);
    reverse_dir::reverse_dir(boma);
    double_jumps::double_jumps(boma);
    dmg::dmg(boma);
    //no_hitboxes::no_hitboxes(boma);
    no_ledges::no_ledges(boma);
}


pub const EFFECT_NAMES: &[&str] = &[
    "turbo",
    "curry",
    "death",
    "sleep",
    "low-grav",
    "slow-mo",
    "flight",
    "trip",
    "final-smash",
    "reverse",
    "jumps",
    "dmg-or-heal",
    //"no-hitboxes",
    "no-ledges",
];

#[derive(Clone, Copy, Debug)]
pub struct Vote { 
    /* 
    Each player ingame has a votes idx. When the gamemode is ChoosePlayer, people vote on a specific effect, and each vote increments the corresponding idx in this array
    When the mode is ChooseEffect, only the first idx of this array is used
    */
    pub votes: [usize ; 8],
    /*
    If this effect is currently "enabled"
    */
    pub is_enabled: bool, 
    /*
    Similar to is_enabled, this array represents each player in a match and if this effect is enabled for that player.
    Having this array allows for multiple players to be affected by an effect at once
    An entry in this array is None when that player idx doesn't exist, and it is Some when that player does exist. 
     I.E if there is player 1 and player 3 in a match, the array will look like this: 
       [Some(true/false), None, Some(true/false), None, None, None, None, None]
    */
    pub players: [Option<bool> ; 8]
}

impl Vote {
    pub fn new() -> Self {
        Self {
            votes: [0;8],
            is_enabled: false,
            players: [None ; 8],
        }
    }
}


use smash_utils::DEFAULT_VEC3;
use smash::phx::*;

pub static DEFAULT_EFFECT_ON_EFF: &str = "sys_sp_flash";
pub const DEACTIVATE_EFFECT_OFFSET_FROM_TOP: Vector3f = Vector3f {x: 7.0, y: 18.0, z: 0.0};

pub unsafe fn toggle_effect_eff(boma: &mut smash::app::BattleObjectModuleAccessor) -> u32 {
    smash::app::lua_bind::EffectModule::req_on_joint(boma, Hash40::new(effects::DEFAULT_EFFECT_ON_EFF), Hash40::new("top"),
        &DEACTIVATE_EFFECT_OFFSET_FROM_TOP, &DEFAULT_VEC3, 1.5, &DEFAULT_VEC3, &DEFAULT_VEC3,
        false, 0, 0, 0) as u32
}