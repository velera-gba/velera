#[cfg(test)]
mod tests {
    use super::super::*;

    const TEST_ARRAYS_SIZE: usize = 4;
    const TEST_OPS_SIZE: usize = 2;

    const THUMB_TEST_ARRAY0: [u8; TEST_ARRAYS_SIZE] = [
        0b0001_1001,
        0b0010_0100,
        0b0001_1101,
        0b0010_0100,
    ];

    const THUMB_TEST_OPS_ADDSUB: [u16; TEST_OPS_SIZE] = [
        0b0001_1001_0010_0100,
        0b0001_1011_0010_0100
    ];

    const THUMB_TEST_OPS_ADDSUBI: [u16; TEST_OPS_SIZE] = [
        0b0011_0100_0000_0100,
        0b0011_1100_0000_0100
    ];

    const THUMB_TEST_MASKS_ADDSUB: [u16; TEST_OPS_SIZE] = [
        thumb_bitmasks::ADD,
        thumb_bitmasks::SUB
    ];

    #[test]
    fn test_thumb_decode_addsub() {
        let mut test_cpu: CPU = Default::default();
        for i in 0..TEST_OPS_SIZE {
            decode_thumb(&mut test_cpu, THUMB_TEST_OPS_ADDSUB[i]);
            assert_eq!(test_cpu.arm.temp_rd, 4);
            assert_eq!(test_cpu.arm.temp_rn, 4);
            assert_eq!(test_cpu.arm.temp_rs, 4);
            assert_eq!(test_cpu.arm.immediate, 0);
        }
    }

    #[test]
    fn test_thumb_decode_addsubi() {
        let mut test_cpu: CPU = Default::default();
        for i in 0..TEST_OPS_SIZE {
            decode_thumb(&mut test_cpu, THUMB_TEST_OPS_ADDSUBI[i]);
            assert_eq!(test_cpu.arm.temp_rd, 4);
            assert_eq!(test_cpu.arm.temp_rn, 0);
            assert_eq!(test_cpu.arm.temp_rs, 0);
            assert_eq!(test_cpu.arm.immediate, 4);
        }
    }

    #[test]
    fn test_pass_operation_thumb_addsub() {
        let mut test_cpu: CPU = Default::default();
        for i in 0..TEST_OPS_SIZE {
            let mut operation = false;
            pass_operation_thumb(
                &mut test_cpu,
                THUMB_TEST_OPS_ADDSUB[i],
                &mut operation,
                ThumbOpPack{
                    op_bitmask: THUMB_TEST_MASKS_ADDSUB[i],
                    opcode_bitmask: thumb_bitmasks::ADDSUB_OP_MASK,
                    rd_bitmask: thumb_bitmasks::ADDSUB_RD_MASK,
                    rs_bitmask: thumb_bitmasks::ADDSUB_RS_MASK,
                    rn_bitmask: thumb_bitmasks::ADDSUB_RN_MASK,
                    immediate_bitmask: 0
                }
            );
            assert_eq!(operation, true);
            assert_eq!(test_cpu.arm.temp_rd, 4);
            assert_eq!(test_cpu.arm.temp_rs, 4);
            assert_eq!(test_cpu.arm.temp_rn, 4);
            assert_eq!(test_cpu.arm.immediate, 0);
        }
    }

    #[test]
    fn test_put_temp_register_thumb() {
        let mut test_cpu: CPU = Default::default();
        for i in 0..TEST_OPS_SIZE {
            put_temp_register_thumb(
                &mut test_cpu.arm.temp_rd,
                thumb_bitmasks::ADDSUB_RD_MASK,
                THUMB_TEST_OPS_ADDSUB[i]
            );

            assert_eq!(test_cpu.arm.temp_rd, 4);

            put_temp_register_thumb(
                &mut test_cpu.arm.temp_rs,
                thumb_bitmasks::ADDSUB_RS_MASK,
                THUMB_TEST_OPS_ADDSUB[i]
            );

            assert_eq!(test_cpu.arm.temp_rs, 4);

            put_temp_register_thumb(
                &mut test_cpu.arm.temp_rn,
                thumb_bitmasks::ADDSUB_RN_MASK,
                THUMB_TEST_OPS_ADDSUB[i]
            );

            assert_eq!(test_cpu.arm.temp_rn, 4);
        }
    }
}