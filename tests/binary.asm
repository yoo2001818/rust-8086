; Test set 0x0400
binary_test_1:
; Test ADD / ADC
clc
mov ax, 0x0001
add ax, 0x0001
adc ax, 0x0001
cmp ax, 0x0003
assert e, 0x0400
stc
adc ax, 0x0001
cmp ax, 0x0005
assert e, 0x0401
; Test CF true
mov ax, 0xfff0
add ax, 0x0010
assert c, 0x0402
; Test CF false
mov ax, 0x8000
add ax, 0x1000
assert nc, 0x0403
; Test PF
mov ax, 0x0000
add ax, 0x1111
assert p, 0x0404
add ax, 0x0001
assert p, 0x0405
add ax, 0x0001
assert np, 0x0406
; Test CF, PF, AF, ZF, SF, OF
