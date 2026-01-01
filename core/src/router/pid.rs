use crate::{AeonError, Result};
use libm::{fmaxf, fminf};
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};

pub struct RouteWeightController {
    kp: f32, ki: f32, kd: f32,
    integral: f32, prev_error: f32,
    rng: ChaCha8Rng, base_damping: f32,
}

impl RouteWeightController {
    pub fn new(kp: f32, ki: f32, kd: f32, seed: u64) -> Self {
        Self {
            kp, ki, kd,
            integral: 0.0, prev_error: 0.0,
            rng: ChaCha8Rng::seed_from_u64(seed),
            base_damping: 1.0,
        }
    }
    pub fn update(&mut self, target: f32, current: f32, dt: f32) -> Result<f32> {
        if dt <= 0.0 { return Err(AeonError::InvalidInput); }
        let error = target - current;
        self.integral = fmaxf(-10.0, fminf(10.0, self.integral + error * dt));
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;
        
        // Frequency Hopping Logic
        let jitter = (self.rng.next_u32() % 20) as f32 / 100.0 - 0.1;
        let damping = self.base_damping * (1.0 + jitter);
        
        let output = self.kp * error + (self.ki * self.integral) / damping + self.kd * derivative;
        Ok(fmaxf(0.0, fminf(1.0, output)))
    }
}
