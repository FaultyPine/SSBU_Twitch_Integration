TODO
- add effects from effects/mod.rs
- fix pythra/icies/characters with subfighters
- fix timing system to work with matches that aren't timed
- contact other smash youtubers/streamers to show it off
- release on gamebanana and twitter and stuff 



# SSBU Twitch Integration
  
A plugin written entirely in Rust through the [skyline-rs](https://github.com/ultimate-research/skyline-rs) framework that allows for interaction between a Twitch streamer's chat, and the game.
  
## Setup
  
Simply drag the contents of the [release zip](PUT_RELEASES_PAGE_URL_HERE) to the root of your SD card on your hacked switch.
  
Then, launch smash. A config file will be generated.
After smash gets to the title screen, close the game. 
Navigate to the config file located at "sd:/Twitch_Integration_Config.toml".
  
  
In this config file you can set the channel to pull chat messages from, the "gamemode", the time between each "voting interval", and most importantly,
your oauth token. You need an oauth token for the mod to function properly. Luckily, getting one is super easy.
  
#### Oauth Token

Visit this site:
https://twitchapps.com/tmi/
Click connect, and after connecting your twitch account, it will give you your oauth token. 
Paste the entire string it gives you into the oauth field in the config file.
  
    
#### Gamemodes

Currently, there are two gamemodes to choose from.
"ChooseEffect", and "ChoosePlayer".

ChooseEffect:
During each voting period, the chat will vote for one out of a selection of "effects". The highest-voted "effect" will apply to all players and cpus.
In the case of a draw, a random option out of the highest-voted options will be chosen.
  
ChoosePlayer:
During each voting period, a random effect will be chosen, the chat will then vote for which player should get the randomized effect.
In the case of a draw, a random option out of the highest-voted options will be chosen.

#### Voting Interval
  
The amount of time (in seconds) that passes in between each voting period. (Must be a positive integer)
  
  
  
  
  
## Contributing
  
Feel free to submit a PR. A great way to contribute is to add new "effects". It isn't super hard since you can mostly copy-paste from whats already there.
[Contact me on discord](https://discordapp.com/users/216754196253245440) with any questions!
  
## Troubleshooting/Issues
  
If you are having issues, I recommend using the skyline_logger_rust.exe logger application (found in the root of this repo) to see skyline logs from your switch.
If that doesn't give you a hint as to what the issue is, contact me or the [Smash Ultimate Modding Discord](https://discord.gg/ASJyTrZ) about your issue with the logs attached.

I am aware that initial boot times can be long. From my experience, it (for some reason) seems to load significantly slower on initial boot with a bad internet connection.
