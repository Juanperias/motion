use core::time::Duration;

/// Configuration for the event loop, specifying the frames per second (fps).
#[derive(Debug, Clone, Copy)]
pub struct EventLoopConfig {
    /// Frames per second for the event loop.
    pub fps: u32,
    /// Delta time calculated from the fps.
    pub delta_time: f32,
}

/// The main structure for the event loop, holding its configuration.
pub struct EventLoop {
    /// Configuration for the event loop.
    pub config: EventLoopConfig,
}

impl EventLoop {
    /// Creates a new event loop with the specified configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = EventLoopConfig { fps: 60, delta_time: 1.0 / 60.0 };
    /// let event_loop = EventLoop::new(config);
    /// ```
    ///
    /// # Parameters
    ///
    /// - `config`: The configuration for the event loop.
    ///
    /// # Returns
    ///
    /// A new `EventLoop` instance.
    #[must_use]
    pub fn new(config: EventLoopConfig) -> Self {
        Self { config }
    }

    /// Starts the event loop, running the provided code at the specified fps, and sleeping in between.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = EventLoopConfig { fps: 60, delta_time: 1.0 / 60.0 };
    /// let event_loop = EventLoop::new(config);
    ///
    /// event_loop.start(|config| {
    ///     // Your code here
    /// }, |duration| {
    ///     // Sleep for the duration
    ///     std::thread::sleep(duration);
    /// });
    /// ```
    ///
    /// # Parameters
    ///
    /// - `code`: A closure that takes `EventLoopConfig` and contains the code to run each frame.
    /// - `sleep`: A closure that takes `Duration` and handles sleeping between frames.
    pub fn start<F, SF>(&self, code: F, sleep: SF)
    where
        F: Fn(EventLoopConfig),
        SF: Fn(Duration),
    {
        let dt = 1 / self.config.fps;
        loop {
            code(self.config);
            sleep(Duration::from_secs(u64::from(dt)));
        }
    }

    /// Starts the event loop with a mutable closure, allowing modification of the loop configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = EventLoopConfig { fps: 60, delta_time: 1.0 / 60.0 };
    /// let event_loop = EventLoop::new(config);
    ///
    /// event_loop.start_mut(|config| {
    ///     // Your code here
    /// }, |duration| {
    ///     // Sleep for the duration
    ///     std::thread::sleep(duration);
    /// });
    /// ```
    ///
    /// # Parameters
    ///
    /// - `code`: A mutable closure that takes `EventLoopConfig` and contains the code to run each frame.
    /// - `sleep`: A closure that takes `Duration` and handles sleeping between frames.
    pub fn start_mut<F, SF>(&self, mut code: F, sleep: SF)
    where
        F: FnMut(EventLoopConfig),
        SF: Fn(Duration),
    {
        let dt = 1 / self.config.fps;
        loop {
            code(self.config);
            sleep(Duration::from_secs(u64::from(dt)));
        }
    }
}

/// Builder pattern for constructing an `EventLoop`.
#[derive(Debug)]
pub struct EventLoopBuilder {
    config: EventLoopConfig,
}

impl EventLoopBuilder {
    /// Creates a new `EventLoopBuilder` with default configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let builder = EventLoopBuilder::new();
    /// ```
    pub fn new() -> Self {
        EventLoopBuilder {
            config: EventLoopConfig {
                fps: 60,
                delta_time: 1.0 / 60.0,
            },
        }
    }

    /// Sets the frames per second (fps) for the event loop.
    ///
    /// # Parameters
    ///
    /// - `fps`: The frames per second to set.
    ///
    /// # Returns
    ///
    /// The updated `EventLoopBuilder` instance.
    pub fn fps(mut self, fps: u32) -> Self {
        self.config = EventLoopConfig {
            fps,
            delta_time: 1.0 / fps as f32,
        };
        self
    }

    /// Builds the `EventLoop` with the specified configuration.
    ///
    /// # Returns
    ///
    /// A new `EventLoop` instance.
    pub fn build(&self) -> EventLoop {
        EventLoop {
            config: self.config,
        }
    }
}

impl Default for EventLoopBuilder {
    fn default() -> Self {
        EventLoopBuilder {
            config: EventLoopConfig {
                fps: 60,
                delta_time: 1.0 / 60.0,
            },
        }
    }
}
