use serde::{Deserialize};
use base64;

#[derive(Deserialize, Debug)]
pub struct SummaryCategory {
    pub digital: String,
    pub hours: u32,
    pub minutes: u32,
    pub name: String,
    pub percent: f32,
    pub text: String,
    pub total_seconds: f32,
}

#[derive(Deserialize, Debug)]
pub struct SummaryEditor {
    pub digital: String,
    pub hours: u32,
    pub minutes: u32,
    pub name: String,
    pub percent: f32,
    pub text: String,
    pub total_seconds: f32,
}

#[derive(Deserialize, Debug)]
pub struct SummaryLanguage {
    pub digital: String,
    pub hours: u32,
    pub minutes: u32,
    pub name: String,
    pub percent: f32,
    pub text: String,
    pub total_seconds: f32,
}

#[derive(Deserialize, Debug)]
pub struct SummaryData {
    pub categories: Vec<SummaryCategory>,
    pub editors: Vec<SummaryEditor>,
    pub languages: Vec<SummaryLanguage>,
    pub human_readable_total: String,
    pub human_readable_daily_average: String,
}

#[derive(Deserialize, Debug)]
pub struct Summaries {
    pub data: SummaryData
}

pub struct WakaTime {
    api_key: String
}

impl WakaTime {
    pub fn new() -> Self {
        let api_key = std::env::var("WAKATIME_API_KEY")
            .expect("The WakaTime API key is not set in the environment variable \"WAKATIME_API_KEY\".");

        WakaTime {
            api_key
        }
    }

    pub async fn fetch_summaries(&self) -> Summaries {
        let token = base64::encode(self.api_key.as_bytes()).to_string();
        let request_url = format!("https://wakatime.com/api/v1/users/current/stats/last_7_days");

        reqwest::Client::new()
            .get(&request_url)
            .header("Authorization", format!("Basic {}", token).to_string())
            .send()
            .await
            .unwrap()
            .json::<Summaries>()
            .await
            .unwrap()
    }
}
