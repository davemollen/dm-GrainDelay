pub struct Clip;

impl Clip {
  pub fn run(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
      min
    } else if input > max {
      max
    } else {
      input
    }
  }
}
