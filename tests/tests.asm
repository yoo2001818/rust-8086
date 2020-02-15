CPU 8086
section .data
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
%include "push.asm"
end:
hlt
