#![no_std]
#![no_main] /// Overwrite crt0 entry point
use core::panic::PanicInfo;

/// no_mangle to disable name mangling to really outputs function witn name "_start"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop{}
}

/// panic handler, "!" represents the never type
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}