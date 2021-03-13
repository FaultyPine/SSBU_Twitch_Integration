use std::collections::HashMap;
use std::sync::Mutex;
use smash::app::lua_bind::FighterManager;
use smash_utils::gameplay::get_fighter_manager;
use crate::*;

/* Holds votes */
lazy_static!(
    pub static ref VOTES: Mutex<HashMap<String, Box<effects::Vote>>> = Mutex::new(HashMap::new());
);

/* Num of effects in a voting rotation at a time */
pub const VOTING_ROTATION_NUM_EFFECTS: usize = 3;

/* Beginning "timestamp" of last voting period */
pub static mut VOTING_PERIOD_BEGIN: u32 = 0;

/* Effects that are in the current "rotatation" of voting. When the gamemode is ChoosePlayer, the current effect is just the effect at position 0 here */
pub static mut VOTING_ROTATION_EFFECTS: [&str ; VOTING_ROTATION_NUM_EFFECTS] = ["" ; VOTING_ROTATION_NUM_EFFECTS];

/* Names of people who have already voted */
static mut VOTER_NAMES: Vec<String> = Vec::new();

/* For ChoosePlayer, this is the player ID that we should run effects for. */
pub static mut PLAYER_EFFECT_NUMBER: Option<usize> = None;

/* Called multiple times a second with chat_msg/chatter_name being blank if no chat messages were sent since last call, or relevant values if there were. */
pub unsafe fn update_votes(chat_msg: &String, chatter_name: String) {
    if VOTING_PERIOD_BEGIN != 0 && smash_utils::externs::is_ready_go() {
        let config = config::CONFIG.clone().unwrap();
        let chat_msg = chat_msg.to_lowercase().trim().to_string();
        let votes = VOTES.lock().unwrap();
        match config.mode {
            GameModes::ChooseEffect => {
                choose_effect_update_votes(&chat_msg, chatter_name, &config, votes);
            }
            GameModes::ChoosePlayer => {
                choose_player_update_votes(&chat_msg, chatter_name, &config, votes);
            }
        }
    }
}

/*   ChooseEffect   */

unsafe fn choose_effect_update_votes(chat_msg: &String, chatter_name: String, config: &Box<config::Config>, mut votes: VoteMap) {
    /* Parse chat msg string into usize */
    if let Ok(parsed) = chat_msg.parse::<usize>() {
        /* If message is a vote */
        if (1..VOTING_ROTATION_NUM_EFFECTS + 1).any(|x| x == parsed) {
            /* parse the type of vote being cast */
            let vote_msg = VOTING_ROTATION_EFFECTS[parsed-1].to_string();
            /* increment the votes on that effect struct */
            if let Some(vote) = votes.get_mut(&vote_msg) {
                /* If person casting the vote hasn't already voted */
                if !VOTER_NAMES.contains(&chatter_name) {
                    vote.votes[0] += 1;
                    VOTER_NAMES.push(chatter_name);
                    print_votes(&mut votes, true);
                }
            }
        }
    }

    /* check if the voting period is over */
    if utils::is_time_range(voting::VOTING_PERIOD_BEGIN, config.voting_interval) {
        /* Iterate through votes that are currently in rotation and find entry with the highest # of votes. max_votes: (key, #votes) */
        let mut max_votes: Vec<(String, usize)> = vec![];
        for &effect_str in VOTING_ROTATION_EFFECTS.iter() {
            let v = votes.get(effect_str).unwrap();
            if max_votes.iter().any(|tup| v.votes[0] > tup.1) {
                max_votes = vec![(effect_str.to_string(), v.votes[0])];
            }
            else if max_votes.len() < 1 || v.votes[0] == max_votes.get(0).unwrap().1 {
                max_votes.push((effect_str.to_string(), v.votes[0]));
            }
        }

        /* enable effects for the voting entry that has the most votes, and if there's multiple votes with the same # of votes, randomize which one we pick */
        if max_votes.len() > 1 {
            println!("Vote result is a tie! Randomizing between the highest votes...");
        }
        let random_idx = smash::app::sv_math::rand(smash::hash40("fighter"), max_votes.len() as i32) as usize;
        let max_votes_entry = max_votes.get(random_idx).unwrap();
        if let Some(max_vote_effect) = votes.get_mut(&max_votes_entry.0) {
            max_vote_effect.is_enabled = true;
            println!("---------\nVote result: {}", max_votes_entry.0);
            randomize_voting_rotation();
            print_votes(&mut votes, false);
        }
        else {
            println!("Error! Couldn't find vote entry. Idx = {}  Entry = {:#?}", random_idx, max_votes_entry);
        }

        /* Reset voting period */
        voting::VOTING_PERIOD_BEGIN = utils::get_remaining_time_as_seconds();
    }
}



