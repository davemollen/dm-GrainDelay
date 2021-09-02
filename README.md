## dm-Octaver

A harsh octave down effect written in Rust.

## Table of contents:

- [Description](#Description)
- [Reuse](#Reuse)
- [Install instructions](#Install-instructions)
- [License](#License)

## Description

This lv2 plugin has been written to run on Mod devices. [Github Actions](https://github.com/davemollen/dm-Octaver/actions) are used to build the binaries. There's a build pipeline for both the Mod Duo and the Mod Dwarf. To my understanding the Mod Dwarf build should also run fine on the Mod Duo X, but this has not been tested yet. Just download the artifacts to use the binaries.

## Reuse

If you would like to use this code for other purposes, you could just use the core audio processing in [`./src/octaver`](./src/octaver).

- Initialize first:

```
octaver: Octaver::new(_plugin_info.sample_rate())
```

- Then run:

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

## License

[![CC BY-SA 4.0][cc-by-sa-shield]][cc-by-sa]

This work is licensed under a
[Creative Commons Attribution-ShareAlike 4.0 International License][cc-by-sa].

[![CC BY-SA 4.0][cc-by-sa-image]][cc-by-sa]

[cc-by-sa]: http://creativecommons.org/licenses/by-sa/4.0/
[cc-by-sa-image]: https://licensebuttons.net/l/by-sa/4.0/88x31.png
[cc-by-sa-shield]: https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg
