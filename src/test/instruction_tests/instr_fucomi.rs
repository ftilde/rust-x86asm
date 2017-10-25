use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fucomi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 236], OperandSize::Word)
}

#[test]
fn fucomi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 237], OperandSize::Dword)
}

#[test]
fn fucomi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 236], OperandSize::Qword)
}
