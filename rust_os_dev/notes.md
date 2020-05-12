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
- `read()` and `write()` method to access `volotaile` vars
- limitation for static interface (due to current Rust state)
	- `lazy_static` crate used to circumvent this
