use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i8:8:32-i16:16:32-i64:64-n32".into(),
        arch: "xtensa".into(),
        
        options: TargetOptions {
            cpu: "esp32-s2".into(),
            linker: Some("xtensa-esp32s2-elf-gcc".into()),
            max_atomic_width: Some(32),
            ..super::xtensa_base::opts()
        },
    }
}