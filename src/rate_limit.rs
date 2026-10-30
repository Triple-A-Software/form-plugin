use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

use axum::http::HeaderMap;

// shortcut: in-memory and per plugin process, so limits reset on restart; move to valkey if the
// plugin ever runs as more than one process
pub struct SubmissionRateLimiter {
    window: Duration,
    max_submissions: usize,
    max_bytes: u64,
    entries: Mutex<HashMap<IpAddr, Vec<(Instant, u64)>>>,
}

impl SubmissionRateLimiter {
    pub fn new(window: Duration, max_submissions: usize, max_bytes: u64) -> Self {
        Self {
            window,
            max_submissions,
            max_bytes,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Records a submission attempt of `bytes` for `ip` and returns whether it is within the limits.
    /// Rejected attempts are not recorded, so a blocked client gets through again once the window
    /// has moved on.
    pub fn check(&self, ip: IpAddr, bytes: u64, now: Instant) -> bool {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|_, attempts| {
            attempts.retain(|(at, _)| now.duration_since(*at) < self.window);
            !attempts.is_empty()
        });
        let attempts = entries.entry(ip).or_default();
        let used_bytes: u64 = attempts.iter().map(|(_, b)| b).sum();
        if attempts.len() >= self.max_submissions || used_bytes + bytes > self.max_bytes {
            return false;
        }
        attempts.push((now, bytes));
        true
    }
}

/// The client ip as forwarded by the neleto gateway (rightmost `X-Forwarded-For` entry, then
/// `X-Real-IP`), mirroring neleto's own `ClientIp` extractor. The plugin itself is only ever
/// reached through the neleto backend, so the socket peer is always localhost.
pub fn client_ip(headers: &HeaderMap) -> Option<IpAddr> {
    if let Some(ip) = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').map(str::trim).rfind(|e| !e.is_empty()))
        .and_then(|v| v.parse().ok())
    {
        return Some(ip);
    }
    headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.trim().parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_submissions_and_bytes_per_ip() {
        let limiter = SubmissionRateLimiter::new(Duration::from_secs(60), 2, 100);
        let a: IpAddr = "1.1.1.1".parse().unwrap();
        let b: IpAddr = "2.2.2.2".parse().unwrap();
        let t0 = Instant::now();
        assert!(limiter.check(a, 10, t0));
        assert!(limiter.check(a, 10, t0));
        assert!(!limiter.check(a, 10, t0), "third submission in window");
        assert!(limiter.check(b, 100, t0), "other ip is independent");
        assert!(!limiter.check(b, 1, t0), "byte budget used up");
        assert!(
            limiter.check(a, 10, t0 + Duration::from_secs(61)),
            "window moved on"
        );
    }

    #[test]
    fn reads_forwarded_ip() {
        let mut headers = HeaderMap::new();
        assert_eq!(client_ip(&headers), None);
        headers.insert("x-real-ip", "9.9.9.9".parse().unwrap());
        assert_eq!(client_ip(&headers), Some("9.9.9.9".parse().unwrap()));
        headers.insert("x-forwarded-for", "6.6.6.6, 5.5.5.5".parse().unwrap());
        assert_eq!(client_ip(&headers), Some("5.5.5.5".parse().unwrap()));
    }
}
