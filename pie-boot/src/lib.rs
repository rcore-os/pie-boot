#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

mod loader;

use pie_boot_if::BootArgs;
use pie_boot_macros::start_code;

static mut BOOT_ARGS: BootArgs = BootArgs::new();
