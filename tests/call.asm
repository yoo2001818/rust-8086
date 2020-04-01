; Test set 0x0500
call_test_1:
; We should test call, ret, int
mov sp, 0xffff
mov ax, 0x1000
call call_test_1_func
cmp ax, 0x1001
assert e, 0x0500
mov cx, call_test_1_func
call cx
cmp ax, 0x1002
assert e, 0x0501
jmp call_test_2
call_test_1_func:
inc ax
ret
call_test_2:
; Test ret with pop
mov sp, 0xffff
sub sp, 4
call call_test_2_func
cmp sp, 0xffff
assert e, 0x0502
; Test jmp far
mov cx, call_test_3
jmp cx
call_test_2_func:
ret 4
call_test_3:
