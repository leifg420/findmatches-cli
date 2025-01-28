use clap::Parser;
use crate::cli::App;
use crate::commands::{auth, search, profile, matches};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::parse();

    match app {
        App::Login { username, password } => {
            let user = User {
                username,
                password,
                email: String::new(),
            };
            let token = auth::login(user).await?;
            auth::save_auth(&token)?;
            println!("Successfully logged in!");
        }
        App::Register {
            username,
            password,
            email,
        } => {
            let user = User {
                username,
                password,
                email,
            };
            let response = auth::register(user).await?;
            println!("Registration successful: {}", response);
        }
        App::Search {
            age_min,
            age_max,
            gender,
            preferences,
        } => {
            let profiles = search::search_profiles(age_min, age_max, gender, preferences).await?;
            println!("Found {} profiles:", profiles.len());
            for p in profiles {
                println!("{} ({}, {})", p.name, p.age, p.gender);
            }
        }
        App::ViewProfile { user_id } => {
            let profile = profile::view_profile(user_id).await?;
            println!("Profile: {} ({})", profile.name, profile.age);
        }
        App::Match { action, user_id } => {
            // Implement match logic here
            println!("Matching action: {}", action);
        }
    }

    Ok(())
}
