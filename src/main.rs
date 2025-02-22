// main.rs 

#![no_std]  // Disable linking for standard library
#![no_main] // Overwrite normal entry point chain i.e. crt0 environment setup

use core::panic::PanicInfo;

mod vga_buffer; 

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { 
    println!("{}", info);
    loop{}
}

#[no_mangle] // Allows use of _start name opposed to random generated function name for rust
pub extern "C" fn _start() -> ! { 
    // function is entry point, since the linker looks for a function
    // named '_start' by default
   
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Welcome to Dalt-os!").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();    

    println!("Welcome to Dalt-os{}", "!");
    panic!("No clue what to do!");
    loop{}
}


