pub struct DcBlock {
  sample_rate: f32,
  xm1: f32,
  ym1: f32,
}

impl DcBlock {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      xm1: 0.,
      ym1: 0.,
    }
  }

  pub fn process(&mut self, x: f32) -> f32 {
    let coeff = 1. - (220.5 / self.sample_rate);
    let y = x - self.xm1 + coeff * self.ym1;
    self.xm1 = x;
    self.ym1 = y;
    y
  }
}
