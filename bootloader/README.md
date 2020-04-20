Hello World Bootloader
----------------------

Hello world bootloader to follow along this book: https://github.com/tuhdo/os01 .

```bash
make
make run
```

In another terminal:

```bash
gdb -q bootloader.o

(gdb) target remote :26000
```
