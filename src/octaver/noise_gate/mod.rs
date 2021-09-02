mod slide;
use self::slide::Slide;

pub struct NoiseGate {
  slide: Slide,
}

impl NoiseGate {
  pub fn new(sample_rate: f64) -> NoiseGate {
    NoiseGate {
      slide: Slide::new(sample_rate),
    }
  }
  pub fn run(&mut self, x: f32, threshold: f32) -> f32 {
    let is_above_threshold = if x.abs() > threshold { 1. } else { 0. };
    let envelope = self.slide.run(is_above_threshold, 2., 120.);
    x * envelope
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn noise_gate() {
    fn gate(x: f32, threshold: f32) -> f32 {
      let is_above_threshold = if x.abs() > threshold { 1. } else { 0. };
      x * is_above_threshold
    }
    assert_eq!(gate(0.2, 0.1), 0.2);
    assert_eq!(gate(-0.5, 0.8), 0.);
    assert_eq!(gate(0.5, 0.8), 0.);
  }
}
