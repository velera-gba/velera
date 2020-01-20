// wrap temporary register operations for THUMB decoding
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
        // if *$operation {
        //     return;
        // }
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

// statement, returns (), best used in an IF or MATCH statement
macro_rules! compare_opcodes_stmt {
    ($instruction: expr,
    $op_bitmask: expr,
    $opcode_bitmask: expr) => {
        $instruction & $opcode_bitmask == $opcode_bitmask
    };
}