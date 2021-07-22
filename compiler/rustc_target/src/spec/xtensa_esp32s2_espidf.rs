use crate::spec::{LinkerFlavor, Target, TargetOptions};
use crate::abi::Endian;

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".to_string(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32".to_string(),
        arch: "xtensa".to_string(),

        options: TargetOptions {
            endian: Endian::Little,
            c_int_width: "32".to_string(),
            families: vec!["unix".to_string()],
            os: "espidf".to_string(),
            env: "newlib".to_string(),
            vendor: "espressif".to_string(),
            linker_flavor: LinkerFlavor::Gcc,

            executables: true,
            cpu: "esp32-s2".to_string(),
            linker: Some("xtensa-esp32s2-elf-gcc".to_string()),

            // See https://github.com/espressif/rust-esp32-example/issues/3#issuecomment-861054477
            //
            // Unlike the original ESP32 chip, ESP32-S2 does not really support atomics.
            // If the missing hardware instruction ends up being emulated in ESP-IDF, we might want to revert
            // this change and claim that atomics are supported "in hardware" (even though they would be emulated
            // by actually trapping the illegal instruction exception handler and calling into an ESP-IDF C emulation code).
            //
            // However, for now we simultaneously claim "max_atomic_width: Some(32)" **and** atomic_cas: true,
            // which should force the compiler to generate libcalls to functions that emulate atomics
            // and which are already implemented in the ESP-IDF main branch anyway.
            max_atomic_width: Some(32),
            atomic_cas: true,

            ..super::xtensa_base::opts()
        },
    }
}