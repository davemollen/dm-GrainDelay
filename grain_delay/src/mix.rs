use crate::shared::float_ext::FloatExt;
use std::f32::consts::FRAC_PI_2;

pub struct Mix;

impl Mix {
  pub fn process(dry: f32, wet: (f32, f32), mix: f32) -> (f32, f32) {
    let fraction = mix * FRAC_PI_2;
    let dry_gain = fraction.fast_cos();
    let wet_gain = fraction.fast_sin();
    let dry_out = dry * dry_gain;
    (dry_out + wet.0 * wet_gain, dry_out + wet.1 * wet_gain)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!((left * 100.).round() / 100., (right * 100.).round() / 100.)
  }

  #[test]
  fn mix() {
    let first = Mix::process(0., (1., 1.), 0.);
    let second = Mix::process(0., (1., 1.), 0.5);
    let third = Mix::process(0., (1., 1.), 1.);
    assert_approximately_eq(first.0, 0.);
    assert_approximately_eq(first.1, 0.);
    assert_approximately_eq(second.0, 0.707);
    assert_approximately_eq(second.1, 0.707);
    assert_approximately_eq(third.0, 1.);
    assert_approximately_eq(third.1, 1.);
  }
}
