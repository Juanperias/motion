use core::time::Duration;

/// Configuration for the event loop, specifying the frames per second (fps).
#[derive(Debug, Clone, Copy)]
pub struct EventLoopConfig {
    /// Frames per second for the event loop.
    pub fps: u32,
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
    /// let config = EventLoopConfig { fps: 60 };
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
    /// let config = EventLoopConfig { fps: 60 };
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
}
