use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

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

impl Default for Team {
    fn default() -> Team {
        Team::ColoradoAvalanche
    }
}

impl From<&'_ str> for Team {
    fn from(team: &'_ str) -> Team {
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
            _ => Team::default()
        }
    }
}

impl Team {
    pub fn icon(&self) -> usize {
        match self {
            Team::CarolinaHurricanes => 555,
            Team::ColumbusBlueJackets => 555,
            Team::NewJerseyDevils => 555,
            Team::NewYorkIslanders => 555,
            Team::NewYorkRangers => 555,
            Team::PhiladelphiaFlyers => 555,
            Team::PittsburghPenguins => 555,
            Team::WashingtonCapitals => 555,
            Team::BostonBruins => 555,
            Team::BuffaloSabres => 555,
            Team::DetroitRedWings => 555,
            Team::FloridaPanthers => 555,
            Team::MontrealCanadiens => 555,
            Team::OttawaSenators => 555,
            Team::TampaBayLightning => 555,
            Team::TorontoMapleLeafs => 555,
            Team::ArizonaCoyotes => 555,
            Team::ChicagoBlackhawks => 555,
            Team::ColoradoAvalanche => 21,
            Team::DallasStars => 555,
            Team::MinnesotaWild => 555,
            Team::NashvillePredators => 555,
            Team::StLouisBlues => 555,
            Team::WinnipegJets => 555,
            Team::AnaheimDucks => 555,
            Team::CalgaryFlames => 555,
            Team::EdmontonOilers => 555,
            Team::LosAngelesKings => 555,
            Team::SanJoseSharks => 555,
            Team::SeattleKraken => 555,
            Team::VancouverCanucks => 555,
            Team::VegasGoldenKnights => 555,
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
    pub fn current_or_next_game(self, now: DateTime<Utc>) -> Option<(i64, bool)> {
        if let Some(t) = self
            .dates
            .into_iter()
            // take nhl api data structure and convert it into a iter of tuple (date, is_active)
            .map(|date| {
                date.games
                    .into_iter()
                    .map(|game| (game.gameDate, (game.status.detailedState == "In Progress")))
            })
            .flatten()
            .find(|g| {
                if let Ok(dt) = DateTime::parse_from_rfc3339(g.0.as_str()) {
                    g.1 || dt > now
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
