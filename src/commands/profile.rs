mod commands;

use crate::models::Profile;
use crate::utils::request;

pub async fn view_profile(user_id: String) -> Result<Profile, Box<dyn std::error::Error>> {
    let url = format!("https://api.findmatches.com/api/profile/{}", user_id);
    let response = request::send_get(&url).await?;
    Ok(serde_json::from_str(&response)?)
}
