use core::panic::PanicInfo;

use pie_boot_loader_macros::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic!");

    if let Some(msg) = info.message().as_str() {
        println!("msg: {}", msg);
    }

    loop {}
}
