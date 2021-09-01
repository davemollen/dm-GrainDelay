#[path = "slide.rs"]
mod slide;
use self::slide::Slide;

pub struct NoiseGate {
  slide: self::slide::Slide,
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
