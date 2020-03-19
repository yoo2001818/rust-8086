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
; Test AF
mov bx, 0x0006
add bx, 0x0001
lahf
test ah, 0x0010
assert z, 0x0407
add bx, 0x0007
lahf
test ah, 0x0010
assert nz, 0x0408
; Test ZF
mov ax, 0x0000
add ax, 0x0000
assert z, 0x0409
add ax, 0x0001
assert nz, 0x040a
; Test SF
mov ax, 3
add ax, -4
assert s, 0x040b
add ax, 6
assert ns, 0x040c
; Test OF
mov ax, 0x7fff
add ax, 0x0001
assert o, 0x040d
add ax, 0x0002
assert no, 0x040e
binary_test_2:
; Test bytes
mov al, 0x01
add al, 0x01
adc al, 0x01
cmp al, 0x03
assert e, 0x0410
stc
adc al, 0x01
cmp al, 0x05
assert e, 0x0411
; Test CF true
mov al, 0xf0
add al, 0x10
assert c, 0x0412
; Test CF false
mov al, 0x80
add al, 0x10
assert nc, 0x0413
; Test PF
mov al, 0x00
add al, 0x11
assert p, 0x0414
add al, 0x01
assert p, 0x0415
add al, 0x01
assert np, 0x0416
; Test AF
mov bl, 0x06
add bl, 0x01
lahf
test ah, 0x10
assert z, 0x0417
add bl, 0x07
lahf
test ah, 0x10
assert nz, 0x0418
; Test ZF
mov al, 0x00
add al, 0x00
assert z, 0x0419
add al, 0x01
assert nz, 0x041a
; Test SF
mov al, 3
add al, -4
assert s, 0x041b
add al, 6
assert ns, 0x041c
; Test OF
mov al, 0x7f
add al, 0x01
assert o, 0x041d
add al, 0x02
assert no, 0x041e
binary_test_3:
mov al, 0x01
mov byte [0x1000], 0x05
; Test b, f, r/m
add byte [0x1000], al ; 6
; Test b, t, r/m
add byte al, [0x1000] ; 7
; Test b, ia
add byte al, 0x01 ; 8
; Test b, r/m, imm
add byte [0x1000], 0x01 ; 7
cmp al, 0x08
assert e, 0x0420
cmp byte [0x1000], 0x07
assert e, 0x0421
mov ax, 0x01
mov word [0x1000], 0x05
; Test w, f, r/m
add word [0x1000], ax ; 6
; Test w, t, r/m
add word ax, [0x1000] ; 7
; Test w, ia
add word ax, 0x01 ; 8
; Test w, r/m, imm
add word [0x1000], 0x01 ; 7
cmp ax, 0x08
assert e, 0x0420
cmp word [0x1000], 0x07
assert e, 0x0421
binary_test_4:
; Subtract test
mov ax, 0x0010
sub ax, 0x0008
cmp ax, 0x0008
assert e, 0x0430
; Test CF true
mov ax, 0x0001
sub ax, 0x0002
assert c, 0x0431
; Test CF false
mov ax, 0x0010
sub ax, 0x0008
assert nc, 0x0432
; Test PF
mov ax, 0x000f
sub ax, 0x0001 ; 0xe
assert np, 0x0433
sub ax, 0x0001 ; 0xd
assert p, 0x0434
; Test AF
; Test ZF
; Test SF
; Test OF