/*   ChoosePlayer   */

unsafe fn choose_player_update_votes(chat_msg: &String, chatter_name: String, config: &Box<config::Config>, mut votes: VoteMap) {
    /* If the votes hashmap is properly populated with all our fighters */
    if IS_PLAYERS_POPULATED {
        
        /* If we can parse chat msg string into usize, then it's a "valid" vote */
        if let Ok(parsed) = chat_msg.parse::<usize>() {
            /* If message is a vote for a valid player */
            if parsed <= FighterManager::total_fighter_num(get_fighter_manager().unwrap()) as usize && parsed > 0 {
                /* increment the votes on that effect struct */
                if let Some(vote) = votes.get_mut(VOTING_ROTATION_EFFECTS[0]) {
                    /* If person casting the vote hasn't already voted */
                    if !VOTER_NAMES.contains(&chatter_name) {
                        vote.votes[parsed-1] += 1;
                        VOTER_NAMES.push(chatter_name);
                        print_votes(&mut votes, true);
                    }
                }
            }
        }

        /* check if the voting period is over */
        if utils::is_time_range(voting::VOTING_PERIOD_BEGIN, config.voting_interval) {

            /* Iterate through players and find entry with the highest # of votes. max_votes: (key, #votes) */
            let mut max_votes: Vec<(String, usize)> = vec![];
            let votes_struct = votes.get(VOTING_ROTATION_EFFECTS[0]).unwrap();
            let votes_arr = votes_struct.votes;
            for vote_idx in 0..votes_arr.len() {
                /* If the current player idx represents a valid player in-game */
                if votes_struct.players[vote_idx].is_some() {
                    if max_votes.iter().any(|tup| votes_arr[vote_idx] > tup.1) {
                        max_votes = vec![((vote_idx+1).to_string(), votes_arr[vote_idx])];
                    }
                    else if max_votes.len() < 1 || votes_arr[vote_idx] == max_votes.get(0).unwrap().1 {
                        max_votes.push(((vote_idx+1).to_string(), votes_arr[vote_idx]));
                    }
                }
            }

            /* enable effects for the voting entry that has the most votes, and if there's multiple votes with the same # of votes, randomize which one we pick */
            if max_votes.len() > 1 {
                println!("Vote result is a tie! Randomizing between the highest votes...");
            }
            let random_idx = smash::app::sv_math::rand(smash::hash40("fighter"), max_votes.len() as i32) as usize;
            let max_votes_entry = max_votes.get(random_idx).unwrap();
            if let Some(max_vote_effect) = votes.get_mut(VOTING_ROTATION_EFFECTS[0]) {
                max_vote_effect.is_enabled = true;
                PLAYER_EFFECT_NUMBER = Some(max_votes_entry.0.parse::<usize>().unwrap()-1);
                println!("---------\nVote result: Player {}!", max_votes_entry.0);
                randomize_voting_rotation();
                print_votes(&mut votes, false);
            }
            else {
                println!("Error! Couldn't find vote entry. Idx = {}  Entry = {:#?}", random_idx, max_votes_entry);
            }

            /* Reset voting period */
            voting::VOTING_PERIOD_BEGIN = utils::get_remaining_time_as_seconds();
        }

    }
}

/* Called when game boots, and after a voting period is over (in sys_line context) */
pub unsafe fn init_votes(votes: &mut VoteMap) {
    match config::CONFIG.clone().unwrap().mode {

        GameModes::ChooseEffect => {
            /* Clear votes hashmap so we can re-init */
            votes.clear();
            /* Iterate through possible effect names and initialize new k,v pairs for those names */
            for &effect_name in effects::EFFECT_NAMES.iter() {
                let effect_name = effect_name.to_string();
                let default_struct = Box::new(effects::Vote::new());
                votes.insert(effect_name, default_struct);
            }
        }

        GameModes::ChoosePlayer => {
            /* If votes hashmap is empty, fill it with default k,v pairs */
            if votes.is_empty() {
                for &effect_name in effects::EFFECT_NAMES.iter() {
                    let effect_name = effect_name.to_string();
                    let default_struct = Box::new(effects::Vote::new());
                    votes.insert(effect_name, default_struct);
                }
            }
            else { /* If hashmap isn't empty, than this is a call to init_votes during a match, after an effect expires */
                /* Iterate through votes hashmap, and reset relevant values */
                for vote in votes.iter() {
                    let mut vote = **vote.1;
                    vote.is_enabled = false;
                    vote.votes = [0;8];
                    /* Reset players that have some values to false */
                    for p in vote.players.iter_mut() {
                        if p.is_some() {
                            *p = Some(false);
                        }
                    }
                }
                PLAYER_EFFECT_NUMBER = None;
            }

        }

    }
}

