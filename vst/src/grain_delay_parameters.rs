use vst::{plugin::PluginParameters, util::AtomicFloat};

pub struct GrainDelayParameters {
  pub spray: AtomicFloat,
  pub frequency: AtomicFloat,
  pub pitch: AtomicFloat,
  pub rand_pitch: AtomicFloat,
  pub delay_time: AtomicFloat,
  pub feedback: AtomicFloat,
  pub low_pass: AtomicFloat,
  pub mix: AtomicFloat,
}

impl Default for GrainDelayParameters {
  fn default() -> Self {
    Self {
      spray: AtomicFloat::new(2.0),
      frequency: AtomicFloat::new(7.0),
      pitch: AtomicFloat::new(12.),
      rand_pitch: AtomicFloat::new(0.),
      delay_time: AtomicFloat::new(0.),
      feedback: AtomicFloat::new(0.),
      low_pass: AtomicFloat::new(5000.),
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
      3 => self.rand_pitch.get(),
      4 => (self.delay_time.get() / 5000.).powf(0.333333),
      5 => self.feedback.get(),
      6 => ((self.low_pass.get() - 20.) / 19980.).powf(0.333333),
      7 => self.mix.get(),
      _ => 0.0,
    }
  }

  fn get_parameter_text(&self, index: i32) -> String {
    match index {
      0 => format!("{:.2} ms", self.spray.get()),
      1 => format!("{:.2} hz", self.frequency.get()),
      2 => format!("{:.2} st", self.pitch.get()),
      3 => format!("{:.2}%", self.rand_pitch.get() * 100.0),
      4 => format!("{:.2} ms", self.delay_time.get()),
      5 => format!("{:.2}%", self.feedback.get() * 100.0),
      6 => format!("{:.2} hz", self.low_pass.get()),
      7 => format!("{:.2}%", self.mix.get() * 100.0),
      _ => "".to_string(),
    }
  }

  fn get_parameter_name(&self, index: i32) -> String {
    match index {
      0 => "Spray",
      1 => "Frequency",
      2 => "Pitch",
      3 => "Rand Pitch",
      4 => "Time",
      5 => "Feedback",
      6 => "Low Cut",
      7 => "Mix",
      _ => "",
    }
    .to_string()
  }

  fn set_parameter(&self, index: i32, val: f32) {
    match index {
      0 => self.spray.set(val.powf(3.) * 500.),
      1 => self.frequency.set(val.powf(3.) * 149. + 1.),
      2 => self.pitch.set(val * 48. - 24.),
      3 => self.rand_pitch.set(val),
      4 => self.delay_time.set(val.powf(3.) * 5000.),
      5 => self.feedback.set(val),
      6 => self.low_pass.set(val.powf(3.) * 19980. + 20.),
      7 => self.mix.set(val),
      _ => (),
    }
  }
}
