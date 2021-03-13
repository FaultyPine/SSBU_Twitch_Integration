#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

#[macro_use(lazy_static)]
extern crate lazy_static;

mod voting;
mod effects;
mod sys_line;
mod utils;
mod hooks;
mod config;
mod twitch;

use serde::{Deserialize, Serialize};
/*
#[derive(PartialEq)]
enum PlayerType {
    CPU,
    PLAYER,
}*/
#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum GameModes {
    ChoosePlayer, // when the effect is randomized and the votes are to choose which player is affected by that effect 
    ChooseEffect, // when there is a rotation of random effects and the votes determine which effect is activated 
}

pub type VoteMap<'a> = std::sync::MutexGuard<'a, std::collections::HashMap<String, Box<effects::Vote>>>;

extern "C" {
    #[link_name = "\u{1}abort"]
    fn abort();
}

#[skyline::hook(replace = abort)]
fn abort_hook() {
    std::thread::sleep(std::time::Duration::from_secs(2));
    original!()();
}


#[skyline::main(name = "SSBU_Twitch_Integration")]
pub unsafe fn main() {
    skyline::install_hook!(abort_hook);
    sys_line::install();
    hooks::install();
    smash_utils::init_smash_utils();
    config::init_config();
    twitch::start_twitch_integration();
}