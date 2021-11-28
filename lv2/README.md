## dm-GrainDelay

A granular delay effect written in Rust.

## Table of contents:

- [Description](#Description)
- [Build](#Build)
- [Install instructions](#Install-instructions)
- [GUI](#GUI)
- [License](#License)

## Description

This [lv2 plugin](./dm-GrainDelay.lv2) has been written to run on Mod devices. In this repository you can find the source code written in Rust and a Docker build tool for the Mod Duo and Mod Dwarf binaries. Follow the steps below to deploy the plugin to your Mod device and/or build for a different Mod platform.

## Build

In order to build the binaries you need to have Docker installed. If so, proceed with the following steps:

- Run `./build.sh` in the root directory.
- Copy/paste the binary of the target platform from the `./out` directory into dm-GrainDelay.lv2

## Install instructions

- Copy the .lv2 folder into your Mod:

  ```
  scp -rp <path to dm-GrainDelay.lv2> root@192.168.51.1:/root/.lv2
  ```

- Enter Mod password
- Reboot Mod

## GUI

The GUI was built with the [MOD SDK](https://github.com/moddevices/mod-sdk)

## License

[![CC BY 4.0][cc-by-shield]][cc-by]

This work is licensed under a
[Creative Commons Attribution 4.0 International License][cc-by].

[![CC BY 4.0][cc-by-image]][cc-by]

[cc-by]: http://creativecommons.org/licenses/by/4.0/
[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png
[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg
