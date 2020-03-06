extern crate arrayvec;
extern crate byteorder;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate matches;

mod decoding;
mod encoding;
mod instruction;
mod instruction_buffer;
mod instruction_def;
mod instruction_defs;
mod mnemonic;
mod operand;
#[cfg(test)]
mod test;

pub use self::decoding::{InstructionDecodingError, InstructionReader};
pub use self::encoding::{InstructionEncodingError, InstructionWriter};
pub use self::instruction::{
    BroadcastMode, Instruction, MaskReg, MergeMode, Reg, RegScale, RegType, RoundingMode,
    SegmentReg,
};
pub use self::mnemonic::Mnemonic;
pub use self::operand::{Operand, OperandSize};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Real,
    Protected,
    Long,
}

impl Mode {
    fn pointer_size(&self) -> OperandSize {
        match *self {
            Mode::Real => OperandSize::Word,
            Mode::Protected => OperandSize::Dword,
            Mode::Long => OperandSize::Qword,
        }
    }

    fn from_size(size: OperandSize) -> Option<Mode> {
        match size {
            OperandSize::Word => Some(Mode::Real),
            OperandSize::Dword => Some(Mode::Protected),
            OperandSize::Qword => Some(Mode::Long),
            _ => None,
        }
    }
}
