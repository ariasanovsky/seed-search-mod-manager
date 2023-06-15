use std::path::PathBuf;

const _DEFAULT_HOME: &str = "C:/Program Files (x86)/Steam/steamapps/common/SlayTheSpire/";

mod game_home;
mod search_config;
mod search_result;

#[derive(Debug)]
struct GameHome {
    home: PathBuf,
    java_w: PathBuf,
    mod_the_spire: PathBuf,
}

#[derive(Debug)]
enum Error {
    InvalidHome,
    InvalidJavaW,
    InvalidModTheSpire,
}

#[derive(Debug)]
struct SearcResults {
    output: String,
}

#[derive(Debug, PartialEq)]
struct SearchResult {
    seed_string: String,
    seed: i64,
    neow_options: Vec<String>,
    bosses: Vec<String>,
    true_map_path: Vec<String>,
    events: Vec<String>,
    card_choices: Vec<Vec<(usize, String)>>,
    potions: Vec<Vec<(usize, String)>>,
    commons: Vec<String>,
    uncommons: Vec<String>,
    rares: Vec<String>,
    boss_relics: Vec<String>,
    shops: Vec<String>,
}
