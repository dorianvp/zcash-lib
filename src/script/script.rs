use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::address::transparent::TransparentAddress;

use super::opcode::{Instruction, OpCode};

#[derive(Debug, Clone, Default)]
pub struct Script(pub Vec<Instruction>);

impl Script {
    pub fn from_bytes(script: Vec<Instruction>) -> Self {
        Self(script)
    }

    pub fn as_bytes(&self) -> &[Instruction] {
        &self.0
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0
            .iter()
            .map(|instr| match instr {
                Instruction::OpCode(op) => *op as u8,
                Instruction::RawByte(b) => *b,
            })
            .collect()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push_opcode(&mut self, opcode: OpCode) {
        self.0.push(Instruction::OpCode(opcode));
    }

    pub fn push_raw(&mut self, data: &[Instruction]) {
        self.0.extend_from_slice(data);
    }

    pub fn p2pkh(pubkey_hash: [u8; 20]) -> Self {
        let mut script = Vec::<Instruction>::with_capacity(25);
        script.push(Instruction::OpCode(OpCode::OP_DUP));
        script.push(Instruction::OpCode(OpCode::OP_HASH160));
        script.push(Instruction::RawByte(0x14)); // push 20 bytes
        script.extend(pubkey_hash.iter().map(|b| Instruction::RawByte(*b)));
        script.push(Instruction::OpCode(OpCode::OP_EQUALVERIFY));
        script.push(Instruction::OpCode(OpCode::OP_CHECKSIG));
        Script(script)
    }

    pub fn p2sh_redeem_script(bytes: &[u8]) -> Self {
        Script(
            bytes
                .to_vec()
                .iter()
                .map(|b| Instruction::RawByte(*b))
                .collect(),
        )
    }

    pub fn to_p2sh_address(&self) -> TransparentAddress {
        let sha256 = Sha256::digest(&self.to_bytes());
        let ripemd = Ripemd160::digest(sha256);

        let mut hash = [0u8; 20];
        hash.copy_from_slice(&ripemd);

        TransparentAddress::P2SH(hash)
    }
}
