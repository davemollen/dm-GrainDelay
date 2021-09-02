## dm-Octaver

A harsh octave down effect written in Rust.

## Table of contents:

- [Description](#Description)
- [Install instructions](#Install-instructions)

## Description

This lv2 plugin has been written to run on Mod devices. [Github Actions](https://github.com/davemollen/dm-Octaver/actions) are used to build the binaries. There's a build pipeline for both the Mod Duo and the Mod Dwarf. To my understanding the Mod Dwarf build should also run fine on the Mod Duo X, but this has not been tested yet.

Feel free to use this code for your own purposes. If you want to compile this for vst for example, you could just use the core audio processing in [`./src/octaver`](./src/octaver).

Initialize first:

```
octaver: Octaver::new(_plugin_info.sample_rate())
```

Then run:

```
self.octaver.run(input: f32, threshold: f32, gain: f32)
```

## Install instructions

- Copy the LV2 folder into your Mod:

  ```
  scp -rp <path to dm-Octaver.lv2> root@192.168.51.1:/root/.lv2
  ```

- Enter Mod password
- Reboot Mod
