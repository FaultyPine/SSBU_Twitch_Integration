use std::{io::BufRead, net::*};
use std::io::*;
use crate::{*, utils::*};

/* socket variables */
const SERVER: &str = "34.217.198.238"; // irc.chat.twitch.tv
const PORT: u16 = 80;
// IRC SPEC: https://tools.ietf.org/html/rfc1459 

pub unsafe fn get_chat_msg_info_loop(stream: &mut TcpStream, CHANNEL: String) -> Result<()> {
    let _ = stream.flush();
    let mut chat_msg = String::new();
    let mut chatter_name = String::new();

    /* Read data from stream into bufreader so we can deal with it more precisely */
    let conn = BufReader::new(stream.try_clone().unwrap());

    /*
    Get chat messages out of current tcp buffer. 
    According to the IRC spec, messages always end with CR-LF (carrige return, line feed)
    so splitting over newline will seperate it into each chat msg 
    */
    for line_res in conn.lines() {
        if let Ok(chat_info) = line_res {

            /* api checks if we're still here every once in a while, sending this notifies them that we're still here */
            if chat_info.starts_with("PING") {
                let _ = stream.write_all("PONG\r\n".as_bytes());
                continue;
            }

            /* Example chat_info (This is how chat messages look from the twitch api's pov):
                    ":monchenjiners!monchenjiners@monchenjiners.tmi.twitch.tv PRIVMSG #moonmoon :monkaGIGA PianoTime\r\n"
                      ^ user        ^ also user   ^ also user                          ^ channel ^^^^^^^^^^^^^^^^^^^^^^^ message
            */

            /* Parse chat msg for chatter name and their corresponding chat message */
            if chat_info.contains("PRIVMSG") {

                /* Parsing chatter name. First find ".tmi.twitch.tv" and then look at the text behind that until an "@" */
                if let Some(end_idx) = chat_info.find(".tmi.twitch.tv") {
                    let start_idx = chat_info.slice( .. end_idx).rfind("@").unwrap_or(0);
                    chatter_name = chat_info.slice(start_idx+1..end_idx).to_string();
                }

                /* Find channel name and if its found, our index is at that spot + the length of the channel string + 2 to get us at the start of the username */
                let mut chat_msg_begin_idx = 0;
                if let Some(idx) = chat_info.find(&CHANNEL) {
                    chat_msg_begin_idx = idx + CHANNEL.chars().count() + 2;
                }

                chat_msg = chat_info.slice(chat_msg_begin_idx ..).to_string();
                
                /* Print chatter name and chat msg. Used for debugging. */
                println!("|{}| {}", chatter_name.clone(), chat_msg);

            }
        }
        voting::update_votes(&chat_msg, chatter_name.clone());
        let _ = stream.flush();
        chat_msg = String::new();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    Err(std::io::Error::new(ErrorKind::Other, "End of stream"))
}

pub unsafe fn start_twitch_integration() {
    //voting::init_votes(&mut voting::VOTES.lock().unwrap());
    let CONFIG = config::CONFIG.clone().unwrap();

    /* Ideally i'd like to use connect_timeout, but using it creates soooo much UB for whatever reason */
    if let Ok(mut stream) = TcpStream::connect((SERVER, PORT)) {
        let mut channel = CONFIG.channel; channel.insert_str(0, "#");
        let token = CONFIG.oauth;
        /* Write relevant info to stream to "connect" to the chat */
        let _ = stream.write_all(format!("PASS {}\r\n", token).as_bytes());
        let _ = stream.write_all(format!("NICK bruh\r\n").as_bytes());
        let _ = stream.write_all(format!("JOIN {}\r\n", channel).as_bytes());
        let _ = stream.set_nonblocking(true);
        let _ = stream.set_ttl(16666666);
        let _ = stream.set_read_timeout(Some(std::time::Duration::new(0, 16666666))); // <- one sixty-th of a second (~~1 frame)

        println!("[Twitch Integration] Connected to {}'s chat", channel.replace("#", ""));

        /* Main loop to get and handle chat messages */
        std::thread::spawn(move ||{
            loop {
                /* Because the buffer in this func continually gets chat info, this func is itself an infinite loop. */
                if let Err(e) = get_chat_msg_info_loop(&mut stream, channel.clone()) {
                    println!("[Twitch Integration] Error: {}", e);
                    let _ = stream.flush();
                    let _ = stream.shutdown(std::net::Shutdown::Both);
                    if skyline_web::Dialog::yes_no("Failed to connect to twitch chat! Would you like to try reconnecting?") {
                        start_twitch_integration();
                    }
                    break;
                }
            }
        });
    }
    else {
        println!("[Twitch Integration] Failed to connect to twitch  :(");
        if skyline_web::Dialog::yes_no("Failed to connect to twitch chat! Would you like to try reconnecting?") {
            start_twitch_integration();
        }
    }
}