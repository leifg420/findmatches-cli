use serde_json::Value;

pub struct Config {
    pub api_base_url: String,
    pub max_connections: usize,
    pub rate_limit: RateLimitConfig,
}

pub struct RateLimitConfig {
    pub global_capacity: usize,
    pub global_refill_rate: usize,
    pub global_refill_interval: Duration,
}

impl Config {
    pub fn new(config_file: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(config_file)?;
        let config: Value = serde_json::from_str(&content)?;
        
        Ok(Config {
            api_base_url: config["api_base_url"].as_str().unwrap_or("https://api.findmatches.com/api").to_string(),
            max_connections: config["max_connections"].as_u64().unwrap_or(10) as usize,
            rate_limit: RateLimitConfig {
                global_capacity: config["rate_limit"]["global_capacity"].as_u64().unwrap_or(60) as usize,
                global_refill_rate: config["rate_limit"]["global_refill_rate"].as_u64().unwrap_or(60) as usize,
                global_refill_interval: Duration::from_secs(config["rate_limit"]["global_refill_interval"].as_u64().unwrap_or(60)),
            },
        })
    }
}
