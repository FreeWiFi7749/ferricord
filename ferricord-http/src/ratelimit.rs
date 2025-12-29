//! Rate limiting implementation for Discord API
//!
//! Discord uses a combination of global and per-route rate limits.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{debug, warn};

/// Information about a rate limit.
#[derive(Clone, Debug)]
pub struct RateLimitInfo {
    /// Number of requests remaining in the current window.
    pub remaining: u32,
    /// Total number of requests allowed in the window.
    pub limit: u32,
    /// When the rate limit resets.
    pub reset_at: Instant,
    /// The bucket this rate limit belongs to.
    pub bucket: Option<String>,
}

impl RateLimitInfo {
    /// Create a new rate limit info with default values.
    pub fn new() -> Self {
        Self {
            remaining: 1,
            limit: 1,
            reset_at: Instant::now(),
            bucket: None,
        }
    }

    /// Check if we should wait before making a request.
    pub fn should_wait(&self) -> bool {
        self.remaining == 0 && Instant::now() < self.reset_at
    }

    /// Get the duration to wait before the rate limit resets.
    pub fn wait_duration(&self) -> Duration {
        let now = Instant::now();
        self.reset_at.saturating_duration_since(now)
    }
}

impl Default for RateLimitInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// A bucket for tracking rate limits on a specific route.
#[derive(Debug)]
struct RouteBucket {
    /// Rate limit info for this bucket.
    info: RateLimitInfo,
    /// Semaphore for limiting concurrent requests to this bucket.
    #[allow(dead_code)]
    semaphore: Arc<Semaphore>,
}

impl RouteBucket {
    fn new() -> Self {
        Self {
            info: RateLimitInfo::new(),
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }
}

/// Global rate limiter for Discord API requests.
#[derive(Debug)]
pub struct RateLimiter {
    /// Global rate limit state.
    global: RwLock<GlobalState>,
    /// Per-route buckets.
    buckets: RwLock<HashMap<String, RouteBucket>>,
    /// Mapping from route to bucket hash.
    route_to_bucket: RwLock<HashMap<String, String>>,
}

#[derive(Debug)]
struct GlobalState {
    /// Whether we're currently globally rate limited.
    limited: bool,
    /// When the global rate limit resets.
    reset_at: Instant,
    /// Remaining requests in the global limit.
    #[allow(dead_code)]
    remaining: u32,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            limited: false,
            reset_at: Instant::now(),
            remaining: 50,
        }
    }
}

impl RateLimiter {
    /// Create a new rate limiter.
    pub fn new() -> Self {
        Self {
            global: RwLock::new(GlobalState::default()),
            buckets: RwLock::new(HashMap::new()),
            route_to_bucket: RwLock::new(HashMap::new()),
        }
    }

    /// Check if we're globally rate limited and wait if necessary.
    pub async fn check_global(&self) {
        let wait_duration = {
            let global = self.global.read();
            if global.limited && Instant::now() < global.reset_at {
                Some(global.reset_at - Instant::now())
            } else {
                None
            }
        };

        if let Some(duration) = wait_duration {
            warn!("Global rate limit hit, waiting {:?}", duration);
            tokio::time::sleep(duration).await;
        }
    }

    /// Acquire permission to make a request to a specific route.
    /// 
    /// This method pessimistically decrements the remaining counter before making
    /// a request to provide preemptive rate limiting. The actual remaining value
    /// will be updated from response headers.
    pub async fn acquire(&self, route: &str) -> RateLimitGuard {
        self.check_global().await;

        let bucket_key = self.get_or_create_bucket(route);

        // Use write lock to check and decrement remaining atomically
        let wait_duration = {
            let mut buckets = self.buckets.write();
            if let Some(bucket) = buckets.get_mut(&bucket_key) {
                if bucket.info.should_wait() {
                    Some(bucket.info.wait_duration())
                } else {
                    // Pessimistically decrement remaining before making request
                    bucket.info.remaining = bucket.info.remaining.saturating_sub(1);
                    None
                }
            } else {
                None
            }
        };

        if let Some(duration) = wait_duration {
            debug!("Route {} rate limited, waiting {:?}", route, duration);
            tokio::time::sleep(duration).await;
        }

        RateLimitGuard {
            route: route.to_string(),
            bucket_key,
        }
    }

    /// Get or create a bucket for a route.
    fn get_or_create_bucket(&self, route: &str) -> String {
        // First check with read lock
        {
            let route_to_bucket = self.route_to_bucket.read();
            if let Some(bucket_key) = route_to_bucket.get(route) {
                return bucket_key.clone();
            }
        }

        // Acquire write lock and check again to avoid TOCTOU race
        let mut route_to_bucket = self.route_to_bucket.write();
        if let Some(bucket_key) = route_to_bucket.get(route) {
            return bucket_key.clone();
        }

        let bucket_key = route.to_string();

        let mut buckets = self.buckets.write();
        buckets
            .entry(bucket_key.clone())
            .or_insert_with(RouteBucket::new);

        route_to_bucket.insert(route.to_string(), bucket_key.clone());

        bucket_key
    }

    /// Update rate limit info from response headers.
    pub fn update_from_headers(
        &self,
        route: &str,
        remaining: Option<u32>,
        limit: Option<u32>,
        reset_after: Option<f64>,
        bucket: Option<String>,
        global: bool,
    ) {
        if global {
            let mut global_state = self.global.write();
            global_state.limited = true;
            if let Some(reset_after) = reset_after {
                global_state.reset_at = Instant::now() + Duration::from_secs_f64(reset_after);
            }
            return;
        }

        let bucket_key = if let Some(ref bucket_hash) = bucket {
            let mut route_to_bucket = self.route_to_bucket.write();
            route_to_bucket.insert(route.to_string(), bucket_hash.clone());
            bucket_hash.clone()
        } else {
            self.get_or_create_bucket(route)
        };

        let mut buckets = self.buckets.write();
        let bucket_entry = buckets.entry(bucket_key).or_insert_with(RouteBucket::new);

        if let Some(remaining) = remaining {
            bucket_entry.info.remaining = remaining;
        }
        if let Some(limit) = limit {
            bucket_entry.info.limit = limit;
        }
        if let Some(reset_after) = reset_after {
            bucket_entry.info.reset_at = Instant::now() + Duration::from_secs_f64(reset_after);
        }
        bucket_entry.info.bucket = bucket;
    }

    /// Handle a 429 rate limit response.
    pub async fn handle_rate_limit(&self, route: &str, retry_after: f64, global: bool) {
        let duration = Duration::from_secs_f64(retry_after);

        if global {
            warn!("Global rate limit hit, retry after {:?}", duration);
            let mut global_state = self.global.write();
            global_state.limited = true;
            global_state.reset_at = Instant::now() + duration;
        } else {
            warn!("Route {} rate limited, retry after {:?}", route, duration);
            let bucket_key = self.get_or_create_bucket(route);
            let mut buckets = self.buckets.write();
            if let Some(bucket) = buckets.get_mut(&bucket_key) {
                bucket.info.remaining = 0;
                bucket.info.reset_at = Instant::now() + duration;
            }
        }

        tokio::time::sleep(duration).await;
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Guard returned when acquiring a rate limit slot.
#[derive(Debug)]
pub struct RateLimitGuard {
    route: String,
    bucket_key: String,
}

impl RateLimitGuard {
    /// Get the route this guard is for.
    pub fn route(&self) -> &str {
        &self.route
    }

    /// Get the bucket key this guard is for.
    pub fn bucket_key(&self) -> &str {
        &self.bucket_key
    }
}
