CPU 8086
global _main 
%macro assert 2
  j%+1 %%success
  call %%fail
  %%fail:
    mov ax, %2
    out 0x01, ax
  %%success:
%endmacro
_main:
push_test_1:
mov bp, 0xffff
test bp, 0xffff
assert e, 0x0001
push_test_2:
mov sp, 0xffff
test sp, 0xffff
assert e, 0x0002
push_test_3:
mov ax, 0xbbbb
test ax, 0xbbbb
assert e, 0x0003
push_test_4:
mov bx, 0xabcd
test bx, 0xabcd
assert e, 0x0004
push_test_5:
mov WORD [0xabcd], 0x2222
test WORD [0xabcd], 0x2222
assert e, 0x0005
push_test_6:
push ax
mov dx, sp
test dx, 0xfffd
assert e, 0x0006
test WORD [0xfffd], 0xbbbb
assert e, 0x0006
push_test_7:
push bx
mov dx, sp
test dx, 0xfffb
assert e, 0x0007
test WORD [0xfffb], 0xabcd
assert e, 0x0007
push_test_8:
push WORD [bx]
mov dx, sp
test dx, 0xfff9
assert e, 0x0008
test WORD [0xfff9], 0x2222
assert e, 0x0008
push_test_9:
push WORD [0xabcd]
mov dx, sp
test dx, 0xfff7
assert e, 0x0009
test WORD [0xfff7], 0x2222
assert e, 0x0009
push_test_10:
push cs
mov dx, sp
test dx, 0xfff5
assert e, 0x000a
test WORD [0xfff5], 0x0000
assert e, 0x000a
end:
hlt
