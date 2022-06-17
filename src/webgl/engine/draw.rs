use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DrawInfo {
    pub frametime: Duration,
    pub timestamp: f64,
}