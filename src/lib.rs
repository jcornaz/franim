#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! A frame animation library for `no_std` game-development
//!
//!
//! # Example
//!
//! ```
//! # use franim::Animation;
//! # use core::time::Duration;
//! // Create an animation state
//! let mut animation = Animation::from_frame_duration(2, Duration::from_millis(100));
//! assert_eq!(animation.current_frame(), 0);
//!
//! // Update the state using the elapsed delta-time
//! animation.update(Duration::from_millis(50));
//! assert_eq!(animation.current_frame(), 0);
//! animation.update(Duration::from_millis(50));
//! assert_eq!(animation.current_frame(), 1);
//! animation.update(Duration::from_millis(110));
//! assert_eq!(animation.current_frame(), 0);
//! animation.update(Duration::from_millis(10));
//! assert_eq!(animation.current_frame(), 0);
//! ```
//! # Features
//!
//! * `std`: *(enabled by default)* enable use of the standard library. Must be disabled for `no_std` crates.

use core::time::Duration;

/// Animation state
#[deprecated(since = "0.1.1", note = "`Animation` is renamed to `AnimationState`")]
pub type Animation = AnimationState;

/// Animation state
#[derive(Debug, Clone)]
pub struct AnimationState {
    frame_count: usize,
    current_frame: usize,
    frame_duration: Duration,
    elapsed: Duration,
}

impl AnimationState {
    /// Creates a new animation from total number of frame and duration for each frame
    ///
    /// The `frame_duration` must be bigger than zero.
    ///
    /// # Panics
    ///
    /// Panics if `frame_duration` is zero
    #[must_use]
    pub fn from_frame_duration(frame_count: usize, frame_duration: Duration) -> Self {
        assert!(!frame_duration.is_zero(), "frame-duration must not be zero");
        Self {
            frame_count,
            current_frame: 0,
            frame_duration,
            elapsed: Duration::ZERO,
        }
    }

    /// Update the animation state with the elapsed `delta_time`
    pub fn update(&mut self, delta_time: Duration) {
        self.elapsed += delta_time;
        while self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;
            self.current_frame += 1;
        }
        self.current_frame %= self.frame_count;
    }

    /// Returns the current frame index
    #[must_use]
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
}
