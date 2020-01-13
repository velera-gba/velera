macro_rules! temp_reg_wrap {
    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr) => {
        pass_operation_thumb($cpu, $instruction, $operation, ThumbOpPack {
            op_bitmask: $x,
            opcode_bitmask: $opcode,
            rd_bitmask: $rd,
            rs_bitmask: $rs,
            rn_bitmask: $rn,
            immediate_bitmask: $immediate
        });
    };

    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr,
    $($xs: expr),*) => {
        temp_reg_wrap!($cpu, $instruction, $operation, $opcode, $rd, $rs, $rn, $immediate, $x);
        temp_reg_wrap!($cpu, $instruction, $operation, $opcode, $rd, $rs, $rn, $immediate, $($xs),*)
    };
}
