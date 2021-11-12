use std::f32;

pub struct Mix;

impl Mix {
  pub fn run(dry: f32, wet: f32, mix: f32) -> f32 {
    let twopi = f32::consts::PI * 2.;
    let phase = mix * 0.25;
    let dry_gain = (phase * twopi).cos();
    let wet_gain = ((phase + 0.75) * twopi).cos();
    dry * dry_gain + wet * wet_gain
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mix() {
    let first = Mix::run(0., 1., 0.) * 1000.;
    let second = Mix::run(0., 1., 0.5) * 1000.;
    let third = Mix::run(0., 1., 1.) * 1000.;
    assert_eq!(first.floor() / 1000., 0.);
    assert_eq!(second.floor() / 1000., 0.707);
    assert_eq!(third.floor() / 1000., 1.);
  }
}
