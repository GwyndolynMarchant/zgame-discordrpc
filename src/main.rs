use discord_rpc_client::Client;
use fancy_regex::Regex;
use std::{env, thread, time};
use window_titles::{Connection, ConnectionTrait};
use std::collections::HashMap;

// Returns the window title; for gzdoom this is "level - game", for lzdoom this is just "game"
fn info(connection: &Connection) -> Result<String, Box<dyn std::error::Error>> {
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    let re = Regex::new("((?!(.*Firefox.*)|(.*Pale Moon.*))(DOOM)|(Project Brutality)|(Duke))").unwrap();
    let window: String = windows.into_iter().filter(|s| re.is_match(s).unwrap()).collect();
    Ok(window)
}

fn main() {
    // Obtain engine from first arg since it would be near impossible to find correct one from the code
    let engine = env::args()
        .nth(1)
        .expect("Requires an engine to be input (gzdoom/raze/lzdoom)");
    // Create the client
    let mut drpc = Client::new(968250311758192650);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Create connection so that window list may be obtained
    let connection = Connection::new().unwrap();

    let supported_engines = HashMap::from([
        ("gzdoom", "GZDoom"),
        ("lzdoom", "LZDoom"),
        ("raze", "Raze")
    ]);

    let eng_str = &engine.as_str().clone();

    if !supported_engines.contains_key(eng_str) {
         println!("Unsupported engine!");
    }

    let s_engine = supported_engines[eng_str];

    loop {
        let window = info(&connection).unwrap();
        let game_vec: Vec<&str> = window.split(" - ").collect();

        let mut level = "Menus";

        if game_vec.len() > 1 {
            level = game_vec[0];
        }

        let game = game_vec[game_vec.len() - 1];
        let status_game = ["Game: ".to_string(), game.to_string()].concat();

        println!("{}\n{}\nLevel: {}\n--------------------", s_engine, status_game, level);

        let mut icon = "lz";

        if *eng_str == "gzdoom" {
            // Get the icon
            icon = match game {
                "DOOM Registered" => "doom",
                "DOOM 2: Hell on Earth" => "doom2",
                "DOOM 2: Unity Edition" => "doom2",
                "The Ultimate DOOM" => "doom",
                _ => "gz"
            };
        } else if *eng_str == "raze" {
            icon = match game {
                "Duke Nukem 3D" => "duke3d",
                "Duke Nukem 3D: Atomic Edition" => "duke3d",
                "BLOOD" => "blood",
                "BLOOD: Unknown Version" => "blood",
                _ => "rz"
            };
        }

        // Set the activity
        if let Err(why) = drpc.set_activity(|a| {
            a.details(game.to_string())
                .state(level)
                .assets(|ass| ass.large_image(icon).large_text(s_engine.to_string()))
        }) {
            println!("Failed to set presence: {}", why);
        }

        // Loop every 15 seconds
        thread::sleep(time::Duration::from_secs(15));
        println!("program has looped\n------------------");
    }
}
