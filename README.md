# opencsd-sys

[![Crates.io Version](https://img.shields.io/crates/v/opencsd-sys)](https://crates.io/crates/opencsd-sys)
[![docs.rs](https://img.shields.io/docsrs/opencsd-sys)](https://docs.rs/opencsd-sys)
[![Crates.io License](https://img.shields.io/crates/l/opencsd-sys)](https://github.com/kesyog/opencsd-sys/blob/main/LICENSE)

Auto-generated Rust bindings to the C API of the [OpenCSD](https://github.com/Linaro/OpenCSD)
library. OpenCSD is an
open-source decoder for ARM CoreSight trace streams.

## Development

* OpenCSD is vendored as a submodule in `vendor/OpenCSD`.
* To save build time and reduce dependencies, bindings are pre-generated and committed to the repository for non-Windows systems. Run `bindings.sh` to re-generate them.
* Bindings are generated on-the-fly for Windows systems.

## Disclaimer

This is not an officially supported Google product.
