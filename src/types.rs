use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LaMetricIndicator {
    frames: Vec<Frame>,
}

impl LaMetricIndicator {
    pub fn new(text: String, icon: u32) -> LaMetricIndicator {
        LaMetricIndicator {
            frames: vec![Frame::new(text, icon)],
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Frame {
    text: String,
    icon: Option<u32>,
}

impl Frame {
    fn new(text: String, icon: u32) -> Frame {
        Frame {
            text,
            icon: Some(icon),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NhlApi {
    pub dates: Vec<Date>,
}

impl NhlApi {
    pub fn next_game_time(self, now: DateTime<Utc>) -> Option<i64> {
        if let Some(t) = self
            .dates
            .into_iter()
            .map(|date| date.games.into_iter().map(|game| game.gameDate))
            .flatten()
            .find(|time| {
                if let Ok(dt) = DateTime::parse_from_rfc3339(time.as_str()) {
                    dt > now
                } else {
                    false
                }
            })
        {
            Some(
                DateTime::parse_from_rfc3339(t.as_str())
                    .unwrap()
                    .timestamp(),
            )
        } else {
            None
        }
    }

    pub fn any_active_games(&self) -> bool {
        self.dates.iter().any(|date| {
            date.games
                .iter()
                .any(|game| game.status.detailedState == "InGame")
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Date {
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub gameDate: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Status {
    pub detailedState: String,
}
