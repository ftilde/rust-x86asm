use instruction_def::*;
use test::run_test;
use Operand::*;
use Reg::*;
use RegScale::*;
use RegType::*;
use {BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};

#[test]
fn knotd_1() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::KNOTD,
            operand1: Some(Direct(K2)),
            operand2: Some(Direct(K1)),
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[196, 225, 249, 68, 209],
        OperandSize::Dword,
    )
}

#[test]
fn knotd_2() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::KNOTD,
            operand1: Some(Direct(K2)),
            operand2: Some(Direct(K5)),
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[196, 225, 249, 68, 213],
        OperandSize::Qword,
    )
}
