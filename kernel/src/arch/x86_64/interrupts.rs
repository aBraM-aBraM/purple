use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{interrupts, println};
use crate::arch::x86_64::gdt;

/// CPU halts and waits for interrupts
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn software_breakpoint() {
    x86_64::instructions::interrupts::int3();
}


// using the first index after the first 32 interrupt
// indices used by the cpu
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    interrupts::timer_interrupt_handler();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn init() {
    gdt::init();
    IDT.load();
}

pub fn init_external_interrupts() {
    unsafe { PICS.lock().initialize() };
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref PORT: Mutex<Port<u8>> = Mutex::new(Port::new(0x60));
    }

    let mut port = PORT.lock();

    let scancode: u8 = unsafe { port.read() };
    interrupts::keyboard_interrupt_handler(scancode);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}


// diverging because x86_64 doesn't permit returning from double fault
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}


// uses x86-interrupt calling conventions
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
