# dalt-os
Following tutorial for https://os.phil-opp.com/ for creating a small operating system

## Build
Building uses the nightly version of rustc to allow for experimental macros
such as asm!.
```sh
apt/dnf install cargo rustc rustup
rustup target add thumbv7em-none-eabihf
```

This is instructions for precompilation stuff for the kernel pt2
'''sh
rustup component add rust-src
cargo install bootimage
rustup component add llvm-tools-preview
'''

Creating image
'''
cargo bootimage
'''

Booting image using QEMU
'''
target/x86_64-dalt_os/debug/bootimage-dalt-os.bin
'''
