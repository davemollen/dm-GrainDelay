extern crate lv2;
mod delta;
use delta::Delta;
mod lowpass;
use lowpass::Lowpass;
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
    gain: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Octaver")]
struct Octaver {
    lowpass: Lowpass,
    delta: Delta,
    flip_flop: f32,
}

impl Plugin for Octaver {
    // Tell the framework which ports this plugin has.
    type Ports = Ports;

    // We don't need any special host features; We can leave them out.
    type InitFeatures = ();
    type AudioFeatures = ();

    // Create a new instance of the plugin; Trivial in this case.
    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            lowpass: Lowpass::new(),
            delta: Delta::new(),
            flip_flop: 1.,
        })
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let amplification = if *(ports.gain) > -90.0 {
            10.0_f32.powf(*(ports.gain) * 0.05)
        } else {
            0.0
        };

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            let amplify = self.lowpass.run(*in_frame, 0.9997) * 10000.;
            let clip = if amplify > 1. {
                1.
            } else if amplify < -1. {
                -1.
            } else {
                amplify
            };

            let is_below_zero = if clip < 0. { 1. } else { 0. };
            let trigger = self.delta.run(is_below_zero) > 0.;
            if trigger {
                if self.flip_flop == 1. {
                    self.flip_flop = -1.
                } else {
                    self.flip_flop = 1.
                }
            };
            *out_frame = clip * self.flip_flop * amplification;
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(Octaver);
