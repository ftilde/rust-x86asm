use instruction_def::*;
use test::run_test;
use Operand::*;
use Reg::*;
use RegScale::*;
use RegType::*;
use {BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};

#[test]
fn xsavec64_1() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::XSAVEC64,
            operand1: Some(Indirect(RDX, Some(OperandSize::Unsized), None)),
            operand2: None,
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[72, 15, 199, 34],
        OperandSize::Qword,
    )
}
