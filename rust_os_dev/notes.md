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
    switch stacks before the handling of an exception so that the CPU can push the
    exception stack frame without causing a page fault
- **Task State Segment** (TSS): Holds the IST, the **Privilege Stack Table** and
    the **I/O Map Base Address**
- **Global Descriptor Table** (GDT): Used to configure segmented virtual memory,
    still needed even in 64-bit mode (where paging is used instead) to load the
    TSS (and for kernel/user mode transition)
- disable test harness in `Cargo.toml` to execute the test directly from
    `_start`, without a `runner`

Hardware interrupts
-------------------

- how the `Mutex`'s `lock()` method from the `spin` crate works (cf. crate's
    doc):
> Locks the spinlock and returns a guard.
> The returned value may be dereferenced for data access and the lock will be
> dropped when the guard falls out of scope.
- `sti` instruction = set interrupts
- add the `pic8259_simple` crate for a PIC implementation (the timer is enabled
    by default, need to catch those INT first)
- the PIC expect an "end of interrupt" (EOI) signal to know that the interrupt
    has been dealt with
- need to disable interrupts in the `_print` macro to avoid deadlocks since it
    is also used in the handlers
- the PS/2 controller won't send new interrupts until the scancode is read (data
    port of the controller)
- `Some(var)` is equivalent to `if var is None` in Python (cf. std::Option)

Paging introduction
-------------------

- page = virtual memory
- frame = physical memory
- paging already implemented here thanks to the bootloader crate
- **Identity Mapped** == **Identity Paging** == **1:1 Paging**
    - the virtual address == the physical address
- the `CR2` register contains the virtual address responsible for a page fault
- the `CR3` register conatins a physical memory address

Paging implementation
---------------------

- recursive mapping allows to skip a level of translation by having an entry
    in the page pointing to another entry of the same page, this time being
    considered by the CPU as an entry of the next level page table
- the `BootInfo` arg of `_start()` comes from the bootloader
- `_start()` can be renamed in a "normal" function, without the feature attribute
    and `extern` keyword thanks to the `entry_point!` macro from the `bootloader`
    crate
- the `x86_64` crate offers abstraction for address translations (has the
    advantages of supporting *huge pages*)
- `'static` lifetime means the object lives for the entire duration of the
    program
- problematic when creating a new mapping:
    > In summary, the difficulty of creating a new mapping depends on the virtual
    > page that we want to map. In the easiest case, the level 1 page table for
    > the page already exists and we just need to write a single entry. In the
    > most difficult case, the page is in a memory region for that no level 3
    > exists yet so that we need to create new level 3, level 2 and level 1 page
    > tables first.

Heap allocation
---------------

- `(x as *mut 32).offset(i)` is equivalent to `(int*)x[i]` in C
- dynamic allocation usually done with `Box::new()` (uses `allocate` behind the
    scene)
- the `Drop` trait defines how to free an object when it goes out of scope
- to add a dependecy to the built-in `alloc` crate we need to implement the
    `GlobalAlloc` trait for a static object (i.e. a heap allocator)
- a `trait` is like a collection of abstract methods
    - a method of the trait can have a default implementation
- `?` akin to a `try..catch`, returns early in case of an error
- takeaway from summary
    > defined a virtual address range for the heap (Page::range_inclusive())
    > mapped all pages of that range to physical frames using the Mapper and FrameAllocator
