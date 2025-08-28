#[derive(Debug, Clone, poise::ChoiceParameter)]
pub enum Game {
    #[name = "Final Fantasy"]
    FF,
    #[name = "Kingdom Hearts"]
    KH,
}

impl Game {
    pub fn folder_name(&self) -> &'static str {
        match self {
            Game::FF => "Final Fantasy/XIV",
            Game::KH => "Kingdom Hearts",
        }
    }
}

#[derive(Debug, Clone, poise::ChoiceParameter)]
pub enum FinalFantasyExpansion {
    #[name = "XIV Online (1.0)"] XIV,
    #[name = "A Realm Reborn (ARR)"] ARR,
    #[name = "Heavensward (HW)"] HW,
    #[name = "Stormblood (SB)"] SB,
    #[name = "Shadowbringers (ShB)"] ShB,
    #[name = "Endwalker (EW)"] EW,
    #[name = "Dawntrail (DT)"] DT,
}

impl FinalFantasyExpansion {
    pub fn folder_name(&self) -> &'static str {
        match self {
            Self::XIV => "XIV",
            Self::ARR => "A Realm Reborn",
            Self::HW => "Heavensward",
            Self::SB => "Stormblood",
            Self::ShB => "Shadowbringers",
            Self::EW => "Endwalker",
            Self::DT => "Dawntrail",
        }
    }
}

#[derive(Debug, Clone, poise::ChoiceParameter)]
pub enum KingdomHeartsTitle {
    #[name = "I"] I,
    #[name = "II"] II,
    #[name = "III"] III,
    #[name = "Dream Drop Distance"] DDD,
    #[name = "Birth by Sleep + 358-2 Days"] BBSDAYS,
}

impl KingdomHeartsTitle {
    pub fn folder_name(&self) -> &'static str {
        match self {
            Self::I => "Kingdom Hearts",
            Self::II => "Kingdom Hearts II",
            Self::III => "Kingdom Hearts III, II.8, Unchained χ & Union χ [Cross]",
            Self::DDD => "Dream Drop Distance",
            Self::BBSDAYS => "Birth by Sleep & 358-2 Days",
        }
    }
}
