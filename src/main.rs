#[macro_use]
extern crate rocket;
use chrono::{DateTime, Duration, Local, Utc};
use rocket::serde::{json::Json};
mod types;
use crate::types::{LaMetricIndicator, NhlApi};

#[get("/?<id>&<icon>")]
async fn index(id: u32, icon: u32) -> Option<Json<LaMetricIndicator>> {
    let now_local = Local::now();
    let in_99_days = now_local + Duration::days(99);
    //Ok(resp) => Some(Json(LaMetricIndicator::new(resp.next_game(utc_time), icon))),
    match reqwest::get(format!(
        "https://statsapi.web.nhl.com/api/v1/schedule?teamid={}&startDate={}&endDate={}",
        id,
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
                    Some(Json(LaMetricIndicator::new("Go Avs Go".to_string(), icon)))
                } else {
                    Some(Json(LaMetricIndicator::new(
                        pretty_timer(now_utc.timestamp(), game_time),
                        icon,
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
    rocket::build().mount("/", routes![index])
}
