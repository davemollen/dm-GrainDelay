pub struct Slide {
  sample_rate: f64,
  z: f32,
}

impl Slide {
  pub fn new(sample_rate: f64) -> Self {
    Self { z: 0., sample_rate }
  }
  fn ms_to_samples(&mut self, milliseconds: f32) -> f32 {
    milliseconds / 1000. * (self.sample_rate as f32)
  }
  pub fn run(&mut self, x: f32, attack: f32, release: f32) -> f32 {
    let slide = if x > self.z {
      (x - self.z) * (1. / self.ms_to_samples(attack))
    } else {
      (x - self.z) * (1. / self.ms_to_samples(release))
    };
    let y = slide + self.z;
    self.z = y;
    y
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn slide_one() {
    let mut slide = Slide::new(1000.);
    assert_eq!(slide.run(1., 10., 100.), 0.1);
  }
  #[test]
  fn slide_two() {
    let mut slide = Slide::new(1000.);
    assert_eq!(slide.run(-1., 10., 100.), -0.01);
  }
  #[test]
  fn mstosamps() {
    let mut slide = Slide::new(44100.);
    assert_eq!(slide.ms_to_samples(2000.), 88200.);
  }
}
