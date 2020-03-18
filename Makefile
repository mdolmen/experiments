# $< : input of the recipe
# $@ : output of the recipe

BOOTLOADER_SRCS := $(wildcard *.asm)
BOOTLOADER_OBJS := $(patsubst %.asm, %.o, $(BOOTLOADER_SRCS))

default: disk

%.o: %.asm
	nasm -f bin $< -o $@

disk: $(BOOTLOADER_OBJS)
	dd if=/dev/zero of=disk.img bs=512 count=2880
	dd conv=notrunc if=bootloader of=disk.img bs=512 count=1 seek=0
	dd if=kernel of=disk.img bs=512 count=1 seek=1

run:
	qemu-system-i386 -machine q35 -fda disk.img -gdb tcp::26000 -S
