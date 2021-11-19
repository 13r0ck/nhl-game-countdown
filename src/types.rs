use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LaMetricIndicator {
    frames: Vec<Frame>,
}

impl LaMetricIndicator {
    pub fn new(text: String, icon: Option<usize>) -> LaMetricIndicator {
        LaMetricIndicator {
            frames: vec![Frame::new(text, icon)],
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
    fn new(text: String, _icon: Option<usize>) -> Frame {
        Frame { text, icon: None }
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
