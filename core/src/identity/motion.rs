use crate::{AeonError, Result};
use libm::{sqrtf, log2f};
use blake3::Hasher;

pub type AccelSample = [f32; 3];

pub struct MotionVerifier {
    min_entropy: f32,
    max_clustering: f32,
}

impl MotionVerifier {
    pub fn new() -> Self {
        Self {
            min_entropy: 2.0,
            max_clustering: 0.8,
        }
    }

    pub fn verify(&self, samples: &[AccelSample]) -> Result<[u8; 32]> {
        if samples.len() < 10 { return Err(AeonError::InsufficientData); }

        let magnitudes = self.compute_magnitudes(samples);
        
        if self.compute_entropy(&magnitudes) < self.min_entropy {
            return Err(AeonError::InvalidInput);
        }

        if self.compute_clustering(&magnitudes) > self.max_clustering {
            return Err(AeonError::InvalidInput);
        }

        Ok(self.hash_trajectory(samples))
    }

    // FIX: Removed 'alloc' dependency, using standard Vec
    fn compute_magnitudes(&self, samples: &[AccelSample]) -> Vec<f32> {
        samples.iter()
            .map(|s| sqrtf(s[0]*s[0] + s[1]*s[1] + s[2]*s[2]))
            .collect()
    }

    fn compute_entropy(&self, mags: &[f32]) -> f32 {
        let mut histogram = [0u32; 10];
        let len = mags.len() as f32;
        
        for &m in mags {
            let idx = (m % 10.0) as usize;
            if idx < 10 { histogram[idx] += 1; }
        }

        let mut entropy = 0.0;
        for &count in &histogram {
            if count > 0 {
                let p = count as f32 / len;
                entropy -= p * log2f(p);
            }
        }
        entropy
    }

    fn compute_clustering(&self, mags: &[f32]) -> f32 {
        let mut sum_diff = 0.0;
        for i in 1..mags.len() {
            let diff = mags[i] - mags[i-1];
            sum_diff += if diff < 0.0 { -diff } else { diff };
        }
        1.0 / (1.0 + sum_diff)
    }

    fn hash_trajectory(&self, samples: &[AccelSample]) -> [u8; 32] {
        let mut hasher = Hasher::new();
        for s in samples {
            let qx = (s[0] * 10.0) as i32;
            let qy = (s[1] * 10.0) as i32;
            let qz = (s[2] * 10.0) as i32;
            hasher.update(&qx.to_le_bytes());
            hasher.update(&qy.to_le_bytes());
            hasher.update(&qz.to_le_bytes());
        }
        *hasher.finalize().as_bytes()
    }
}
