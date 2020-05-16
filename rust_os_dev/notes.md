Freestanding Rust binary
------------------------

- do not include the standard library
- do not link to the std lib so that the linker does not try to look for
  `_start` from the C library
- disable stack unwinding in `Cargo.toml`
- manually implement the `panic()` handler (by default part of the std lib)
- manually implement the `_start()` entrypoint, without name mangling

Minimal kernel
--------------

- create a target specification files
- disable SIMD instructions for kernel code
- switch to rust *nightly* version
- use the crates `bootimage` and `bootloader` to automatically add a bootloader
  to our kernel and link them both

VGA text mode
-------------

- VGA buffer accessible at 0xb8000
- `volatile` to avoid optimization for value that may seem unused by the
    compiler (like writing to MMIO buffer)
- `read()` and `write()` method to access `volatile` vars
- limitation for static interface (due to current Rust state)
	- `lazy_static` crate used to circumvent this, it performs the
	    initialization of the var at the first encounter instead of doing it
	    at compile time

Testing
-------

- need some feature attribute to make a custom test framework which does not
    rely on `std`
- the crate `x86_64` gives features to manipulate system regs (like `in` and
    `out` instructions)
- the crate `uart_16550` ease the implementation of an UART interface (to print
    QEMU's output to `stdio`)
- executable in `tests/` are automatically consider as tests (no need for
   `#[test_case]`)
- `lib.rs` recognized by cargo and build as a library, contains all public code
  usable by other modules (e.g. the tests)

CPU exceptions
--------------

- `x86-interrupt` as calling convention to preserves all registers when entering
    an interrupt handler
- **IDT** and **Interrupt Stack Frame** are represented in the `x86_64` crate
- `iretq` instruction to return from interrupt
- A `.rs` file is a `module` except for the `main.rs` and `lib.rs` which are
    crates (the final binary objects)
- **IDTR**: register to hold the address of the IDT

Double faults
-------------

- double faults occur only in certain combinaison of successive faults (it is ok
    for a page fault to occur within a divide-by-zero interruption handler)
- **Interrupt Stack Table** (IST): Table to valid stack base addresses, used to
    switch stack before an handling exception so the CPU can push the exception
    stack frame without causing a page fault
- **Task State Segment** (TSS): Holds the IST, the **Privilege Stack Table** and
    the **I/O Map Base Address**
- **Global Descriptor Table** (GDT): Use to configure segmented virtual memory,
    still needed even in 64-bit mode (where paging is used instead) to load the
    TSS (and kernel/user mode config)
- disable test harness in `Cargo.toml` to execute the test directly from
    `_start`, without a `runner`
