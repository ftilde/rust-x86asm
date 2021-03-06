use instruction_def::*;
use test::run_test;
use Operand::*;
use Reg::*;
use RegScale::*;
use RegType::*;
use {BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};

#[test]
fn pmovzxwq_1() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::PMOVZXWQ,
            operand1: Some(Direct(XMM4)),
            operand2: Some(Direct(XMM7)),
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[102, 15, 56, 52, 231],
        OperandSize::Dword,
    )
}

#[test]
fn pmovzxwq_2() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::PMOVZXWQ,
            operand1: Some(Direct(XMM1)),
            operand2: Some(IndirectScaledDisplaced(
                ESI,
                Eight,
                68842024,
                Some(OperandSize::Dword),
                None,
            )),
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[102, 15, 56, 52, 12, 245, 40, 114, 26, 4],
        OperandSize::Dword,
    )
}

#[test]
fn pmovzxwq_3() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::PMOVZXWQ,
            operand1: Some(Direct(XMM7)),
            operand2: Some(Direct(XMM2)),
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[102, 15, 56, 52, 250],
        OperandSize::Qword,
    )
}

#[test]
fn pmovzxwq_4() {
    run_test(
        &Instruction {
            mnemonic: Mnemonic::PMOVZXWQ,
            operand1: Some(Direct(XMM2)),
            operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)),
            operand3: None,
            operand4: None,
            lock: false,
            rounding_mode: None,
            merge_mode: None,
            sae: false,
            mask: None,
            broadcast: None,
        },
        &[102, 15, 56, 52, 16],
        OperandSize::Qword,
    )
}
