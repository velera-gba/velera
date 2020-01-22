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
    $x: expr => $func: block) => {
        // saves temporary data for execution
        if $cpu.execution_queue.is_empty() {
            pass_operation_thumb($cpu, $instruction, $operation, ThumbOpPack {
                op_bitmask: $x,
                opcode_bitmask: $opcode,
                rd_bitmask: $rd,
                rs_bitmask: $rs,
                rn_bitmask: $rn,
                immediate_bitmask: $immediate
            });

            // todo: enqueue functions for next execution
        }
    };

    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr => $func: block,
    $($xs: expr => $funcs: block),*) => {
        temp_reg_wrap!($cpu, $instruction, $operation, $opcode, $rd, $rs, $rn, $immediate, $x => $func);
        temp_reg_wrap!($cpu, $instruction, $operation, $opcode, $rd, $rs, $rn, $immediate, $($xs => $funcs),*)
    };
}

/*
macro_rules! thumb_execute_wrap {
    ($cpu: expr,
    $instruction: expr,
    $opcode_bitmask: expr,
    $x: expr,
    $execution: expr) => {
        if $instruction & $opcode_bitmask == $x {
            $execution($cpu);
        }
    };

    ($cpu: expr,
    $instruction: expr,
    $opcode_bitmask: expr,
    $x: expr,
    $execution: expr,
    $($xs: expr, $exs: expr),*) => {
        thumb_execute_wrap!($cpu, $instruction, $opcode_bitmask, $x, $execution);
        thumb_execute_wrap!($cpu, $instruction, $opcode_bitmask, $($xs, $exs),*)
    }
}
*/