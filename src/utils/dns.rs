use std::net::{IpAddr, TcpStream};
use std::time::Duration;

pub struct DnsLoadBalancer {
    endpoints: Vec<String>,
    current_endpoint: usize,
    timeout: Duration,
}

impl DnsLoadBalancer {
    pub fn new(endpoints: Vec<String>, timeout: Duration) -> Self {
        DnsLoadBalancer {
            endpoints,
            current_endpoint: 0,
            timeout,
        }
    }

    pub async fn get_connection(&mut self) -> Result<TcpStream, String> {
        loop {
            if self.current_endpoint >= self.endpoints.len() {
                self.current_endpoint = 0;
            }

            let endpoint = &self.endpoints[self.current_endpoint];
            self.current_endpoint += 1;

            match TcpStream::connect_timeout(endpoint, self.timeout).await {
                Ok(stream) => return Ok(stream),
                Err(e) => {
                    // Log connection attempt
                    println!("Failed to connect to {}: {}", endpoint, e);
                    continue;
                }
            }
        }
    }
}
