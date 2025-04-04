#[derive(Debug, Clone)]
pub enum Instruction {
    OpCode(OpCode),
    RawByte(u8),
}

// TODO: Add opcodes
// See https://en.bitcoin.it/wiki/Script
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Constants
    OP_0 = 0x00,
    OP_PUSH1 = 0x51,
    OP_PUSH2 = 0x52,
    OP_PUSH3 = 0x53,
    OP_PUSH4 = 0x54,
    OP_PUSH5 = 0x55,
    OP_PUSH6 = 0x56,
    // TODO: Add other opcodes

    // Stack
    OP_DUP = 0x76,
    OP_DROP = 0x75,

    // Crypto
    OP_HASH160 = 0xA9,
    OP_EQUALVERIFY = 0x88,
    OP_CHECKSIG = 0xAC,
    OP_CHECKMULTISIG = 0xAE,

    // Flow
    OP_RETURN = 0x6A,
    // TODO: Add other opcodes
}
