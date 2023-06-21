use std::{str::FromStr, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::GameHome;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    ascensionLevel: i32,
    playerClass: String,
    startSeed: i32,
    endSeed: i32,
    verbose: bool,
    exitAfterSearch: bool,
    highestFloor: i32,
    ironcladUnlocks: i32,
    silentUnlocks: i32,
    defectUnlocks: i32,
    watcherUnlocks: i32,
    firstBoss: i32,
    secondBoss: i32,
    thirdBoss: i32,
    eliteRoomWeight: f32,
    monsterRoomWeight: f32,
    restRoomWeight: f32,
    shopRoomWeight: f32,
    eventRoomWeight: f32,
    wingBootsThreshold: f32,
    relicsToBuy: Vec<String>,
    potionsToBuy: Vec<String>,
    cardsToBuy: Vec<String>,
    bossRelicsToTake: Vec<String>,
    forceNeowLament: bool,
    neowChoice: i32,
    useShovel: bool,
    speedrunPace: bool,
    act4: bool,
    alwaysSpawnBottledTornado: bool,
    alwaysSpawnBottledLightning: bool,
    alwaysSpawnBottledFlame: bool,
    ignorePandoraCards: bool,
    takeSerpentGold: bool,
    takeWarpedTongs: bool,
    takeBigFishRelic: bool,
    takeDeadAdventurerFight: bool,
    takeMausoleumRelic: bool,
    takeScrapOozeRelic: bool,
    takeAddictRelic: bool,
    takeMysteriousSphereFight: bool,
    takeRedMaskAct3: bool,
    takeMushroomFight: bool,
    takeMaskedBanditFight: bool,
    takeGoldenIdolWithoutCurse: bool,
    takeGoldenIdolWithCurse: bool,
    tradeGoldenIdolForBloody: bool,
    takeCursedTome: bool,
    tradeFaces: bool,
    takeMindBloomGold: bool,
    takeMindBloomFight: bool,
    takeMindBloomUpgrade: bool,
    tradeGoldenIdolForMoney: bool,
    takePortal: bool,
    numSensoryStoneCards: i32,
    takeWindingHallsCurse: bool,
    takeWindingHallsMadness: bool,
    takeColosseumFight: bool,
    takeDrugDealerRelic: bool,
    takeDrugDealerTransform: bool,
    takeLibraryCard: bool,
    takeWeMeetAgainRelic: bool,
    requiredAct1Cards: Vec<String>,
    bannedAct1Cards: Vec<String>,
    requiredAct1Relics: Vec<String>,
    requiredAct1Potions: Vec<String>,
    requiredRelics: Vec<String>,
    requiredPotions: Vec<String>,
    requiredEvents: Vec<String>,
    requiredCombats: Vec<String>,
    minimumElites: i32,
    maximumElites: i32,
    minimumCombats: i32,
    maximumCombats: i32,
    minimumRestSites: i32,
    showNeowOptions: bool,
    showCombats: bool,
    showBosses: bool,
    showBossRelics: bool,
    showRelics: bool,
    showShopRelics: bool,
    showShopCards: bool,
    showShopPotions: bool,
    showEvents: bool,
    showCardChoices: bool,
    showPotions: bool,
    showOtherCards: bool,
    showRawRelicPools: bool,
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl FromStr for SearchConfig {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl GameHome {
    pub fn search_config_path(&self) -> Option<PathBuf> {
        Some(
            self.home
            .join("searchConfig.json")
        )
        .filter(|path| path.exists())
    }

    pub fn search_config(&self) -> Option<Result<SearchConfig, Error>> {
        self
        .search_config_path()
        .map(std::fs::read_to_string)
        .map(|path| {
            path?
            .parse()
            .map_err(Into::into)
        })
    }
}

#[cfg(test)]
mod test_parse_search_config {
    use std::path::PathBuf;

    use crate::{_DEFAULT_HOME, GameHome};

    #[test]
    fn default_search_config_path() {
        let home = PathBuf::from(_DEFAULT_HOME);
        let game_home = GameHome::try_from(home).unwrap();
        let search_config = game_home.search_config_path().unwrap();
        dbg!(search_config);
    }

    #[test]
    fn default_search_config() {
        let home = PathBuf::from(_DEFAULT_HOME);
        let game_home = GameHome::try_from(home).unwrap();
        let search_config = game_home.search_config().unwrap().unwrap();
        dbg!(search_config);
    }
}
