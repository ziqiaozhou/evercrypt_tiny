//! A simplified replacement for `./configure`

use std::env;

/// The architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Arch {
    /// arm
    arm,
    /// x86
    x86,
    /// x64
    x86_64,
}

/// A configuration object
#[derive(Debug, Clone, Copy)]
pub struct Configuration {
    /// The architecture
    pub arch: Arch,
    /// Support for 128 bit vector instructions
    pub v128: bool,
    /// Support for 256 bit vector instructions
    pub v256: bool,
    /// Support for vale
    pub vale: bool,
    /// Support for inline assembly
    pub inline_asm: bool,
    /// Support for builtin compiler intrinsics
    pub intrinsics: bool,
    /// Support for native u128
    pub native_u128: bool,
}
impl Configuration {
    /// Creates a new configuration object
    pub fn new() -> Self {
        // Return a failsafe config
        if env::var("EVERCRYPT_FAILSAFE").is_ok() {
            return Self::failsafe();
        }

        // Current feature detection is simply based on arch detection
        // Therefore we try to chose a reasonable base line
        let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cannot determine target architecture");
        match arch.as_str() {
            "arm" | "aarch64" => Self {
                arch: Arch::arm,
                v128: false,
                v256: false,
                vale: false,
                inline_asm: false,
                intrinsics: false,
                native_u128: false,
            },
            "x86" => Self {
                arch: Arch::x86,
                v128: false,
                v256: false,
                vale: false,
                inline_asm: false,
                intrinsics: true,
                native_u128: false,
            },
            "x86_64" => Self {
                arch: Arch::x86_64,
                v128: false,
                v256: false,
                vale: true,
                inline_asm: true,
                intrinsics: true,
                native_u128: false,
            },
            arch => {
                panic!("Unsupported target platform {arch}")
            }
        }
    }

    /// Creates a minimal failsafe config
    pub fn failsafe() -> Self {
        // Determine arch
        let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Cannot determine target architecture");
        let arch = match arch.as_str() {
            "arm" | "aarch64" => Arch::arm,
            "x86" => Arch::x86,
            "x86_64" => Arch::x86_64,
            arch => panic!("Unsupported target platform {arch}"),
        };

        // Build config
        Self { arch, v128: false, v256: false, vale: false, inline_asm: false, intrinsics: false, native_u128: false }
    }
}
