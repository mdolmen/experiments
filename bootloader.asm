;
; Very basic bootloader. Taken from "Operating System From 0 to 1" book.
;
; Source (to print text): https://stackoverflow.com/questions/36847806/printing-a-string-of-characters-from-a-boot-sector-only-displays-first-character
;

org 0x7c00
bits 16

start:
    jmp boot

msg:
    db "Welcome from my bootloader!", 0x0a, 0x0d, 0x00

boot:
    cli             ; no interrupts
    cld             ; all that we need to init
    xor ax, ax
    mov ds, ax
    mov ah, 0x0e    ; BIOS call number for writing text
    mov si, msg

print:
    lodsb           ; load byte at DS into AL, increment SI
    or al, al       ; check if AL is 0, set the ZF flag
    jz load_kernel
    int 0x10        ; do the print call
    jmp print

load_kernel:
    mov ax, 0x50
    
    ; set the buffer
    mov es, ax
    xor bx, bx
    mov al, 2       ; read 2 sector
    mov ch, 0       ; track 0
    mov cl, 2       ; sector to read (the second one)
    mov dh, 0       ; head number
    mov dl, 0       ; drive number
    mov ah, 0x02    ; read sectors from disk
    int 0x13        ; call the BIOS routine
    jmp 0x500       ; jump and execute sector

hltloop:
    hlt ; halt the system
    jmp hltloop

; We have to be 512 bytes. Clear the rest of the bytes with 0.
times 510 - ($-$$) db 0
dw 0xAA55 ; boot signature
