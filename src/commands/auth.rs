use crate::models::User;
use crate::utils::request;
use serde_json::json;

pub async fn register(user: User) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.findmatches.com/api/register";
    let response = request::send_post(url, json!({
        "username": user.username,
        "password": user.password,
        "email": user.email,
    })).await?;
    Ok(response)
}

pub async fn login(user: User) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.findmatches.com/api/login";
    let response = request::send_post(url, json!({
        "username": user.username,
        "password": user.password,
    })).await?;
    Ok(response)
}
