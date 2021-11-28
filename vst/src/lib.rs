#[macro_use]
extern crate vst;

use grain_delay::GrainDelay;
use std::sync::Arc;
use vst::api::TimeInfo;
use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::{HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

struct DmGrainDelay {
    params: Arc<GrainDelayParameters>,
    grain_delay: GrainDelay,
}

struct GrainDelayParameters {
    spray: AtomicFloat,
    frequency: AtomicFloat,
    pitch: AtomicFloat,
    rand_pitch: AtomicFloat,
    delay_time: AtomicFloat,
    feedback: AtomicFloat,
    mix: AtomicFloat,
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
            mix: AtomicFloat::new(0.5),
        }
    }
}

impl Default for DmGrainDelay {
    fn default() -> Self {
        Self {
            params: Arc::new(GrainDelayParameters::default()),
            grain_delay: GrainDelay::new(44100.),
        }
    }
}

impl Plugin for DmGrainDelay {
    fn new(host: HostCallback) -> Self {
        fn get_sample_rate(info: TimeInfo) -> f64 {
            info.sample_rate
        }
        let sample_rate = host.get_time_info(0).map(get_sample_rate).unwrap();
        Self {
            params: Arc::new(GrainDelayParameters::default()),
            grain_delay: GrainDelay::new(sample_rate),
        }
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.grain_delay = GrainDelay::new(f64::from(sample_rate));
    }

    fn get_info(&self) -> Info {
        Info {
            name: "dm-GrainDelay".to_string(),
            inputs: 1,
            outputs: 1,
            parameters: 7,
            unique_id: 1358,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let spray = self.params.spray.get();
        let frequency = self.params.frequency.get();
        let pitch = self.params.pitch.get();
        let rand_pitch = self.params.rand_pitch.get();
        let delay_time = self.params.delay_time.get();
        let feedback = self.params.feedback.get();
        let mix = self.params.mix.get();

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = self.grain_delay.run(
                    *input_sample,
                    spray,
                    frequency,
                    pitch,
                    rand_pitch,
                    delay_time,
                    feedback,
                    mix,
                );
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for GrainDelayParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.spray.get(),
            1 => self.frequency.get(),
            2 => self.pitch.get(),
            3 => self.rand_pitch.get(),
            4 => self.delay_time.get(),
            5 => self.feedback.get(),
            6 => self.mix.get(),
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
            6 => format!("{:.2}%", self.mix.get() * 100.0),
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
            6 => "Mix",
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
            6 => self.mix.set(val),
            _ => (),
        }
    }
}

plugin_main!(DmGrainDelay);
