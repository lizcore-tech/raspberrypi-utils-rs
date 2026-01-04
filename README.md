# raspberrypi-utils-rs

[//]: # ([![Rust]&#40;https://github.com/lizcore-tech/raspberrypi-utils-rs/workflows/CI/badge.svg&#41;]&#40;https://github.com/lizcore-tech/raspberrypi-utils-rs/actions&#41;)
[![Latest version](https://img.shields.io/crates/v/raspberrypi-utils.svg)](https://crates.io/crates/raspberrypi-utils)
[![Documentation](https://docs.rs/raspberrypi-utils/badge.svg)](https://docs.rs/raspberrypi-utils)
![License](https://img.shields.io/crates/l/raspberrypi-utils.svg)

Rust bindings for [raspberrypi-utils](https://github.com/raspberrypi/utils). Mainly for `piolib`.

Project structure:
- [raspberrypi-utils-sys](./raspberrypi-utils-sys/) - Low-level unsafe bindings to raspberrypi-utils.
- [raspberrypi-utils](./raspberrypi-utils/) - Safe Ws2812 Rust interface on top of `raspberrypi-utils-sys`.

Code generation uses a cached clone under `raspberrypi-utils-git`; run `git checkout <commit-hash-or-tag>` occasionally to fetch the latest upstream tags before regenerating `versioned_files`.

## Building

`libcamera-sys` requires `cmake device-tree-compiler libfdt-dev libgnutls28-dev` packages installed and accessible via pkg-config.

No other special dependencies are needed. All crates can be built from the root workspace dir with `cargo build --workspace`.

## Running examples

Run LED string animation ([code](./raspberrypi-utils/examples/ws281x.rs)):

```console
cargo run --example ws281x
```

## Notes

Contributions are welcome! Please open an issue or submit a pull request.

## License

Licensed under either of

* BSD 3-Clause License, ([LICENSE-BSD](LICENSE-BSD) or https://opensource.org/license/bsd-3-clause)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
