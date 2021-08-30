## dm-Octaver

A harsh octave down effect written in Rust.

## Table of contents:

- [Description](#Description)
- [Install instructions](#Install-instructions)

## Description

This lv2 plugin has been written to run on Mod devices. [Github Actions](https://github.com/davemollen/dm-Octaver/actions) are used to build the binary. There's a build pipeline for both the Mod Duo and the Mod Dwarf. To my understanding the Mod Dwarf should also run fine on the Mod Duo X, but this has not been tested yet.

## Install instructions

- Copy the LV2 folder into your Mod:

  ```
  scp -rp <path to dm-Octaver.lv2> root@192.168.51.1:/root/.lv2
  ```

- Enter Mod password
- Reboot Mod
