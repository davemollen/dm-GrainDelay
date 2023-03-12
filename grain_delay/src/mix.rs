use std::f32::{self, consts::FRAC_1_SQRT_2};

pub struct Mix;

impl Mix {
  pub fn run(dry: f32, wet: (f32, f32), mix: f32) -> (f32, f32) {
    let twopi = f32::consts::PI * 2.;
    let phase = mix * 0.25;
    let dry_gain = (phase * twopi).cos() * FRAC_1_SQRT_2;
    let wet_gain = ((phase + 0.75) * twopi).cos();
    let dry_out = dry * dry_gain;
    (dry_out + wet.0 * wet_gain, dry_out + wet.1 * wet_gain)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_approximately_eq(left: f32, right: f32) {
    assert_eq!(
      (left * 1000.).round() / 1000.,
      (right * 1000.).round() / 1000.
    )
  }

  #[test]
  fn mix() {
    let first = Mix::run(0., (1., 1.), 0.);
    let second = Mix::run(0., (1., 1.), 0.5);
    let third = Mix::run(0., (1., 1.), 1.);
    assert_approximately_eq(first.0, 0.);
    assert_approximately_eq(first.1, 0.);
    assert_approximately_eq(second.0, 0.707);
    assert_approximately_eq(second.1, 0.707);
    assert_approximately_eq(third.0, 1.);
    assert_approximately_eq(third.1, 1.);
  }
}
