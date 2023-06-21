use std::process::Command;

use crate::SearchResult;

mod parse;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Nom(String),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Nom(value.to_string())
    }
}

impl super::GameHome {
    pub fn search_output(&self) -> Result<String, Error> {
        let output = Command::new(&self.java_w)
        .current_dir(&self.home)
        .arg("-jar")
        .arg(&self.mod_the_spire)
        .arg("--skip-launcher")
        .arg("--mods")
        .arg("SeedSearch")
        .arg("|")
        .arg("tee")
        .output()
        .map(|output| output.stdout)?;
        
        Ok(String::from_utf8(output)?)
    }

    pub fn search_results(&self) -> Result<Vec<SearchResult>, Error> {
        let search = self.search_output()?;
        let (_, search_results) = parse::parse_search_results(&search)?;
        Ok(search_results)
    }
}

#[cfg(test)]
mod test_game_home {
    use std::path::PathBuf;

    use crate::{GameHome, _DEFAULT_HOME};

    #[test]
    fn test_search() {
        let home: GameHome = 
        PathBuf::from(_DEFAULT_HOME)
        .try_into().unwrap();

        let search = home.search_output().unwrap();
        for line in search.lines() {
            println!("{line}");
        }
    }
}