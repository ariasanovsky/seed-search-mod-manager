use std::path::PathBuf;

const _DEFAULT_HOME: &str = "C:/Program Files (x86)/Steam/steamapps/common/SlayTheSpire/";

mod game_home;
mod search_config;
mod search_result;
mod verification_config;

#[derive(Debug)]
pub struct GameHome {
    home: PathBuf,
    java_w: PathBuf,
    mod_the_spire: PathBuf,
}

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    seed_string: String,
    seed: String,
    neow_options: Vec<String>,
    combats: Vec<String>,
    bosses: Vec<String>,
    events: Vec<String>,
    true_map_path: Vec<String>,
    card_choices: Vec<(String, Vec<String>)>,
    potions: Vec<(String, Vec<String>)>,
    common_relics: Vec<String>,
    uncommon_relics: Vec<String>,
    rare_relics: Vec<String>,
    boss_relics: Vec<String>,
    shop_relics: Vec<String>,
    leftover: String,
}
