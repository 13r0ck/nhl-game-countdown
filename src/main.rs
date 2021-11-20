#[macro_use]
extern crate rocket;
use chrono::{DateTime, Duration, Local, Utc};
use rocket::serde::json::Json;
mod types;
use crate::types::{LaMetricIndicator, NhlApi, Team};

#[get("/")]
fn index() -> &'static str {
    "nhl-game-countdown - Visit https://github.com/13r0ck/nhl-game-countdown for more information"
}

#[get("/?<team>")]
async fn api(team: &'_ str) -> Option<Json<LaMetricIndicator>> {
    let team = Team::new(team);
    let now_local = Local::now();
    let in_99_days = now_local + Duration::days(99);
    match reqwest::get(format!(
        "https://statsapi.web.nhl.com/api/v1/schedule?teamId={}&startDate={}&endDate={}",
        team,
        simple_date(now_local),
        simple_date(in_99_days)
    ))
    .await
    .unwrap()
    .json::<NhlApi>()
    .await
    {
        Ok(nhl_api) => {
            let now_utc = DateTime::<Utc>::from_utc(now_local.naive_utc(), Utc);
            if let Some((game_time, is_active)) = nhl_api.current_or_next_game(now_utc) {
                if is_active {
                    Some(Json(LaMetricIndicator::new(
                        "In Game".to_string(),
                        team.icon(),
                    )))
                } else {
                    Some(Json(LaMetricIndicator::new(
                        pretty_timer(now_utc.timestamp(), game_time),
                        team.icon(),
                    )))
                }
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn simple_date(now: DateTime<Local>) -> String {
    now.to_rfc3339()[..10].to_string()
}

fn pretty_timer(now: i64, later: i64) -> String {
    let dur = later - now;
    let days = dur / 86400;
    let hours = (dur - (days * 86400)) / 3600;
    let min = (dur - (days * 86400) - (hours * 3600)) / 60;
    format!("{}:{}:{}", pu(days), pu(hours), pu(min))
}

// pretty_unit
fn pu(num: i64) -> String {
    let mut num_string = num.to_string();
    if num.to_string().len() == 1 {
        num_string.insert(0, '0');
    }
    num_string
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![api])
}
