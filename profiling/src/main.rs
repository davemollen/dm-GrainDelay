use grain_delay::GrainDelay;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut grain_delay = GrainDelay::new(44100.);

  loop {
    let input = generate_signal();
    grain_delay.process(input, 2., 7., 12., 0.2, 0.5, 200., 0.8, 4000., 0.5, 0.5);
  }
}