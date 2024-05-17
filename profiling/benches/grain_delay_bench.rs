use criterion::{criterion_group, criterion_main, Criterion};
use grain_delay::GrainDelay;

fn generate_signal() -> f32 {
  fastrand::f32() * 2. - 1.
}

fn generate_signal_stream(length: usize) -> Vec<f32> {
  (0..length).map(|_| generate_signal()).collect()
}

fn reverb_bench(c: &mut Criterion) {
  let mut grain_delay = GrainDelay::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("grain_delay", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        grain_delay.process(*signal, 2., 7., 12., 0.2, 0.5, 200., 0.8, 4000., 0.5, 0.5);
      }
    })
  });
}

criterion_group!(benches, reverb_bench);
criterion_main!(benches);
