#[allow(non_upper_case_globals)]
#[cfg(test)]
mod tests {
    use super::super::*;

    use crate::{
        arm::{
            decode::{
                branch, data_processing, data_transfer, interrupt, multiply, psr_transfer, swap,
                BaseInstruction,
            },
            DecodedInstruction,
        },
        enums::{MnemonicARM, ShiftType},
    };

    const cond: u8 = 0b0000;

    #[test]
    fn test_arm_decode_branch() {
        let instruction = 0b0000_1011_0000_1110_0011_1100_1111_1010;
        let result = branch(instruction, cond);

        assert_eq!(
            result,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::BX,
                val1: Some(1),
                offset: Some(0b0000_1110_0011_1100_1111_1010),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arm_decode_data_proc() {
        let instruction_add_register: u32 = 0b0000_0000_1000_1100_1010_0000_0001_0101;
        let instruction_sub_imm: u32 = 0b0000_0010_0101_1001_1100_0010_0001_1111;

        let result_add_register = data_processing(instruction_add_register, cond);
        let result_sub_imm = data_processing(instruction_sub_imm, cond);

        assert_eq!(
            result_add_register,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::ADD,
                rn: Some(0b1100),
                rd: Some(0b1010),
                set_cond: Some(false),
                rm: Some(0b0101),
                rs: Some(0),
                imm: Some(false),
                shift_type: Some(ShiftType::LSL),
                ..Default::default()
            }
        );

        assert_eq!(
            result_sub_imm,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::SUB,
                rn: Some(0b1001),
                rd: Some(0b1100),
                set_cond: Some(true),
                val1: Some(0b0010),
                val2: Some(0b0001_1111),
                imm: Some(true),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arm_decode_data_transfer() {
        let instruction_simple_load_register = 0b0000_0111_0011_0001_0100_0010_0100_1000;
        let result_simple_load_register = data_transfer(instruction_simple_load_register, cond);

        assert_eq!(
            result_simple_load_register,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::LDR,
                rn: Some(0b0001),
                rd: Some(0b0100),
                rm: Some(0b1000),
                val1: Some(0b1001),
                val2: Some(4),
                shift_type: Some(ShiftType::ASR),
                imm: Some(false),
                ..Default::default()
            }
        );

        let instruction_simple_store_imm = 0b0000_0101_1110_0001_0100_1100_1001_0100;
        let result_simple_store_imm = data_transfer(instruction_simple_store_imm, cond);

        assert_eq!(
            result_simple_store_imm,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::STR,
                rn: Some(0b0001),
                rd: Some(0b0100),
                val1: Some(0b1111),
                offset: Some(0b1100_1001_0100),
                imm: Some(true),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arm_decode_data_half() {
        let instruction_store_half_register = 0b0000_0001_1010_0111_1100_0000_1011_1110;
        let result_store_half_register = data_transfer(instruction_store_half_register, cond);
        assert_eq!(
            result_store_half_register,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::STRH,
                val1: Some(0b1101),
                rn: Some(0b0111),
                rd: Some(0b1100),
                rm: Some(0b1110),
                imm: Some(false),
                ..Default::default()
            }
        );

        let instruction_load_signed_half_imm = 0b0000_0001_0111_1011_1001_1001_1111_0110;
        let result_load_signed_half_imm = data_transfer(instruction_load_signed_half_imm, cond);
        assert_eq!(
            result_load_signed_half_imm,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::LDRSH,
                val1: Some(0b1011),
                rn: Some(0b1011),
                rd: Some(0b1001),
                offset: Some(0b1001_0110),
                imm: Some(true),
                ..Default::default()
            }
        )
    }

    #[test]
    fn test_arm_decode_data_byte() {
        let instruction_load_signed_byte_register = 0b0000_0001_1011_0011_1110_0000_1101_1001;
        let result_load_signed_byte_register =
            data_transfer(instruction_load_signed_byte_register, cond);

        assert_eq!(
            result_load_signed_byte_register,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::LDRSB,
                val1: Some(0b1101),
                rn: Some(0b0011),
                rd: Some(0b1110),
                rm: Some(0b1001),
                imm: Some(false),
                ..Default::default()
            }
        )
    }

    #[test]
    fn test_arm_decode_data_block() {
        let instruction_block_load = 0b0000_1000_1001_1110_0100_1111_1000_0100;
        let result_block_load = data_transfer(instruction_block_load, cond);

        assert_eq!(
            result_block_load,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::LDM,
                rn: Some(0b1110),
                val1: Some(0b0100),
                offset: Some(0b0100_1111_1000_0100),
                ..Default::default()
            }
        );

        let instruction_block_store = 0b0000_1001_0110_1101_0101_1101_1010_0111;
        let result_block_store = data_transfer(instruction_block_store, cond);

        assert_eq!(
            result_block_store,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::STM,
                rn: Some(0b1101),
                val1: Some(0b1011),
                offset: Some(0b0101_1101_1010_0111),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arm_decode_data_swap() {
        // test this on both the swap and the data_transfer function and check if they are equal
        let instruction_swap = 0b0000_0001_0100_1100_0010_0000_1001_1001;
        let result_swap = swap(instruction_swap, cond);
        let result_data_transfer = data_transfer(instruction_swap, cond);

        assert_eq!(
            result_swap,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::SWP,
                val1: Some(1),
                rn: Some(0b1100),
                rd: Some(0b0010),
                rm: Some(0b1001),
                ..Default::default()
            }
        );
        assert_eq!(result_swap, result_data_transfer);
    }

    #[test]
    fn test_arm_decode_psr_transfer() {
        let instruction_mrs = 0b0000_0001_0100_1111_0000_0000_0000_0000;
        let result_mrs = psr_transfer(instruction_mrs, cond);

        assert_eq!(
            result_mrs,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::MRS,
                val1: Some(1),
                rd: Some(0),
                imm: Some(false),
                ..Default::default()
            }
        );

        let instruction_msr_simple = 0b0000_0001_0010_1001_1111_0000_0000_0001;
        let result_msr_simple = psr_transfer(instruction_msr_simple, cond);

        assert_eq!(
            result_msr_simple,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::MSR,
                val1: Some(0),
                rm: Some(0b0001),
                imm: Some(false),
                ..Default::default()
            }
        );

