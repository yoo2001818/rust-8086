; Test set 0x0300
flags_test_1:
; Validate if lahf / sahf works.
mov ah, 0x80
sahf
assert s, 0x0300
mov ah, 0x00
lahf
cmp ah, 0x80
assert e, 0x0301
; Validate if pushf / popf works.
mov ax, 0x2000
mov ss, ax
mov ah, 0x80
sahf
pushf
cmp ah, 0x80
assert ns, 0x0302
popf
assert s, 0x0303
flags_test_2:
; Test clc
mov al, 0xff
add al, 0x01
clc
assert nc, 0x0310
; Test cmc
cmc
assert c, 0x0311
; Test cmc again
cmc
assert nc, 0x0312
; Test stc
stc
assert c, 0x0313
flags_test_3:
mov ax, 0x0000
push ax
popf
std
sti
pushf
pop ax
and ax, 0x0600
cmp ax, 0x0600
assert e, 0x0320
cld
cli
pushf
pop ax
and ax, 0x0600
cmp ax, 0x0000
assert e, 0x0321
sti
