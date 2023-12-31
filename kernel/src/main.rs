#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"] // set test entry point name
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]

use kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    kernel::init();

    #[cfg(test)]
    test_main();

    kernel::interrupts::hlt_loop();
}

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::interrupts::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
