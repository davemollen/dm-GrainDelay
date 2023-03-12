use std::f32::consts::PI;

pub trait Pan {
  fn pan(self, pan: f32) -> (f32, f32);
}

impl Pan for f32 {
  fn pan(self, pan: f32) -> (f32, f32) {
    let radians = (pan + 50.) * 0.005 * PI;
    (self * radians.cos(), self * radians.sin())
  }
}

#[cfg(test)]
mod tests {
  use super::Pan;
  use std::f32::consts::FRAC_1_SQRT_2;

  fn assert_approximately_eq(left: (f32, f32), right: (f32, f32)) {
    assert_eq!(
      (left.0 * 1000.).round() / 1000.,
      (right.0 * 1000.).round() / 1000.
    );
    assert_eq!(
      (left.1 * 1000.).round() / 1000.,
      (right.1 * 1000.).round() / 1000.
    );
  }

  #[test]
  fn mix() {
    assert_approximately_eq(1f32.pan(-50.), (1., 0.));
    assert_approximately_eq(1f32.pan(50.), (0., 1.));
    assert_approximately_eq(1f32.pan(0.), (FRAC_1_SQRT_2, FRAC_1_SQRT_2));
  }
}