        let instruction_msr_imm = 0b0000_0011_0010_1000_1111_1100_0010_1100;
        let result_msr_imm = psr_transfer(instruction_msr_imm, cond);

        assert_eq!(
            result_msr_imm,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::MSR,
                imm: Some(true),
                val1: Some(0),
                val2: Some(0b0010_1100),
                val3: Some(0b1100),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arm_decode_multiply() {
        let instruction_mul_simple = 0b0000_0000_0000_0011_1000_1100_1001_1110;
        let result_mul_simple = multiply(instruction_mul_simple, cond);

        assert_eq!(
            result_mul_simple,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::MUL,
                rd: Some(0b0011),
                rn: Some(0b1000),
                rs: Some(0b1100),
                rm: Some(0b1110),
                set_cond: Some(false),
                ..Default::default()
            }
        );

        let instruction_mul_unsigned_long_accumulate = 0b0000_0000_1111_1100_1001_0011_1001_0001;
        let result_mul_unsigned_long_accumulate =
            multiply(instruction_mul_unsigned_long_accumulate, cond);

        assert_eq!(
            result_mul_unsigned_long_accumulate,
            DecodedInstruction {
                cond: 0,
                instr: MnemonicARM::UMLAL,
                rd: Some(0b1100),
                rn: Some(0b1001),
                rs: Some(0b0011),
                rm: Some(0b0001),
                set_cond: Some(true),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_arm_decode_get_instr() {
        let instr_mul = 0b0000_0000_1111_1100_1001_0011_1001_0001;
        let result = BaseInstruction::get_instr(instr_mul);

        assert_eq!(result, BaseInstruction::Multiply);
    }
}
