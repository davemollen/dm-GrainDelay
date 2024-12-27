mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub spray: f32,
  pub freq: f32,
  pub speed: LinearSmooth,
  pub drift: f32,
  pub reverse: f32,
  pub time: f32,
  pub feedback: LinearSmooth,
  pub filter: LinearSmooth,
  pub spread: f32,
  pub mix: LinearSmooth,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      spray: 0.,
      freq: 0.,
      speed: LinearSmooth::new(12., sample_rate),
      drift: 0.,
      reverse: 0.,
      time: 0.,
      feedback: LinearSmooth::new(12., sample_rate),
      filter: LinearSmooth::new(12., sample_rate),
      spread: 0.,
      mix: LinearSmooth::new(12., sample_rate),
    }
  }

  pub fn set(
    &mut self,
    spray: f32,
    freq: f32,
    pitch: f32,
    drift: f32,
    reverse: f32,
    time: f32,
    feedback: f32,
    filter: f32,
    spread: f32,
    mix: f32,
  ) {
    self.spray = spray;
    self.freq = freq;
    self.speed.set_target(2_f32.powf(pitch / 12.));
    self.drift = drift * drift;
    self.reverse = reverse;
    self.time = time;
    self.feedback.set_target(feedback);
    self.filter.set_target(filter);
    self.spread = spread;
    self.mix.set_target(mix);
  }
}
