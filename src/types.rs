use chrono::{DateTime, FixedOffset, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy)]
pub enum Team {
    CarolinaHurricanes,
    ColumbusBlueJackets,
    NewJerseyDevils,
    NewYorkIslanders,
    NewYorkRangers,
    PhiladelphiaFlyers,
    PittsburghPenguins,
    WashingtonCapitals,
    BostonBruins,
    BuffaloSabres,
    DetroitRedWings,
    FloridaPanthers,
    MontrealCanadiens,
    OttawaSenators,
    TampaBayLightning,
    TorontoMapleLeafs,
    ArizonaCoyotes,
    ChicagoBlackhawks,
    #[default]
    ColoradoAvalanche,
    DallasStars,
    MinnesotaWild,
    NashvillePredators,
    StLouisBlues,
    WinnipegJets,
    AnaheimDucks,
    CalgaryFlames,
    EdmontonOilers,
    LosAngelesKings,
    SanJoseSharks,
    SeattleKraken,
    VancouverCanucks,
    VegasGoldenKnights,
}

impl Team {
    pub fn new(team: &'_ str) -> Team {
        match team {
            "Carolina Hurricanes" => Team::CarolinaHurricanes,
            "Columbus Blue Jackets" => Team::ColumbusBlueJackets,
            "New Jersey Devils" => Team::NewJerseyDevils,
            "New York Islanders" => Team::NewYorkIslanders,
            "New York Rangers" => Team::NewYorkRangers,
            "Philadelphia Flyers" => Team::PhiladelphiaFlyers,
            "Pittsburgh Penguins" => Team::PittsburghPenguins,
            "Washington Capitals" => Team::WashingtonCapitals,
            "Boston Bruins" => Team::BostonBruins,
            "Buffalo Sabres" => Team::BuffaloSabres,
            "Detroit Red Wings" => Team::DetroitRedWings,
            "Florida Panthers" => Team::FloridaPanthers,
            "Montréal Canadiens" => Team::MontrealCanadiens,
            "Ottawa Senators" => Team::OttawaSenators,
            "Tampa Bay Lightning" => Team::TampaBayLightning,
            "Toronto Maple Leafs" => Team::TorontoMapleLeafs,
            "Arizona Coyotes" => Team::ArizonaCoyotes,
            "Chicago Blackhawks" => Team::ChicagoBlackhawks,
            "Colorado Avalanche" => Team::ColoradoAvalanche,
            "Dallas Stars" => Team::DallasStars,
            "Minnesota Wild" => Team::MinnesotaWild,
            "Nashville Predators" => Team::NashvillePredators,
            "St. Louis Blues" => Team::StLouisBlues,
            "Winnipeg Jets" => Team::WinnipegJets,
            "Anaheim Ducks" => Team::AnaheimDucks,
            "Calgary Flames" => Team::CalgaryFlames,
            "Edmonton Oilers" => Team::EdmontonOilers,
            "Los Angeles Kings" => Team::LosAngelesKings,
            "San Jose Sharks" => Team::SanJoseSharks,
            "Seattle Kraken" => Team::SeattleKraken,
            "Vancouver Canucks" => Team::VancouverCanucks,
            "Vegas Golden Knights" => Team::VegasGoldenKnights,
            _ => Team::default(),
        }
    }

