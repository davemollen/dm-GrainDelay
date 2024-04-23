## dm-GrainDelay

A granular delay effect written in Rust.
The effect is available for [Mod devices](https://moddevices.com/) and you can compile it to a VST plugin.
This plugin has been written primarily to run on [Mod devices](https://moddevices.com/). It's a mono to stereo plugin.

## Table of contents:

- [Mod devices installation](#Mod-devices-installation)
- [VST installation](#VST-installation)

## Mod devices installation
Install it through the Mod Plugin store. It's available as a beta plugin.

## VST installation

To build the plugin you need to install Rust first.

Windows:

1. Go to the vst folder and run `cargo build --release`
2. Copy the plugin file in /target/release to your vst plugin folder

Intel Mac:

- Run `./scripts/build-vst-for-mac.sh`.

M1 Mac:

- Run `./scripts/build-vst-for-mac-m1.sh`.
