OS dev practice following this tutorial series: https://os.phil-opp.com/ .

Compile
-------

```bash
#cargo rustc -- -C link-args=-nostartfiles

#cargo xbuild --target x86_64-ros-target.json

cargo bootimage
```

Run
---

```bash
#qemu-system-x86_64 -drive format=raw,file=target/x86_64-ros-target/debug/bootimage-ros.bin

# With a .cargo/config file and the "runner" option set
cargo xrun

# To run the tests...
cargo xtests
# ...or
cargo xtests --test <name_of_module>
```