    pub fn icon(&self) -> usize {
        match self {
            Team::CarolinaHurricanes => 47582,
            Team::ColumbusBlueJackets => 47583,
            Team::NewJerseyDevils => 47584,
            Team::NewYorkIslanders => 47585,
            Team::NewYorkRangers => 47586,
            Team::PhiladelphiaFlyers => 47587,
            Team::PittsburghPenguins => 47588,
            Team::WashingtonCapitals => 47589,
            Team::BostonBruins => 47590,
            Team::BuffaloSabres => 47591,
            Team::DetroitRedWings => 47592,
            Team::FloridaPanthers => 47593,
            Team::MontrealCanadiens => 47594,
            Team::OttawaSenators => 47595,
            Team::TampaBayLightning => 47596,
            Team::TorontoMapleLeafs => 47597,
            Team::ArizonaCoyotes => 47598,
            Team::ChicagoBlackhawks => 47599,
            Team::ColoradoAvalanche => 47478,
            Team::DallasStars => 47600,
            Team::MinnesotaWild => 47601,
            Team::NashvillePredators => 47602,
            Team::StLouisBlues => 47603,
            Team::WinnipegJets => 47604,
            Team::AnaheimDucks => 47605,
            Team::CalgaryFlames => 47606,
            Team::EdmontonOilers => 47607,
            Team::LosAngelesKings => 47608,
            Team::SanJoseSharks => 47609,
            Team::SeattleKraken => 47610,
            Team::VancouverCanucks => 47611,
            Team::VegasGoldenKnights => 47612,
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Team::CarolinaHurricanes => "12",
                Team::ColumbusBlueJackets => "29",
                Team::NewJerseyDevils => "1",
                Team::NewYorkIslanders => "2",
                Team::NewYorkRangers => "3",
                Team::PhiladelphiaFlyers => "4",
                Team::PittsburghPenguins => "5",
                Team::WashingtonCapitals => "15",
                Team::BostonBruins => "6",
                Team::BuffaloSabres => "7",
                Team::DetroitRedWings => "17",
                Team::FloridaPanthers => "13",
                Team::MontrealCanadiens => "8",
                Team::OttawaSenators => "9",
                Team::TampaBayLightning => "14",
                Team::TorontoMapleLeafs => "10",
                Team::ArizonaCoyotes => "53",
                Team::ChicagoBlackhawks => "16",
                Team::ColoradoAvalanche => "21",
                Team::DallasStars => "25",
                Team::MinnesotaWild => "30",
                Team::NashvillePredators => "18",
                Team::StLouisBlues => "19",
                Team::WinnipegJets => "52",
                Team::AnaheimDucks => "24",
                Team::CalgaryFlames => "20",
                Team::EdmontonOilers => "22",
                Team::LosAngelesKings => "26",
                Team::SanJoseSharks => "28",
                Team::SeattleKraken => "55",
                Team::VancouverCanucks => "23",
                Team::VegasGoldenKnights => "54",
            }
        )
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub enum Offset {
    Minus1200,
    Minus1100,
    Minus1000,
    Minus0930,
    Minus0900,
    Minus0800,
    #[default]
    Minus0700,
    Minus0600,
    Minus0500,
    Minus0400,
    Minus0300,
    Minus0330,
    Minus0200,
    Minus0100,
    Utc,
    Plus0100,
    Plus0200,
    Plus0300,
    Plus0330,
    Plus0400,
    Plus0430,
    Plus0500,
    Plus0530,
    Plus0545,
    Plus0600,
    Plus0700,
    Plus0800,
    Plus0900,
    Plus0930,
    Plus1000,
    Plus1030,
    Plus1100,
    Plus1200,
    Plus1245,
    Plus1300,
    Plus1400,
}

impl Offset {
    pub fn new(offset: &str) -> Self {
        match offset {
            "UTC-1200" => Offset::Minus1200,
            "UTC-1100" => Offset::Minus1100,
            "UTC-1000" => Offset::Minus1000,
            "UTC-0930" => Offset::Minus0930,
            "UTC-0900" => Offset::Minus0900,
            "UTC-0800" => Offset::Minus0800,
            "UTC-0700" => Offset::Minus0700,
            "UTC-0600" => Offset::Minus0600,
            "UTC-0500" => Offset::Minus0500,
            "UTC-0400" => Offset::Minus0400,
            "UTC-0300" => Offset::Minus0300,
            "UTC-0330" => Offset::Minus0330,
            "UTC-0200" => Offset::Minus0200,
            "UTC-0100" => Offset::Minus0100,
            "UTC-0000" => Offset::Utc,
            "UTC+0100" => Offset::Plus0100,
            "UTC+0200" => Offset::Plus0200,
            "UTC+0300" => Offset::Plus0300,
            "UTC+0330" => Offset::Plus0330,
            "UTC+0400" => Offset::Plus0400,
            "UTC+0430" => Offset::Plus0430,
            "UTC+0500" => Offset::Plus0500,
            "UTC+0530" => Offset::Plus0530,
            "UTC+0545" => Offset::Plus0545,
            "UTC+0600" => Offset::Plus0600,
            "UTC+0700" => Offset::Plus0700,
            "UTC+0800" => Offset::Plus0800,
            "UTC+0900" => Offset::Plus0900,
            "UTC+0930" => Offset::Plus0930,
            "UTC+1000" => Offset::Plus1000,
            "UTC+1030" => Offset::Plus1030,
            "UTC+1100" => Offset::Plus1100,
            "UTC+1200" => Offset::Plus1200,
            "UTC+1245" => Offset::Plus1245,
            "UTC+1300" => Offset::Plus1300,
            "UTC+1400" => Offset::Plus1400,
            _ => Offset::default(),
        }
    }
}

impl From<Offset> for FixedOffset {
    fn from(offset: Offset) -> FixedOffset {
        const HOURS: i32 = 3600;
        const MINS: i32 = 60;
        const EAST: fn(i32) -> Option<FixedOffset> = FixedOffset::east_opt;
        const WEST: fn(i32) -> Option<FixedOffset> = FixedOffset::west_opt;
        match offset {
            Offset::Minus1200 => WEST(HOURS * 12),
            Offset::Minus1100 => WEST(HOURS * 11),
            Offset::Minus1000 => WEST(HOURS * 10),
            Offset::Minus0930 => WEST(HOURS * 9 + MINS * 30),
            Offset::Minus0900 => WEST(HOURS * 9),
            Offset::Minus0800 => WEST(HOURS * 8),
            Offset::Minus0700 => WEST(HOURS * 7),
            Offset::Minus0600 => WEST(HOURS * 6),
            Offset::Minus0500 => WEST(HOURS * 5),
            Offset::Minus0400 => WEST(HOURS * 4),
            Offset::Minus0300 => WEST(HOURS * 3),
            Offset::Minus0330 => WEST(HOURS * 3 + MINS * 30),
            Offset::Minus0200 => WEST(HOURS * 2),
            Offset::Minus0100 => WEST(HOURS),
            Offset::Utc => WEST(0),
            Offset::Plus0100 => EAST(HOURS),
            Offset::Plus0200 => EAST(HOURS * 2),
            Offset::Plus0300 => EAST(HOURS * 3),
            Offset::Plus0330 => EAST(HOURS * 3 + MINS * 30),
            Offset::Plus0400 => EAST(HOURS * 4),
            Offset::Plus0430 => EAST(HOURS * 4 + MINS * 30),
            Offset::Plus0500 => EAST(HOURS * 5),
            Offset::Plus0530 => EAST(HOURS * 5 + MINS * 30),
            Offset::Plus0545 => EAST(HOURS * 5 + MINS * 45),
            Offset::Plus0600 => EAST(HOURS * 6),
            Offset::Plus0700 => EAST(HOURS * 7),
            Offset::Plus0800 => EAST(HOURS * 8),
            Offset::Plus0900 => EAST(HOURS * 9),
            Offset::Plus0930 => EAST(HOURS * 9 + MINS * 30),
            Offset::Plus1000 => EAST(HOURS * 10),
            Offset::Plus1030 => EAST(HOURS * 10 + MINS * 30),
            Offset::Plus1100 => EAST(HOURS * 11),
            Offset::Plus1200 => EAST(HOURS * 12),
            Offset::Plus1245 => EAST(HOURS * 12 + MINS * 45),
            Offset::Plus1300 => EAST(HOURS * 13),
            Offset::Plus1400 => EAST(HOURS * 14),
        }
        .unwrap()
    }
}

impl chrono::offset::Offset for Offset {
    fn fix(&self) -> FixedOffset {
        (*self).into()
    }
}

pub enum GameState {
    Scheduled,
    PreGame,
    InProgress,
}

impl GameState {
    pub fn new(state: &'_ str) -> GameState {
        match state {
            "Scheduled" => GameState::Scheduled,
            "In Progress" => GameState::InProgress,
            "Pre-Game" => GameState::PreGame,
            _ => GameState::Scheduled,
        }
    }

