push_test_1:
mov bp, 0xffff
cmp bp, 0xffff
assert e, 0x0010
push_test_2:
mov sp, 0xffff
cmp sp, 0xffff
assert e, 0x0020
push_test_3:
mov ax, 0xbbbb
cmp ax, 0xbbbb
assert e, 0x0030
push_test_4:
mov bx, 0xabcd
cmp bx, 0xabcd
assert e, 0x0040
push_test_5:
mov WORD [0xabcd], 0x2222
cmp WORD [0xabcd], 0x2222
assert e, 0x0050
push_test_6:
push ax
mov dx, sp
cmp dx, 0xfffd
assert e, 0x0060
cmp WORD [0xfffd], 0xbbbb
assert e, 0x0061
push_test_7:
push bx
mov dx, sp
cmp dx, 0xfffb
assert e, 0x0070
cmp WORD [0xfffb], 0xabcd
assert e, 0x0071
push_test_8:
push WORD [bx]
mov dx, sp
cmp dx, 0xfff9
assert e, 0x0080
cmp WORD [0xfff9], 0x2222
assert e, 0x0081
push_test_9:
push WORD [0xabcd]
mov dx, sp
cmp dx, 0xfff7
assert e, 0x0090
cmp WORD [0xfff7], 0x2222
assert e, 0x0091
push_test_10:
push cs
mov dx, sp
cmp dx, 0xfff5
assert e, 0x00a0
cmp WORD [0xfff5], 0x0010
assert e, 0x00a1
