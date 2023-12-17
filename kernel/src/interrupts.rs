use crate::{print, arch};
use pc_keyboard::Keyboard;

#[cfg(target_arch = "x86_64")]
use arch::x86_64::interrupts as arch_interrupts;

pub use arch_interrupts::hlt_loop;

pub fn init() {
    arch_interrupts::init();
    arch_interrupts::init_external_interrupts();
}


#[test_case]
fn test_breakpoint_interrupt() {
    // invoke a breakpoint interrupt
    arch_interrupts::software_breakpoint();
}


pub fn timer_interrupt_handler() {
    print!(".");
}

pub fn keyboard_interrupt_handler(scancode: u8) {
    use pc_keyboard::{layouts, ScancodeSet1, HandleControl, DecodedKey};

    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
}