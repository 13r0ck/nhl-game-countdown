use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// A simple wrapper so that our json is structured to Lametric's formatting.
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
