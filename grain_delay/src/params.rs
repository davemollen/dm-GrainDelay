mod smooth;
use smooth::LinearSmooth;
pub use {crate::MAX_DRIFT, smooth::Smoother};

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
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      spray: 0.,
      freq: 0.,
      speed: LinearSmooth::new(sample_rate, 12.),
      drift: 0.,
      reverse: 0.,
      time: 0.,
      feedback: LinearSmooth::new(sample_rate, 12.),
      filter: LinearSmooth::new(sample_rate, 12.),
      spread: 0.,
      mix: LinearSmooth::new(sample_rate, 12.),
      is_initialized: false,
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
    self.drift = drift * drift * MAX_DRIFT;
    self.reverse = reverse;
    self.time = time;
    self.spread = spread;

    let speed = 2_f32.powf(pitch / 12.);
    if self.is_initialized {
      self.speed.set_target(speed);
      self.feedback.set_target(feedback);
      self.filter.set_target(filter);
      self.mix.set_target(mix);
    } else {
      self.speed.reset(speed);
      self.feedback.reset(feedback);
      self.filter.reset(filter);
      self.mix.reset(mix);
      self.is_initialized = true;
    }
  }
}
