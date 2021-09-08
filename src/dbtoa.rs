pub struct Dbtoa;

impl Dbtoa {
  pub fn run(input: f32) -> f32 {
    let amplitude = if input > -90.0 {
      10.0_f32.powf(input * 0.05)
    } else {
      0.0
    };
    amplitude
  }
}
