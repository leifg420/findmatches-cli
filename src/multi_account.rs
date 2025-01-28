use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AccountContext {
    pub auth_token: String,
    pub user_id: String,
    pub session_id: String,
    pub rate_limiter: RateLimiter,
}

#[derive(Debug, Clone)]
pub struct ClientSession {
    pub accounts: HashMap<String, AccountContext>, // Key: username
    pub queue: RequestQueue,
    pub active_account: Option<String>,           // Active username
    pub api_base_url: String,
    pub rate_limiter: AccountRateLimiter,
}

impl ClientSession {
    pub fn new(api_base_url: String) -> Self {
        ClientSession {
            accounts: HashMap::new(),
            queue: RequestQueue::new(),
            active_account: None,
            api_base_url,
            rate_limiter: AccountRateLimiter::new(
                60, // Global rate limit capacity
                60,
                Duration::from_secs(60),
            ),
        }
    }

    pub fn set_active_account(&mut self, username: String) -> Result<(), String> {
        if self.accounts.contains_key(&username) {
            self.active_account = Some(username);
            Ok(())
        } else {
            Err(format!("Account '{}' not found.", username))
        }
    }

    pub fn get_active_account(&self) -> Option<&AccountContext> {
        self.active_account.as_ref()
            .and_then(|username| self.accounts.get(username))
    }

    pub fn add_account(&mut self, account: AccountContext, username: String) -> Result<(), String> {
        if self.accounts.insert(username.clone(), account).is_some() {
            Ok(())
        } else {
            Err(format!("Account '{}' already exists.", username))
        }
    }

    pub async fn process_requests(&mut self) {
        while let Some(request) = self.queue.get_next_request() {
            if let Some(active_account) = &self.active_account {
                if self.rate_limiter.try_consume(active_account) {
                    // Process the request
                    println!("Processing request for account '{}': {}", active_account, request);
                } else {
                    // Rate limit exceeded; re-queue the request
                    self.queue.add_request(request, QoSLevel::Low);
                }
            } else {
                // No active account; skip request
            }
        }
    }
}
