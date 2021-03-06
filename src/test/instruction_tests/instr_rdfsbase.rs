use instruction_def::*;
use test::run_test;
use Operand::*;
use Reg::*;
use RegScale::*;
use RegType::*;
use {BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};

#[test]
fn rdfsbase_1() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::RDFSBASE,
            operand1: Some(Direct(EDX)),
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
        &[243, 15, 174, 194],
        OperandSize::Qword,
    )
}

#[test]
fn rdfsbase_2() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::RDFSBASE,
            operand1: Some(Direct(RBP)),
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
        &[243, 72, 15, 174, 197],
        OperandSize::Qword,
    )
}
