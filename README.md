## dm-GrainDelay

A granular delay effect written in Rust.
The effect can be compiled to a [lv2](./lv2) or [vst](./vst) plugin.
This plugin has been written primarily to run on [Mod devices](https://moddevices.com/). And because I mainly use this for guitar it's just mono for now.

## Table of contents:

- [Mod devices installation](#Mod-devices-installation)
- [VST installation](#VST-installation)

## Mod devices installation

You can find the plugin [here](./lv2/dm-GrainDelay.lv2/).

- Copy the .lv2 plugin to your Mod:

  ```
  scp -rp <path to dm-GrainDelay.lv2> root@192.168.51.1:/root/.lv2
  ```

- Enter Mod password
- Reboot Mod

## LV2 installation (for Mod)

In order to build the binaries you need to have Docker installed. If so, proceed with the following steps:

- Pull docker image: `docker pull ubuntu:16.04` for Mod Duo, `docker pull ubuntu:18.04` for Mod Dwarf.
- Run `./scripts/build-lv2-for-mod.sh` in the root directory.
- Copy/paste the binary of the target platform from the `./lv2/out` directory into `./lv2/<plugin name>.lv2`

## VST installation

Windows:

1. Run `./scripts/build-vst.sh`
2. Copy dll file in /target/release to your vst plugin folder

Intel Mac:

- Run `./scripts/build-vst-for-mac.sh`.

M1 Mac:

- Run `./scripts/build-vst-for-mac-m1.sh`.
