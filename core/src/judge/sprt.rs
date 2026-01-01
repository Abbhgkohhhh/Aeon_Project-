use crate::{AeonError, Result};
use libm::{logf, fmaxf, fminf};

#[derive(Debug, Clone, Copy)]
pub struct BatchStats { pub sent: u32, pub received: u32 }
#[derive(Debug, PartialEq)]
pub enum SlashVerdict { Slash, Trust, Continue }

pub struct RelayReputation {
    llr: f32, upper: f32, lower: f32,
}

impl RelayReputation {
    pub fn new(alpha: f32, beta: f32) -> Self {
        Self {
            llr: 0.0,
            upper: logf((1.0 - beta) / alpha),
            lower: logf(beta / (1.0 - alpha)),
        }
    }
    pub fn evaluate(&mut self, batch: BatchStats, baseline: f32) -> Result<SlashVerdict> {
        if batch.sent == 0 { return Err(AeonError::InvalidInput); }
        let p0 = fmaxf(0.01, baseline);
        let p1 = fminf(0.99, baseline * 1.5);
        
        let failures = (batch.sent - batch.received) as f32;
        let successes = batch.received as f32;
        
        self.llr += failures * logf(p1/p0) + successes * logf((1.0-p1)/(1.0-p0));
        
        if self.llr >= self.upper { Ok(SlashVerdict::Slash) }
        else if self.llr <= self.lower { Ok(SlashVerdict::Trust) }
        else { Ok(SlashVerdict::Continue) }
    }
}
