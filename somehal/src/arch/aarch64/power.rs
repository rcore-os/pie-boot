use core::{ops::Deref, ptr::NonNull};

use aarch64_cpu::asm::wfi;
use fdt_parser::Fdt;
use smccc::{Hvc, Smc, psci};

use crate::{println, static_once::StaticOnce};

static METHOD: StaticOnce<Method> = StaticOnce::new();

#[derive(Debug, Clone, Copy)]
enum Method {
    Smc,
    Hvc,
}
impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "smc" => Method::Smc,
            "hvc" => Method::Hvc,
            _ => {
                panic!("Unsupported power method: {}", value);
            }
        }
    }
}

pub fn init_by_fdt(fdt: Option<NonNull<u8>>) -> Option<()> {
    let fdt = fdt?;
    let fdt = Fdt::from_ptr(fdt).ok()?;
    let node = fdt
        .find_compatible(&["arm,psci-1.0", "arm,psci-0.2", "arm,psci"])
        .next()?;
    let method: Method = node.find_property("method")?.str().into();
    METHOD.init(method);
    println!("Power management method : {:?}", method);

    crate::power::set_power_off(shutdown);
    Some(())
}

fn shutdown() -> ! {
    match METHOD.deref() {
        Method::Smc => psci::system_off::<Smc>(),
        Method::Hvc => psci::system_off::<Hvc>(),
    }
    .unwrap();
    loop {
        wfi();
    }
}
