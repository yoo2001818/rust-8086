; Test set 0x0000
; Before testing anything, we need to make sure we're not overwritting over
; code segment.
; Therefore, we have to check if segment registers are working correctly first.
seg_test_1:
mov ax, 0x2000
mov ss, ax
; Test if data segment is valid.
mov ax, 0x1000
mov ds, ax
mov WORD [0x0010], 0x5353
mov ax, 0x1001
mov ds, ax
mov WORD bx, [0x0000]
cmp WORD [0x0000], 0x5353
assert e, 0x0010
seg_test_2:
; Test if segment selector works.
mov ax, 0x1010
mov es, ax
mov WORD [es:0x0010], 0x5353
mov ax, 0x1011
mov es, ax
cmp WORD [es:0x0000], 0x5353
assert e, 0x0020
seg_test_3:
; Test if segment selector works - 2.
mov ax, 0x1000
mov ds, ax
mov ax, 0x1001
mov ss, ax
mov ax, 0x1002
mov es, ax
mov WORD [ds:0x0000], 0xaaaa
mov WORD [ds:0x0010], 0xbbbb
mov WORD [ds:0x0020], 0xcccc
cmp WORD [ds:0x0000], 0xaaaa
assert e, 0x0030
cmp WORD [ss:0x0000], 0xbbbb
assert e, 0x0031
cmp WORD [es:0x0000], 0xcccc 
assert e, 0x0032
