use crate::{Game, GameCache};
use std::sync::OnceLock;

static CAROLINAHURRICANES: OnceLock<GameCache> = OnceLock::new();
static COLUMBUSBLUEJACKETS: OnceLock<GameCache> = OnceLock::new();
static NEWJERSEYDEVILS: OnceLock<GameCache> = OnceLock::new();
static NEWYORKISLANDERS: OnceLock<GameCache> = OnceLock::new();
static NEWYORKRANGERS: OnceLock<GameCache> = OnceLock::new();
static PHILADELPHIAFLYERS: OnceLock<GameCache> = OnceLock::new();
static PITTSBURGHPENGUINS: OnceLock<GameCache> = OnceLock::new();
static WASHINGTONCAPITALS: OnceLock<GameCache> = OnceLock::new();
static BOSTONBRUINS: OnceLock<GameCache> = OnceLock::new();
static BUFFALOSABRES: OnceLock<GameCache> = OnceLock::new();
static DETROITREDWINGS: OnceLock<GameCache> = OnceLock::new();
static FLORIDAPANTHERS: OnceLock<GameCache> = OnceLock::new();
static MONTREALCANADIENS: OnceLock<GameCache> = OnceLock::new();
static OTTAWASENATORS: OnceLock<GameCache> = OnceLock::new();
static TAMPABAYLIGHTNING: OnceLock<GameCache> = OnceLock::new();
static TORONTOMAPLELEAFS: OnceLock<GameCache> = OnceLock::new();
static ARIZONACOYOTES: OnceLock<GameCache> = OnceLock::new();
static CHICAGOBLACKHAWKS: OnceLock<GameCache> = OnceLock::new();
static COLORADOAVALANCHE: OnceLock<GameCache> = OnceLock::new();
static DALLASSTARS: OnceLock<GameCache> = OnceLock::new();
static MINNESOTAWILD: OnceLock<GameCache> = OnceLock::new();
static NASHVILLEPREDATORS: OnceLock<GameCache> = OnceLock::new();
static STLOUISBLUES: OnceLock<GameCache> = OnceLock::new();
static WINNIPEGJETS: OnceLock<GameCache> = OnceLock::new();
static ANAHEIMDUCKS: OnceLock<GameCache> = OnceLock::new();
static CALGARYFLAMES: OnceLock<GameCache> = OnceLock::new();
static EDMONTONOILERS: OnceLock<GameCache> = OnceLock::new();
static LOSANGELESKINGS: OnceLock<GameCache> = OnceLock::new();
static SANJOSESHARKS: OnceLock<GameCache> = OnceLock::new();
static SEATTLEKRAKEN: OnceLock<GameCache> = OnceLock::new();
static VANCOUVERCANUCKS: OnceLock<GameCache> = OnceLock::new();
static VEGASGOLDENKNIGHTS: OnceLock<GameCache> = OnceLock::new();

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

    async fn get_cache(&self, stat: &OnceLock<GameCache>) -> Option<Game> {
        if let Some(cache) = stat.get() {
            cache.next_game().await
        } else {
            let cache = GameCache::new(*self).await;
            let ret = cache.next_game().await;
            let _ = stat.set(cache);
            ret
        }
    }

    pub async fn next_game(&self) -> Option<Game> {
        match self {
            Team::CarolinaHurricanes => self.get_cache(&CAROLINAHURRICANES),
            Team::ColumbusBlueJackets => self.get_cache(&COLUMBUSBLUEJACKETS),
            Team::NewJerseyDevils => self.get_cache(&NEWJERSEYDEVILS),
            Team::NewYorkIslanders => self.get_cache(&NEWYORKISLANDERS),
            Team::NewYorkRangers => self.get_cache(&NEWYORKRANGERS),
            Team::PhiladelphiaFlyers => self.get_cache(&PHILADELPHIAFLYERS),
            Team::PittsburghPenguins => self.get_cache(&PITTSBURGHPENGUINS),
            Team::WashingtonCapitals => self.get_cache(&WASHINGTONCAPITALS),
            Team::BostonBruins => self.get_cache(&BOSTONBRUINS),
            Team::BuffaloSabres => self.get_cache(&BUFFALOSABRES),
            Team::DetroitRedWings => self.get_cache(&DETROITREDWINGS),
            Team::FloridaPanthers => self.get_cache(&FLORIDAPANTHERS),
            Team::MontrealCanadiens => self.get_cache(&MONTREALCANADIENS),
            Team::OttawaSenators => self.get_cache(&OTTAWASENATORS),
            Team::TampaBayLightning => self.get_cache(&TAMPABAYLIGHTNING),
            Team::TorontoMapleLeafs => self.get_cache(&TORONTOMAPLELEAFS),
            Team::ArizonaCoyotes => self.get_cache(&ARIZONACOYOTES),
            Team::ChicagoBlackhawks => self.get_cache(&CHICAGOBLACKHAWKS),
            Team::ColoradoAvalanche => self.get_cache(&COLORADOAVALANCHE),
            Team::DallasStars => self.get_cache(&DALLASSTARS),
            Team::MinnesotaWild => self.get_cache(&MINNESOTAWILD),
            Team::NashvillePredators => self.get_cache(&NASHVILLEPREDATORS),
            Team::StLouisBlues => self.get_cache(&STLOUISBLUES),
            Team::WinnipegJets => self.get_cache(&WINNIPEGJETS),
            Team::AnaheimDucks => self.get_cache(&ANAHEIMDUCKS),
            Team::CalgaryFlames => self.get_cache(&CALGARYFLAMES),
            Team::EdmontonOilers => self.get_cache(&EDMONTONOILERS),
            Team::LosAngelesKings => self.get_cache(&LOSANGELESKINGS),
            Team::SanJoseSharks => self.get_cache(&SANJOSESHARKS),
            Team::SeattleKraken => self.get_cache(&SEATTLEKRAKEN),
            Team::VancouverCanucks => self.get_cache(&VANCOUVERCANUCKS),
            Team::VegasGoldenKnights => self.get_cache(&VEGASGOLDENKNIGHTS),
        }
        .await
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Team::CarolinaHurricanes => "CAR",
                Team::ColumbusBlueJackets => "BBJ",
                Team::NewJerseyDevils => "NJD",
                Team::NewYorkIslanders => "NYI",
                Team::NewYorkRangers => "NYR",
                Team::PhiladelphiaFlyers => "PHI",
                Team::PittsburghPenguins => "PIT",
                Team::WashingtonCapitals => "WSH",
                Team::BostonBruins => "BOS",
                Team::BuffaloSabres => "BUF",
                Team::DetroitRedWings => "DET",
                Team::FloridaPanthers => "FLA",
                Team::MontrealCanadiens => "MTL",
                Team::OttawaSenators => "OTT",
                Team::TampaBayLightning => "TBL",
                Team::TorontoMapleLeafs => "TOR",
                Team::ArizonaCoyotes => "ARI",
                Team::ChicagoBlackhawks => "CHI",
                Team::ColoradoAvalanche => "COL",
                Team::DallasStars => "DAL",
                Team::MinnesotaWild => "MIN",
                Team::NashvillePredators => "NSH",
                Team::StLouisBlues => "STL",
                Team::WinnipegJets => "WPG",
                Team::AnaheimDucks => "ANA",
                Team::CalgaryFlames => "CGY",
                Team::EdmontonOilers => "EDM",
                Team::LosAngelesKings => "LAK",
                Team::SanJoseSharks => "SJS",
                Team::SeattleKraken => "SEA",
                Team::VancouverCanucks => "VAN",
                Team::VegasGoldenKnights => "VNK",
            }
        )
    }
}
