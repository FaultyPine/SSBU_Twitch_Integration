use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use crate::*;

pub fn install() {
    acmd::add_custom_hooks!(sys_line);
}


/* Runs once-per-frame, per-fighter in a match */
pub fn sys_line(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let boma = app::sv_system::battle_object_module_accessor(lua_state);
        
        /* If its a player, and its not a cpu, and we're not in training mode,
         then we want to run our effect funcs which themselves handle whether they should be "on" or not */
        if app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER 
        && !smash_utils::externs::is_training_mode()
        {

            static mut EFF_IDX: usize = 0;

            if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && voting::PLAYER_EFFECT_NUMBER.is_none()
            {
                let test_eff = effects::EFFECT_NAMES[EFF_IDX];
                println!("Force effect: {}", test_eff);
                // effect testing zone
                for _ in 0..1 {
                    let vote_map = voting::VOTES.try_lock();
                    if vote_map.is_err() { println!("Failed to lock mutex!"); break; }
                    let mut vote_map = vote_map.unwrap();
                    let effect_struct = vote_map.get_mut(test_eff);
                    if effect_struct.is_none() { println!("Effect struct was none!"); break; }
                    let effect_struct = effect_struct.unwrap();

                    effect_struct.is_enabled = true;
                    voting::PLAYER_EFFECT_NUMBER = Some(0);
                }

            }
            else if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)
            {
                EFF_IDX = (EFF_IDX + 1) % effects::EFFECT_NAMES.len();
                println!("Eff: {} : idx {}", effects::EFFECT_NAMES[EFF_IDX], EFF_IDX);
            }

            effects::once_per_frame(boma, fighter);
            let mut votes = voting::VOTES.lock().unwrap();
            handle_game_resets(boma, &mut votes);
            if config::CONFIG.clone().unwrap().mode == crate::GameModes::ChoosePlayer {
                voting::get_players(boma, &mut votes);
            }
        }
    }
}


unsafe fn handle_game_resets(_boma: &mut app::BattleObjectModuleAccessor, votes: &mut VoteMap) {
    static mut LAST_READY_GO: bool = false;
    static mut IS_READY_GO: bool = true;

    IS_READY_GO = smash_utils::externs::is_ready_go();

    //THIS BLOCK RUNS WHEN A "SESSION" ENDS
    if !IS_READY_GO && LAST_READY_GO
    {
        //votes.clear();
        voting::VOTING_PERIOD_BEGIN = 0;
    }
    //THIS BLOCK RUNS WHEN A "SESSION" BEGINS
    else if IS_READY_GO && !LAST_READY_GO
    {
        println!("Init!");
        voting::IS_PLAYERS_POPULATED = false;
        voting::VOTING_PERIOD_BEGIN = utils::get_remaining_time_as_seconds();
        voting::init_votes(votes);
        voting::randomize_voting_rotation();
        if config::CONFIG.clone().unwrap().mode == GameModes::ChooseEffect {
            voting::print_votes(votes, false);
        }
    }
    LAST_READY_GO = IS_READY_GO;
}


/* DEPRECATED in favor of allowing the user to pick their gamemode based on the config file. */

/* Automatically determine what gamemode we should use based on the number of players in a match */

/*
static mut PLAYER_TYPES: Vec<PlayerType> = Vec::new();
pub static mut GAME_MODE: GameModes = GameModes::Undetermined;

/* This will run for the first few iterations of sys_line and will determine how many players are in a match. */
unsafe fn determine_voting_mode(boma: &mut app::BattleObjectModuleAccessor) {
    if GAME_MODE == GameModes::Undetermined {
        let entry_id_int = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
        let entry_id = app::FighterEntryID(entry_id_int);
        let mgr = *(utils::FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    
        /* when we've iterated through each fighter at least once */
        if PLAYER_TYPES.len() == FighterManager::total_fighter_num(mgr) as usize {
            let num_players = PLAYER_TYPES.iter().filter(|&x| *x == PlayerType::PLAYER).count();
            match num_players {
                0 | 1 => GAME_MODE = GameModes::ChooseEffect,
                _ => GAME_MODE = GameModes::ChoosePlayer,
            };
        }
        else {
            /* Add current fighter's type to vec */
            match utils::is_operation_cpu(boma) {
                true => PLAYER_TYPES.push(PlayerType::PLAYER),
                false => PLAYER_TYPES.push(PlayerType::CPU),
            }
        }
    }
}
*/