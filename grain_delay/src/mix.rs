use crate::shared::float_ext::FloatExt;
use std::f32::consts::FRAC_PI_2;

pub struct Mix {
  mix: f32,
  dry_gain: f32,
  wet_gain: f32,
}

impl Mix {
  pub fn new() -> Self {
    Self {
      mix: 0.,
      dry_gain: 1.,
      wet_gain: 0.,
    }
  }

  pub fn process(&mut self, dry: f32, wet: (f32, f32), mix: f32) -> (f32, f32) {
    if mix != self.mix {
      let factor = mix * FRAC_PI_2;
      self.mix = mix;
      self.dry_gain = factor.fast_cos();
      self.wet_gain = factor.fast_sin();
    }
    let dry_out = dry * self.dry_gain;
    (
      dry_out + wet.0 * self.wet_gain,
      dry_out + wet.1 * self.wet_gain,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::Mix;

  fn assert_approximately_eq(left: f32, right: f32, digits: usize) {
    let tol = 10f32.powi(-(digits as i32));
    let diff = (left - right).abs();
    assert!(
      diff <= tol,
      "Values are not approximately equal: left={left}, right={right}, diff={diff}, tol={tol}"
    );
  }

  #[test]
  fn mix() {
    let mut mix = Mix::new();
    let first = mix.process(0., (1., 1.), 0.);
    let second = mix.process(0., (1., 1.), 0.5);
    let third = mix.process(0., (1., 1.), 1.);
    assert_approximately_eq(first.0, 0., 3);
    assert_approximately_eq(first.1, 0., 3);
    assert_approximately_eq(second.0, 0.707, 3);
    assert_approximately_eq(second.1, 0.707, 3);
    assert_approximately_eq(third.0, 1., 3);
    assert_approximately_eq(third.1, 1., 3);
  }
}
