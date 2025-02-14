pub struct DcBlock {
  coeff: f32,
  xm1: f32,
  ym1: f32,
}

impl DcBlock {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      coeff: 1. - (220.5 / sample_rate),
      xm1: 0.,
      ym1: 0.,
    }
  }

  pub fn process(&mut self, x: f32) -> f32 {
    let y = x - self.xm1 + self.coeff * self.ym1;
    self.xm1 = x;
    self.ym1 = y;
    y
  }
}
