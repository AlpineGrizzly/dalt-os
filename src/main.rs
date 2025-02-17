// main.rs 

#![no_std]  // Disable linking for standard library
#![no_main] // Overwrite normal entry point chain i.e. crt0 environment setup

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { 
    loop{}
}

static HELLO: &[u8] = b"Hello World!"; 

#[no_mangle] // Allows use of _start name opposed to random generated function name for rust
pub extern "C" fn _start() -> ! { 
    // function is entry point, since the linker looks for a function
    // named '_start' by default

    let vga_buffer = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() { 
        unsafe { 
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}


