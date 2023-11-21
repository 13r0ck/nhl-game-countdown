use std::{fmt, sync::RwLock};

use crate::teams::Team;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use serde::{
    de::{Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Deserializer,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Season {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    start_time_utc: DateTime<Utc>,
    game_state: GameState,
}

impl Game {
    pub fn new(start_time_utc: DateTime<Utc>, game_state: GameState) -> Self {
        Game {
            start_time_utc,
            game_state,
        }
    }

    pub fn is_future(&self) -> bool {
        matches!(self.game_state, GameState::Future)
    }

    pub fn is_current(&self) -> bool {
        match self.game_state {
          GameState::Live => true,
          GameState::Pregame => true,
          _ => false,
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = match self.game_state {
            GameState::Future => {
                const DAYMINS: i64 = 60 * 24;
                const HOURMINS: i64 = 60;
                let mut remaining = (self.start_time_utc - Utc::now()).num_minutes();
                let days = remaining / DAYMINS;
                remaining -= days * DAYMINS;
                let hours = remaining / HOURMINS;
                remaining -= hours * HOURMINS;
                format!("{:02}:{:02}:{:02}", days, hours, remaining)
            }
            GameState::Live => String::from("Live"),
            GameState::Pregame => String::from("Pregame"),
            _ => String::from("ERROR"),
        };
        write!(f, "{}", stdout)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Final,
    Off,
    Future,
    Pregame,
    Live,
}

/// Caching logic should be as follows:
/// 1. If game is in the future, cache should invalidate at game start time
/// 2. If the game is current, refresh the cache every minute
/// 3. If there is no game, then we cache invalidate every 4 hours.
pub struct GameCache {
    cache: RwLock<GameCacheInner>,
}

struct GameCacheInner {
    team: Team,
    game: Option<Game>,
    invalidate_at: DateTime<Utc>,
}

impl GameCacheInner {
    pub async fn new(team: Team) -> Self {
        if let Some(game) = GameCacheInner::fetch_current_season(team)
            .await
            .ok()
            .and_then(|season| {
                season
                    .games
                    .into_iter()
                    .find(|game| game.is_future() || game.is_current())
            })
        {
            let invalidate_at = if game.is_future() {
                game.start_time_utc
            } else {
                Utc::now() + Duration::minutes(1)
            };
            GameCacheInner {
                team,
                game: Some(game),
                invalidate_at,
            }
        } else {
            GameCacheInner {
                team,
                game: None,
                invalidate_at: Utc::now() + Duration::hours(4),
            }
        }
    }

    async fn fetch_current_season(team: Team) -> Result<Season, reqwest::Error> {
        let req = reqwest::get(format!(
            "https://api-web.nhle.com/v1/club-schedule-season/{team}/now"
        ))
        .await?;
        req.json::<Season>().await
    }
}

impl GameCache {
    pub async fn new(team: Team) -> Self {
        GameCache {
            cache: RwLock::new(GameCacheInner::new(team).await),
        }
    }

    pub async fn next_game(&self) -> Option<Game> {
        let team = match self.cache.read() {
            Ok(cache) => {
                if Utc::now() < cache.invalidate_at {
                    return cache.game;
                }
                cache.team
            }
            Err(_) => return None,
        };

        let inner = GameCacheInner::new(team).await;
        if let Ok(mut cache) = self.cache.write() {
            *cache = inner;
            return cache.game;
        }

        None
    }
}

// Manual implementation of deserializeing JSON -> struct Game for speed and convenience.
impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            StartTimeUTC,
            GameState,
            Ignore,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`gameState` or `startTimeUtc`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: Error,
                    {
                        match value {
                            "gameState" => Ok(Field::GameState),
                            "startTimeUTC" => Ok(Field::StartTimeUTC),
                            _ => Ok(Field::Ignore),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct GameVisitor;

        impl<'de> Visitor<'de> for GameVisitor {
            type Value = Game;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct `Game`")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Game, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut start: Option<DateTime<FixedOffset>> = None;
                let mut state: Option<GameState> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::StartTimeUTC => {
                            let start_str: &str = map.next_value()?;
                            start =
                                Some(DateTime::parse_from_rfc3339(start_str).map_err(|_| {
                                    Error::custom("Time format of game did not parse correctly")
                                })?);
                        }
                        Field::GameState => {
                            state = Some(map.next_value::<GameState>()?);
                        }
                        Field::Ignore => {
                            let _ = map.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                let start = start.ok_or_else(|| Error::missing_field("startTimeUTC"))?;
                let state = state.ok_or_else(|| Error::missing_field("GameState"))?;
                Ok(Game::new(start.into(), state))
            }
        }

        const FIELDS: &[&str; 2] = &["startTimeUtc", "gameState"];
        deserializer.deserialize_struct("Game", FIELDS, GameVisitor)
    }
}

impl<'de> Deserialize<'de> for GameState {
    fn deserialize<D>(deserializer: D) -> Result<GameState, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GameStateVisitor;

        impl<'de> Visitor<'de> for GameStateVisitor {
            type Value = GameState;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid `GameState`")
            }

            fn visit_str<E>(self, value: &str) -> Result<GameState, E>
            where
                E: Error,
            {
                match value {
                    "FINAL" => Ok(GameState::Final),
                    "OFF" => Ok(GameState::Off),
                    "FUT" => Ok(GameState::Future),
                    "PRE" => Ok(GameState::Pregame),
                    "LIVE" => Ok(GameState::Live),
                    "CRIT" => Ok(GameState::Live),
                    _ => Ok(GameState::Future),
                }
            }
        }

        deserializer.deserialize_identifier(GameStateVisitor)
    }
}
