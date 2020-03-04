; Before testing anything, we need to make sure we're not overwritting over
; code segment.
; Therefore, we have to check if segment registers are working correctly first.
seg_test_1:
; We expect code segment to be located in 0000h.... Though it doesn't matter.
; Test if data segment is valid.
mov ax, 0x2000
push ax 
pop ss
mov ax, 0x1000
push ax
pop ds
mov WORD [0x0010], 0x5353
mov ax, 0x1001
push ax
pop ds
cmp WORD [0x0001], 0x5353
assert e, 0x0010
seg_test_2:
; Test if segment selector works.
mov ax, 0x1010
push ax
pop es
mov WORD [es:0x0010], 0x5353
mov ax, 0x1011
push ax
pop es
cmp WORD [es:0x0001], 0x5353
assert e, 0x0020
seg_test_3:
; Test if segment selector works - 2.
mov ax, 0x1000
push ax
pop ds
mov ax, 0x1001
push ax
pop ss
mov ax, 0x1002
push ax
pop es
mov WORD [ds:0x0000], 0xaaaa
mov WORD [ds:0x0010], 0xbbbb
mov WORD [ds:0x0020], 0xcccc
cmp WORD [ds:0x0000], 0xaaaa
assert e, 0x0030
cmp WORD [ss:0x0000], 0xbbbb
assert e, 0x0031
cmp WORD [es:0x0000], 0xcccc 
assert e, 0x0032
