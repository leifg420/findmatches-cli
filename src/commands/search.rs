mod commands;

use crate::models::Profile;
use crate::utils::request;

pub async fn search_profiles(age_min: Option<u32>, age_max: Option<u32>, gender: Option<String>, preferences: Option<Vec<String>>) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
    let mut url = String::from("https://api.findmatches.com/api/search");
    let mut params = Vec::new();

    if let Some(age_min) = age_min {
        params.push(("age_min", age_min.to_string()));
    }
    if let Some(age_max) = age_max {
        params.push(("age_max", age_max.to_string()));
    }
    if let Some(gender) = gender {
        params.push(("gender", gender));
    }
    if let Some(preferences) = preferences {
        for preference in preferences {
            params.push((" preferences", preference));
        }
    }

    let response = request::send_get(&format!("{}?{}", url, serde_urlencoded::from_iter(params))).await?;
    Ok(serde_json::from_str(&response)?)
}
