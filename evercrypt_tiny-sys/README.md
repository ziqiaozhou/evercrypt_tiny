[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/evercrypt-tiny-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/evercrypt-tiny-rust)
[![docs.rs](https://docs.rs/evercrypt_tiny-sys/badge.svg)](https://docs.rs/evercrypt_tiny-sys)
[![crates.io](https://img.shields.io/crates/v/evercrypt_tiny-sys.svg)](https://crates.io/crates/evercrypt_tiny-sys)
[![Download numbers](https://img.shields.io/crates/d/evercrypt_tiny-sys.svg)](https://crates.io/crates/evercrypt_tiny-sys)
[![dependency status](https://deps.rs/crate/evercrypt_tiny-sys/0.1.0/status.svg)](https://deps.rs/crate/evercrypt_tiny-sys/0.1.0)


# `evercrypt_tiny-sys`
Welcome to `evercrypt_tiny-sys` 🎉

This library provides a vendored copy of [EverCrypt](https://github.com/project-everest/hacl-star)'s c89-compatible
distribution (currently v0.4.5) together with bindgen-generated bindings.

## Important
Please note that – depending on your target platform – some symbols may be unavailable even if they are exposed by
bindgen.
