// main.rs 

#![no_std]  // Disable linking for standard library
#![no_main] // Overwrite normal entry point chain i.e. crt0 environment setup

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { 
    loop{}
}

#[no_mangle] // Allows use of _start name opposed to random generated function name for rust
pub extern "C" fn _start() -> ! { // _start is convention name for entry point
    loop{}
}


