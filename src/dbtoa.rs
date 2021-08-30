pub struct Dbtoa {}

impl Dbtoa {
  pub fn run(x: f32) -> f32 {
    let amplitude = if x > -90.0 {
      10.0_f32.powf(x * 0.05)
    } else {
      0.0
    };
    amplitude
  }
}
