//! A simplified replacement for `make`

use cc::Build;

use crate::{
    config::{Arch, Configuration},
    files::{
        FileList,
        Pattern::{Contains, End, Exact, Start},
        DIST_C89, DIST_KARAMEL_INCLUDE, DIST_KARAMEL_MINIMAL_INCLUDE,
    },
};
use std::{env, fs, path::Path};

/// A simplified replacement for `make`
#[derive(Debug)]
pub struct Make {
    /// The platform configuration
    config: Configuration,
}
impl Make {
    /// Creates a new builder
    pub fn new(config: Configuration) -> Self {
        Self { config }
    }

    /// Builds the library
    pub fn build(&self) {
        // Determine the output directory
        let out_dir = env::var("OUT_DIR").expect("Failed to get target output directory");

        // Create config file
        let config_h_path = Path::new(&out_dir).join("config.h");
        let config_h = self.config_h();
        fs::write(config_h_path, config_h).expect("Failed to create config.h");

        // Gather sources and include paths
        let c_sources = dbg!(self.c_sources());
        let asm_sources = dbg!(self.asm_sources());
        let includes = self.includes();

        // Build the library
        Build::new()
            .include(out_dir)
            .includes(includes)
            .files(c_sources.paths())
            .files(asm_sources.paths())
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wno-unused-variable")
            .flag_if_supported("-Wno-unused-but-set-variable")
            .flag_if_supported("-Wno-unused-function")
            .flag_if_supported("-Wno-cpp")
            .compile("evercrypt");
    }

    /// Build a config.h
    fn config_h(&self) -> String {
        // Target architecture constant
        let target_arch = match self.config.arch {
            Arch::arm if !self.config.v128 => "#define TARGET_ARCHITECTURE TARGET_ARCHITECTURE_ID_ARM7",
            Arch::arm => "#define TARGET_ARCHITECTURE TARGET_ARCHITECTURE_ID_ARM8",
            Arch::x86 => "#define TARGET_ARCHITECTURE TARGET_ARCHITECTURE_ID_X86",
            Arch::x86_64 => "#define TARGET_ARCHITECTURE TARGET_ARCHITECTURE_ID_X64",
        };
        // Support for compiler intrinsics
        let intrinsics = match self.config.intrinsics {
            true => "#define HACL_CAN_COMPILE_INTRINSICS 1",
            false => "// #define HACL_CAN_COMPILE_INTRINSICS 1",
        };
        // Support for vale
        let vale = match self.config.vale {
            true => "#define HACL_CAN_COMPILE_VALE 1",
            false => "// #define HACL_CAN_COMPILE_VALE 1",
        };
        // Support for inline assembly
        let inline_asm = match self.config.inline_asm {
            true => "#define HACL_CAN_COMPILE_INLINE_ASM 1",
            false => "// #define HACL_CAN_COMPILE_INLINE_ASM 1",
        };
        // 128 bit vector support
        let v128 = match self.config.v128 {
            true => "#define HACL_CAN_COMPILE_VEC128 1",
            false => "#define Lib_IntVector_Intrinsics_vec128 void *",
        };
        // 256 bit vector support
        let v256 = match self.config.v256 {
            true => "#define HACL_CAN_COMPILE_VEC256 1",
            false => "#define Lib_IntVector_Intrinsics_vec256 void *",
        };
        // Support for native 128 bit integer types
        let native_u128 = match self.config.native_u128 {
            true => "#define HACL_CAN_COMPILE_UINT128 1",
            false => "// #define HACL_CAN_COMPILE_UINT128 1",
        };

        // Build header
        let header = format! {
            r#"
            {target_arch}
            {intrinsics}
            {vale}
            {inline_asm}
            {v128}
            {v256}
            {native_u128}
            #define LINUX_NO_EXPLICIT_BZERO 1
            "#
        };
        header.lines().map(|line| format!("{}\n", line.trim())).collect()
    }

    /// Gather all config-specific C source files
    fn c_sources(&self) -> FileList {
        // Collect all sources
        let mut c_sources = FileList::new();
        c_sources.add(DIST_C89, End(".c"));

        // Blacklist x64 assembly files
        if !self.config.vale {
            c_sources.remove(Start("Hacl_HPKE_Curve64_") + End(".c"));
            c_sources.remove(Exact("Hacl_Curve25519_64.c"));
            c_sources.remove(Exact("evercrypt_vale_stubs.c"));
        }
        // Blacklist 128-bit vector arithmetic files
        if !self.config.v128 {
            c_sources.remove(Contains("CP128") + End(".c"));
            c_sources.remove(End("_128.c"));
            c_sources.remove(End("_Vec128.c"));
        }
        // Blacklist 256-bit vector arithmetic files
        if !self.config.v256 {
            c_sources.remove(Contains("CP256") + End(".c"));
            c_sources.remove(End("_256.c"));
            c_sources.remove(End("_Vec256.c"));
        }
        c_sources
    }

    /// Gather all config-specific assembly source files
    fn asm_sources(&self) -> FileList {
        // Collect assembly sources
        let mut asm_sources = FileList::new();
        if self.config.vale {
            // Get target arch or abort if a parameter is unsupported
            let arch = match self.config.arch {
                Arch::x86_64 => "x86_64",
                _ => return asm_sources,
            };

            // Get target OS
            let os = env::var("CARGO_CFG_TARGET_OS").expect("Cannot determine target OS");
            let os_env = env::var("CARGO_CFG_TARGET_ENV").expect("Cannot determine target environment");
            let (os, ext) = match os.as_str() {
                "macos" | "ios" => ("darwin", "S"),
                "linux" => ("linux", "S"),
                "windows" => match os_env.as_str() {
                    "msvc" => ("msvc", "asm"),
                    "gnu" => ("linux", "S"),
                    _ => return asm_sources,
                },
                _ => ("linux", "S"),
                //_ => return asm_sources,
            };

            // Gather files
            let ext = format!("-{arch}-{os}.{ext}");
            asm_sources.add(DIST_C89, End(ext));
        }
        asm_sources
    }

    /// The include paths
    fn includes(&self) -> &'static [&'static str] {
        &[DIST_C89, DIST_KARAMEL_INCLUDE, DIST_KARAMEL_MINIMAL_INCLUDE]
    }
}
