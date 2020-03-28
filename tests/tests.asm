CPU 8086
section .data
global _main 
%macro startdebug 0
  mov ax, 0x0001
  out 0x04, ax
%endmacro
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
%include "flags.asm"
%include "binary.asm"
startdebug
%include "call.asm"
end:
hlt
