use std::path::{PathBuf, Path};

use crate::{GameHome, Error};

impl TryFrom<PathBuf> for GameHome {
    type Error = Error;

    fn try_from(home: PathBuf) -> Result<Self, Self::Error> {
        if !home.is_dir() {
            return Err(Error::InvalidHome);
        }

        let javaw = &home
        .join("jre/bin/javaw.exe");

        if !javaw.is_file() {
            return Err(Error::InvalidJavaW);
        }

        let mod_the_spire = home
        .parent()
        .and_then(Path::parent)
        .ok_or(Error::InvalidModTheSpire)?
        .join("workshop/content/646570/1605060445/ModTheSpire.jar");

        if !mod_the_spire.is_file() {
            return Err(Error::InvalidModTheSpire);
        }
        Ok(GameHome {
            home: home.clone(),
            java_w: javaw.into(),
            mod_the_spire: mod_the_spire,
        })
    }
}

#[cfg(test)]
mod test_try_into_game_home {
    use std::path::PathBuf;

    use crate::{_DEFAULT_HOME, GameHome};

    #[test]
    fn test_try_into_game_home() {
        let home = PathBuf::from(_DEFAULT_HOME);
        let game_home = GameHome::try_from(home);
        assert!(game_home.is_ok());
    }
}