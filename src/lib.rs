// Import everything we need.
extern crate lv2;
use lv2::prelude::*;

// The input and output ports are defined by a struct which implements the `PortCollection` trait.
// In this case, there is an input control port for the gain of the amplification, an input audio
// port and an output audio port.
#[derive(PortCollection)]
struct Ports {
    gain: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

// The plugin struct. In this case, we don't need any data and therefore, this struct is empty.
//
// LV2 uses URIs to identify types. This association is expressed via the `UriBound` trait,
// which tells the framework that the type `Amp` is identified by the given URI. The usual
// way to implement this trait is to use the `uri` attribute.
#[uri("urn:rust-lv2-book:eg-amp-rs")]
struct Amp;

// The implementation of the `Plugin` trait, which turns `Amp` into a plugin.
impl Plugin for Amp {
    // Tell the framework which ports this plugin has.
    type Ports = Ports;

    // We don't need any special host features; We can leave them out.
    type InitFeatures = ();
    type AudioFeatures = ();

    // Create a new instance of the plugin; Trivial in this case.
    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self)
    }

    // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
    // iterates over.
    fn run(&mut self, ports: &mut Ports, _features: &mut ()) {
        let coef = if *(ports.gain) > -90.0 {
            10.0_f32.powf(*(ports.gain) * 0.05)
        } else {
            0.0
        };

        for (in_frame, out_frame) in Iterator::zip(ports.input.iter(), ports.output.iter_mut()) {
            *out_frame = in_frame * coef;
        }
    }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(Amp);
