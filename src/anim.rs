/// Easing function type
#[derive(Clone, Copy, Default)]
pub enum Easing {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Easing {
    fn apply(&self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

/// Smoothly interpolates a value toward a target over time
pub struct Tween {
    current: f32,
    target: f32,
    /// Linear speed per frame (before easing)
    speed: f32,
    easing: Easing,
    /// Internal progress 0.0–1.0 through current transition
    progress: f32,
    start_value: f32,
}

impl Tween {
    pub fn new(initial: f32, speed: f32) -> Self {
        Self {
            current: initial,
            target: initial,
            speed,
            easing: Easing::Linear,
            progress: 1.0,
            start_value: initial,
        }
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Sets a new target. Resets internal progress for eased transitions.
    pub fn set_target(&mut self, target: f32) {
        if (target - self.target).abs() > f32::EPSILON {
            self.target = target;
            self.start_value = self.current;
            self.progress = 0.0;
        }
    }

    /// Advances the animation by one frame. Call once per frame.
    pub fn update(&mut self) {
        if self.progress >= 1.0 {
            self.current = self.target;
            return;
        }

        self.progress = (self.progress + self.speed).min(1.0);
        let eased = self.easing.apply(self.progress);
        self.current = self.start_value + (self.target - self.start_value) * eased;
    }

    /// Returns the current interpolated value
    pub fn value(&self) -> f32 {
        self.current
    }

    /// Returns true if the tween has reached its target
    pub fn done(&self) -> bool {
        self.progress >= 1.0
    }

    /// Immediately jumps to the target value
    pub fn snap(&mut self) {
        self.current = self.target;
        self.progress = 1.0;
    }
}
