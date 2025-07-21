use crate::static_once::StaticOnce;

static POWER_OFF: StaticOnce<fn() -> !> = StaticOnce::with_default(shutdown_empty);

pub(crate) fn set_power_off(power_off: fn() -> !) {
    POWER_OFF.init(power_off);
}

pub fn shutdown() -> ! {
    POWER_OFF()
}

fn shutdown_empty() -> ! {
    panic!("Shutdown not implemented");
}
