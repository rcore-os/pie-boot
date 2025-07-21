use core::ptr::NonNull;

use crate::static_once::StaticOnce;

static UART: StaticOnce<any_uart::Sender> = StaticOnce::new();

pub(crate) fn init_debugcon(fdt: Option<NonNull<u8>>) -> Option<()> {
    fn phys_to_virt(p: usize) -> *mut u8 {
        p as _
    }

    let uart = any_uart::init(fdt?, phys_to_virt)?;
    UART.init(uart.tx?);

    crate::early_debug::set_tx_fun(write_byte);
    Some(())
}

fn write_byte(b: u8) -> Result<(), crate::early_debug::TError> {
    let tx = unsafe { &mut *UART.get() };

    match tx.write(b) {
        Ok(_) => Ok(()),
        Err(e) => Err(match e {
            any_uart::Error::WouldBlock => crate::early_debug::TError::ReTry,
            any_uart::Error::Other(_e) => crate::early_debug::TError::Other,
        }),
    }
}
