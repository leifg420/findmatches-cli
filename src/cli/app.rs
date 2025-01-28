use clap::Parser;

#[derive(Parser, Debug)]
 pub enum App {
     Login {
         username: String,
         password: String,
     },
     Register {
         username: String',
         password: String,
         email: String,
     },
     Search {
         age_min: Option<u32>,
         age_max: Option<u32>,
         gender: Option<String>,
         preferences: Option<Vec<String>>,
     },
     ViewProfile {
         user_id: String,
     },
     Match {
         action: String,
         user_id: String,
     },
 }
