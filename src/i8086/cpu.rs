use crate::mem;
use super::register;

pub struct CPU {
  memory: mem::LinearMemory,
  register: register::Register,
}

impl CPU {
  pub fn new(memory: mem::LinearMemory) -> Self {
    CPU { memory, register: register::Register::new() }
  }
  pub fn next() -> () {
    // Retrieve next opcode (which is a 16bit word), and parse them.
    // We have to enumerate all opcodes here.
    // MOD REG R/M byte directs where to read from / write to, which can be
    // a register, or memory address.
    //
    // When MOD = 00: Memory mode, no displacement follows
    // MOV
    // PUSH
    // POP
    // XCHG
    // IN
    // OUT
    // XLAT
    // LEA
    // LDS
    // LES
    // LAHF
    // SAHF
    // PUSHF
    // POPF
    // ADD
    // ADC
    // INC
    // AAA
    // DAA
    // SUB
    // SBB
    // DEC
    // NEG
    // CMP
    // AAS
    // DAS
    // MUL
    // IMUL
    // AAM
    // DIV
    // IDIV
    // AAD
    // CSW
    // CWD
    // NOT
    // SHL/SAL
    // SHR
    // SAR
    // ROL
    // ROR
    // RCL
    // RCR
    // AND
    // TEST
    // OR
    // XOR
    // REP
    // MOVS
    // CMPS
    // SCAS
    // LODS
    // STDS
    // CALL
    // JMP
    // RET
    // JE / JZ
    // JL / JNGE
    // JLE / JNG
    // JB / JNAE
    // JBE / JNA
    // JP / JPE
    // JO
    // JS
    // JNE / JNZ
    // JNL / JGE
    // JNLE / JG
    // JNB / JAE
    // JNBE / JA
    // JNP / JPO
    // JNO
    // RET
    // JNS
    // LOOP
    // LOOPZ / LOOPE
    // LOOPNZ / LOOPNE
    // JCXZ
    // INT
    // INTO
    // IRET
    // CLC
    // CMC
    // STC
    // CLD
    // STD
    // CLI
    // STI
    // HLT
    // WAIT
    // ESC
    // LOCK
    // SEGMENT
  }
}
