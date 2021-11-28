## dm-Reverse

A reverse delay effect written in Rust.

## Table of contents:

- [Install instructions](#Install-instructions)
- [License](#License)

## Install instructions

Windows:

1. Run `cargo build --release`
2. Copy libdm_reverse.dll in /target/release to your vst plugin folder

Mac

1. Run `cargo build --release`
2. Run `./osx_vst_bundler.sh dm-Reverse target/release/libdm_reverse.dylib`
3. Copy dm-Reverse.vst in the root of this folder to your vst plugin folder

## License

[![CC BY 4.0][cc-by-shield]][cc-by]

This work is licensed under a
[Creative Commons Attribution 4.0 International License][cc-by].

[![CC BY 4.0][cc-by-image]][cc-by]

[cc-by]: http://creativecommons.org/licenses/by/4.0/
[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png
[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg
