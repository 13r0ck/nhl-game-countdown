#[macro_use]
extern crate rocket;
use chrono::{DateTime, FixedOffset, Local};
use rocket::serde::json::Json;

mod games;
mod lametric;
mod offset_utc;
mod teams;

pub use games::{Game, GameCache};
use lametric::LaMetricIndicator;
use offset_utc::OffsetUTC;
pub use teams::Team;

#[get("/", rank = 0)]
fn index() -> &'static str {
    "nhl-game-countdown - Visit https://github.com/13r0ck/nhl-game-countdown for more information"
}

#[get("/?<team>&<utc_offset>&<format>")]
async fn api(team: &'_ str, utc_offset: &'_ str, format: &'_ str) -> Json<LaMetricIndicator> {
    let team = Team::new(team);
    if let Some(next_game) = team.next_game().await {
        Json(LaMetricIndicator::new(next_game.to_string(), team.icon()))
    } else {
        let fixed: FixedOffset = OffsetUTC::new(utc_offset).into();
        let user_time = Local::now().with_timezone(&fixed);
        let use_24_hour = format == "true" || format == "1";
        error(user_time, team, use_24_hour)
    }
}

fn error(time: DateTime<FixedOffset>, team: Team, use_24_hour: bool) -> Json<LaMetricIndicator> {
    Json(LaMetricIndicator::new(
        time.format(if use_24_hour {"%k:%M"} else {"%l:%M"}).to_string(),
        team.icon(),
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![api])
        .mount("/", routes![index])
}
