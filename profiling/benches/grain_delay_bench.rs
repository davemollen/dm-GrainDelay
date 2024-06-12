use criterion::{criterion_group, criterion_main, Criterion};
use grain_delay::GrainDelay;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_signal_stream(length: usize) -> Vec<f32> {
  (0..length).map(|_| generate_signal()).collect()
}

fn grain_delay_bench(c: &mut Criterion) {
  let mut grain_delay = GrainDelay::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  let pitch = 13.;
  let filter = 4000.;
  let feedback = 0.8;
  let mix = 0.8;
  grain_delay.initialize_params(pitch, filter, feedback, mix);

  c.bench_function("grain_delay", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        grain_delay.process(
          *signal, 2., 7., pitch, 0.2, 0.5, 200., feedback, filter, 0.5, mix,
        );
      }
    })
  });
}

criterion_group!(benches, grain_delay_bench);
criterion_main!(benches);