    pub fn is_active(&self) -> bool {
        !matches!(self, GameState::Scheduled)
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameState::InProgress => "In Game",
                GameState::PreGame => "PreGame",
                GameState::Scheduled => "Scheduled",
            }
        )
    }
}

impl From<&'_ str> for GameState {
    fn from(state: &'_ str) -> Self {
        match state {
            "Scheduled" => GameState::Scheduled,
            "In Progress" => GameState::InProgress,
            "Pre-Game" => GameState::PreGame,
            _ => GameState::Scheduled,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LaMetricIndicator {
    frames: Vec<Frame>,
}

impl LaMetricIndicator {
    pub fn new(text: String, icon: usize) -> LaMetricIndicator {
        LaMetricIndicator {
            frames: vec![Frame::new(text, Some(icon))],
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Frame {
    text: String,
    icon: Option<usize>,
}

impl Frame {
    fn new(text: String, icon: Option<usize>) -> Frame {
        Frame { text, icon }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NhlApi {
    pub dates: Vec<Date>,
}

impl NhlApi {
    pub fn current_or_next_game(self, now: DateTime<Utc>) -> Option<(i64, GameState)> {
        if let Some(t) = self
            .dates
            .into_iter()
            // take nhl api data structure and convert it into a iter of tuple (date, is_active)
            .flat_map(|date| {
                date.games
                    .into_iter()
                    .map(|game| (game.gameDate, GameState::new(&game.status.detailedState)))
            })
            .find(|g| {
                if let Ok(dt) = DateTime::parse_from_rfc3339(g.0.as_str()) {
                    g.1.is_active() || dt > now
                } else {
                    false
                }
            })
        {
            Some((
                DateTime::parse_from_rfc3339(t.0.as_str())
                    .unwrap()
                    .timestamp(),
                t.1,
            ))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct Game {
    pub gameDate: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[allow(non_snake_case)]
pub struct Status {
    pub detailedState: String,
}
