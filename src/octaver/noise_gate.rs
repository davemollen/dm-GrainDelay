use super::slide::Slide;

pub struct NoiseGate {
  slide: Slide,
}

impl NoiseGate {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      slide: Slide::new(sample_rate),
    }
  }
  pub fn run(&mut self, x: f32, threshold: f32, attack: f32, release: f32) -> f32 {
    let is_above_threshold = if x.abs() > threshold { 1. } else { 0. };
    let envelope = self.slide.run(is_above_threshold, attack, release);
    x * envelope
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn noise_gate_one() {
    let mut noise_gate = NoiseGate::new(1000.);
    assert_eq!(noise_gate.run(1., 0.5, 10., 100.), 0.1);
  }
  #[test]
  fn noise_gate_two() {
    let mut noise_gate = NoiseGate::new(1000.);
    assert_eq!(noise_gate.run(1., 0., 1., 0.), 1.);
    assert_eq!(noise_gate.run(1., 1., 10., 100.), 0.99);
  }
}
