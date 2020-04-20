# $< : input of the recipe
# $@ : output of the recipe

BOOTLOADER_SRCS := $(wildcard *.asm)
BOOTLOADER_OBJS := $(patsubst %.asm, %.o, $(BOOTLOADER_SRCS))

default: disk

%.o: %.asm
	#nasm -f bin $< -o $@
	nasm -f elf $< -F dwarf -g -o $@                # enable debug information
	ld -m elf_i386 -T bootloader.lds $@ -o $@.elf   # uses our linker script
	objcopy -O binary bootloader.o.elf $@           # construct a flat binary from the ELF

disk: $(BOOTLOADER_OBJS)
	dd if=/dev/zero of=disk.img bs=512 count=2880
	dd conv=notrunc if=bootloader.o of=disk.img bs=512 count=1 seek=0
	dd if=kernel.o of=disk.img bs=512 count=1 seek=1

run:
	qemu-system-i386 -machine q35 -fda disk.img -gdb tcp::26000 -S

clean:
	rm *.o
	rm disk.img
