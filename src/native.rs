pub type Instant = std::time::Instant;

#[cfg(feature = "now")]
pub static mut TIME: f64 = 0.0;

/// The current time, in milliseconds.
#[cfg(feature = "now")]
pub fn now() -> f64 {
   unsafe {TIME} 
}

