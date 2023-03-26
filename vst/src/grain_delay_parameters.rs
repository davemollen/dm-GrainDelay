use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct GrainDelayParameters {
  pub spray: AtomicFloat,
  pub frequency: AtomicFloat,
  pub pitch: AtomicFloat,
  pub drift: AtomicFloat,
  pub reverse: AtomicFloat,
  pub scrub: AtomicFloat,
  pub time: AtomicFloat,
  pub feedback: AtomicFloat,
  pub filter: AtomicFloat,
  pub spread: AtomicFloat,
  pub mix: AtomicFloat,
}

impl Default for GrainDelayParameters {
  fn default() -> Self {
    Self {
      spray: AtomicFloat::new(2.0),
      frequency: AtomicFloat::new(7.0),
      pitch: AtomicFloat::new(12.),
      drift: AtomicFloat::new(0.),
      reverse: AtomicFloat::new(0.),
      scrub: AtomicFloat::new(0.),
      time: AtomicFloat::new(0.),
      feedback: AtomicFloat::new(0.),
      filter: AtomicFloat::new(0.),
      spread: AtomicFloat::new(0.),
      mix: AtomicFloat::new(0.5),
    }
  }
}

impl PluginParameters for GrainDelayParameters {
  fn get_parameter(&self, index: i32) -> f32 {
    match index {
      0 => (self.spray.get() / 500.).powf(0.333333),
      1 => ((self.frequency.get() - 1.) / 149.).powf(0.333333),
      2 => (self.pitch.get() + 24.) / 48.,
      3 => self.drift.get(),
      4 => self.reverse.get(),
      5 => self.scrub.get(),
      6 => (self.time.get() / 5000.).powf(0.333333),
      7 => self.feedback.get(),
      8 => self.filter.get(),
      9 => self.spread.get(),
      10 => self.mix.get(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2}ms", self.spray.get()),
      1 => format!("{:.2}hz", self.frequency.get()),
      2 => format!("{:.2}st", self.pitch.get()),
      3 => format!("{:.2}%", self.drift.get() * 100.0),
      4 => format!("{:.2}%", self.reverse.get() * 100.0),
      5 => format!("{:.2}%", self.scrub.get() * 100.0),
      6 => format!("{:.2}ms", self.time.get()),
      7 => format!("{:.2}%", self.feedback.get() * 100.0),
      8 => format!("{:.2}%", self.filter.get() * 100.0),
      9 => format!("{:.2}%", self.spread.get() * 100.0),
      10 => format!("{:.2}%", self.mix.get() * 100.0),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Spray",
      1 => "Frequency",
      2 => "Pitch",
      3 => "Drift",
      4 => "Reverse",
      5 => "Scrub",
      6 => "Time",
      7 => "Feedback",
      8 => "Filter",
      9 => "Spread",
      10 => "Mix",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.spray.set(val.powf(3.) * 500.),
      1 => self.frequency.set(val.powf(3.) * 149. + 1.),
      2 => self.pitch.set(val * 48. - 24.),
      3 => self.drift.set(val),
      4 => self.reverse.set(val),
      5 => self.scrub.set(val),
      6 => self.time.set(val.powf(3.) * 5000.),
      7 => self.feedback.set(val),
      8 => self.filter.set(val),
      9 => self.spread.set(val),
      10 => self.mix.set(val),
      _ => (),
    }
  }
}