pub unsafe fn randomize_voting_rotation() {
    for rotation_slot in 0..VOTING_ROTATION_NUM_EFFECTS {
        let mut random_effect = VOTING_ROTATION_EFFECTS[rotation_slot];
        while VOTING_ROTATION_EFFECTS.contains(&random_effect) {
            random_effect = effects::EFFECT_NAMES[smash::app::sv_math::rand(smash::hash40("fighter"), effects::EFFECT_NAMES.len() as i32) as usize];
        }
        VOTING_ROTATION_EFFECTS[rotation_slot] = random_effect;
    }
    /* Every time we re-randomize the votes, clear the list of people who have already voted */
    VOTER_NAMES.clear();
}



pub unsafe fn print_votes(votes: &mut VoteMap, should_display_votes: bool) {
    if !VOTING_ROTATION_EFFECTS.contains(&"") {
        match config::CONFIG.clone().unwrap().mode {
            GameModes::ChooseEffect => {
                println!("---------");
                println!("Current votes: (Type corresponding number to vote)");
                let mut num_vote = 1;
                for &eff_name in VOTING_ROTATION_EFFECTS.iter() {
                    match should_display_votes {
                        true => println!("{}) {} | Votes: [{}]", num_vote, eff_name, votes.get(eff_name).unwrap().votes[0]),
                        false => println!("{}) {}", num_vote, eff_name)
                    }
                    num_vote += 1;
                }
                println!("---------");
            }
            GameModes::ChoosePlayer => {
                println!("---------");
                println!("Current Effect: {}\n Who will you give it to? (Type corresponding player number to vote)", VOTING_ROTATION_EFFECTS[0]); // just grab the first effect since they get randomized anyway
                let vote_struct = votes.get(VOTING_ROTATION_EFFECTS[0]).unwrap();
                let num_votes = vote_struct.votes;
                for idx in 0..num_votes.len() {
                    if vote_struct.players[idx].is_some() {
                        match should_display_votes {
                            true => println!("Player {} | Votes: {}", idx+1, num_votes[idx]),
                            false => println!("Player {}", idx+1)
                        }
                    }
                }
                println!("---------");
            }
        }
    }
}

/* 
Called once per frame via sys_line.
A better way to do this would be to nro hook sys_line_system_init...
*/
pub static mut IS_PLAYERS_POPULATED: bool = false;
pub unsafe fn get_players(boma: &mut smash::app::BattleObjectModuleAccessor, votes: &mut VoteMap) {
    /*
    Iterate through vote structs - if the number of Some(x) values in the players array is equal to (or greater than) the total number of ingame fighters, 
    then we've populated the player arrays properly and set IS_PLAYERS_POPULATED to true.

    A None value in the players array means that player slot isn't populated. EX: if there's a player 1 & 2 in a match, the players array (after init) will be [Some(false), Some(false), None, None, None, None, None, None]

    The boolean contained in the Some value represents if that effect is active for that player.

    Otherwise, we initialize the proper slot in the players array with Some(false)
    */
// fix ice climbers
    if !IS_PLAYERS_POPULATED && VOTING_PERIOD_BEGIN != 0 {
        let id = smash_utils::gameplay::get_player_number(boma);
        let total_num_fighters = FighterManager::total_fighter_num(get_fighter_manager().unwrap()) as usize;
        for vote in votes.iter_mut() {
            let mut vote_struct = vote.1;
            let num_somes = vote_struct.players.iter().filter(|x| x.is_some()).count();
            if num_somes /* Number of Some(x) values in vote_struct.players */ >= total_num_fighters {
                IS_PLAYERS_POPULATED = true;
                print_votes(votes, false);
                break;
            }
            else {
                vote_struct.players[id] = Some(false);
            }
        }
    }
}