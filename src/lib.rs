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
struct SearcResult {
    output: String,
}