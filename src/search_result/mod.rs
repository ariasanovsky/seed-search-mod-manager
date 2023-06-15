use std::process::Command;

use crate::SearcResults;

impl super::GameHome {
    fn search(&self) -> Result<SearcResults, std::io::Error> {

        dbg!(&self);

        let output = Command::new(&self.java_w)
        .current_dir(&self.home)
        .arg("-jar")
        .arg(&self.mod_the_spire)
        .arg("--skip-launcher")
        .arg("--mods")
        .arg("SeedSearch")
        .arg("|")
        .arg("tee")
        .output()?;

        Ok(SearcResults {
            output: String::from_utf8_lossy(&output.stdout).into(),
        })
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

        let search = home.search().unwrap();
        for line in search.output.lines() {
            println!("{line}");
        }
    }
}