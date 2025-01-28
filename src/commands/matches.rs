use crate::models::Match;
use crate::utils::request;

pub async fn get_matches() -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    let url = "https://api.findmatches.com/api/matches";
    let response = request::send_get(&url).await?;
    Ok(serde_json::from_str(&response)?)
}
