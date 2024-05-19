use grain_delay::GrainDelay;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn main() {
  let mut grain_delay = GrainDelay::new(44100.);

  let pitch = 12.;
  let filter = 4000.;
  let feedback = 0.8;
  let mix = 0.5;
  grain_delay.initialize_params(pitch, filter, feedback, mix);

  loop {
    let input = generate_signal();
    grain_delay.process(
      input, 2., 7., pitch, 0.2, 0.5, 200., feedback, filter, 0.5, mix,
    );
  }
}
