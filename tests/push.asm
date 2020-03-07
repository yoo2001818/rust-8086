; Test set 0x0200
push_test_1:
mov ax, 0x2000
mov ss, ax
mov sp, 0xffff
; Validate PUSH AX
mov ax, 0xbbbb
push ax
; Validate if sp has decreased
mov dx, sp
cmp dx, 0xfffd
assert e, 0x0260
; Validate if correct value is stored
cmp WORD [ss:0xfffd], 0xbbbb
assert e, 0x0261
push_test_7:
; Validate PUSH BX
mov bx, 0xabcd
push bx
mov dx, sp
cmp dx, 0xfffb
assert e, 0x0270
cmp WORD [ss:0xfffb], 0xabcd
assert e, 0x0271
push_test_8:
; Validate PUSH R/M16
mov WORD [0xabcd], 0x2222
push WORD [bx]
mov dx, sp
cmp dx, 0xfff9
assert e, 0x0280
cmp WORD [ss:0xfff9], 0x2222
assert e, 0x0281
push_test_9:
; Validate PUSH R/M16
push WORD [0xabcd]
mov dx, sp
cmp dx, 0xfff7
assert e, 0x0290
cmp WORD [ss:0xfff7], 0x2222
assert e, 0x0291
push_test_10:
; Validate PUSH SEG
push cs
mov bx, cs
mov dx, sp
cmp dx, 0xfff5
assert e, 0x02a0
cmp WORD [ss:0xfff5], bx
assert e, 0x02a1
