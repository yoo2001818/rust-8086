CPU 8086
section .data
global _main 
mov WORD [0x5353], 0xabcd
mov WORD [0x2000], 0x8086
mov WORD bx, [0x2000]
mov WORD [bx], 0x5353
mov WORD bx, [bx]
%macro assert 2
  j%+1 %%success
  call %%fail
  %%fail:
    mov ax, %2
    out 0x00, ax
    hlt
  %%success:
%endmacro
_main:
%include "segment.asm"
%include "mov.asm"
%include "push.asm"
end:
hlt
