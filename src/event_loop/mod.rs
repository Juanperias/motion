use core::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct EventLoopConfig {
    pub fps: u32,
}

pub struct EventLoop {
    pub config: EventLoopConfig,
}

impl EventLoop {
    pub fn new(config: EventLoopConfig) -> Self {
        Self { config }
    }
    pub fn start<F, SF>(&self, code: F, sleep: SF)
    where
        F: Fn(EventLoopConfig),
        SF: Fn(Duration),
    {
        let dt = 1.0 / self.config.fps as f32;
        loop {
            code(self.config);
            sleep(Duration::from_secs_f32(dt));
        }
    }
}
