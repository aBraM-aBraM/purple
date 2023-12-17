#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]


use core::panic::PanicInfo;
use kernel::{exit_qemu, serial_println, QemuExitCode, interrupts};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[cfg(target_arch = "x86_64")]
use kernel::arch::x86_64 as arch;

#[cfg(target_arch = "x86_64")]
lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(arch::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[cfg(target_arch = "x86_64")]
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    interrupts::hlt_loop();
}

#[cfg(target_arch = "x86_64")]
pub fn init_test_idt() {
    TEST_IDT.load();
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("stack_overflow::stack_overflow...\t");

    interrupts::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read(); // prevent tail recursion optimization
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}
