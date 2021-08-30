pub struct Slide {
  sample_rate: f64,
  z: f32,
}

impl Slide {
  pub fn new(sample_rate: f64) -> Slide {
    Slide { z: 0., sample_rate }
  }
  fn ms_to_samples(&mut self, milliseconds: f32) -> f32 {
    1000. / milliseconds * (self.sample_rate as f32)
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
