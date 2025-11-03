//! Buffered random number generator for improved performance.
//!
//! This implementation reduces syscall overhead by maintaining a buffer filled
//! from the system's cryptographically secure random number generator (CSRNG).
//! Small requests are served from the buffer, while large requests bypass it.
//!
//! Performance characteristics:
//! - Small requests (< buffer size): Fast, served from buffer without syscalls
//! - Large requests (>= buffer size): Direct syscall, maintaining security
//! - Thread-safe: Uses RefCell for interior mutability within single-threaded V8 isolate
//!
//! This optimization significantly improves crypto.getRandomValues() performance
//! for typical use cases (small to medium arrays) while maintaining cryptographic security.

use ring::rand::{SecureRandom, SystemRandom};

/// Buffer size for random data caching (8KB)
/// This size balances memory usage and syscall reduction.
/// Larger buffers reduce syscalls but use more memory.
const BUFFER_SIZE: usize = 8192;

pub struct BufferedRandom {
    /// System random number generator (CSRNG)
    system_random: SystemRandom,
    /// Buffer for caching random data
    buffer: [u8; BUFFER_SIZE],
    /// Current position in the buffer (how many bytes have been consumed)
    position: usize,
    /// Number of valid bytes in the buffer
    available: usize,
}

impl BufferedRandom {
    /// Creates a new buffered random number generator
    pub fn new() -> Self {
        Self {
            system_random: SystemRandom::new(),
            buffer: [0u8; BUFFER_SIZE],
            position: 0,
            available: 0,
        }
    }

    /// Fills the provided slice with cryptographically secure random bytes.
    ///
    /// For small requests (< buffer size), data is served from the internal buffer.
    /// For large requests (>= buffer size), data is fetched directly from the system RNG.
    ///
    /// # Arguments
    /// * `dest` - Destination slice to fill with random bytes
    ///
    /// # Returns
    /// * `Ok(())` on success
    /// * `Err(ring::error::Unspecified)` if the system RNG fails
    #[inline]
    pub fn fill(&mut self, dest: &mut [u8]) -> Result<(), ring::error::Unspecified> {
        let requested = dest.len();

        // For large requests, bypass the buffer and use system RNG directly
        if requested >= BUFFER_SIZE {
            return self.system_random.fill(dest);
        }

        let mut offset = 0;

        while offset < requested {
            // If buffer is exhausted, refill it
            if self.position >= self.available {
                self.refill_buffer()?;
            }

            // Copy from buffer to destination
            let remaining_in_buffer = self.available - self.position;
            let remaining_to_copy = requested - offset;
            let to_copy = remaining_in_buffer.min(remaining_to_copy);

            dest[offset..offset + to_copy]
                .copy_from_slice(&self.buffer[self.position..self.position + to_copy]);

            self.position += to_copy;
            offset += to_copy;
        }

        Ok(())
    }

    /// Refills the internal buffer from the system RNG
    #[inline]
    fn refill_buffer(&mut self) -> Result<(), ring::error::Unspecified> {
        self.system_random.fill(&mut self.buffer)?;
        self.position = 0;
        self.available = BUFFER_SIZE;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_fill() {
        let mut rng = BufferedRandom::new();
        let mut data = [0u8; 16];
        assert!(rng.fill(&mut data).is_ok());
        // Just check that we got some data (not all zeros)
        assert!(data.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_large_fill() {
        let mut rng = BufferedRandom::new();
        let mut data = vec![0u8; 16384];
        assert!(rng.fill(&mut data).is_ok());
        // Check that we got some data
        assert!(data.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_multiple_fills() {
        let mut rng = BufferedRandom::new();
        for _ in 0..100 {
            let mut data = [0u8; 64];
            assert!(rng.fill(&mut data).is_ok());
        }
    }

    #[test]
    fn test_exact_buffer_size() {
        let mut rng = BufferedRandom::new();
        let mut data = vec![0u8; BUFFER_SIZE];
        assert!(rng.fill(&mut data).is_ok());
    }

    #[test]
    fn test_larger_than_buffer() {
        let mut rng = BufferedRandom::new();
        let mut data = vec![0u8; BUFFER_SIZE + 1];
        assert!(rng.fill(&mut data).is_ok());
    }
}
