// wrap temporary register operations for THUMB decoding
macro_rules! temp_reg_wrap {
    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $queue: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr => $func: block) => {
        // saves temporary data for execution
        if $cpu.execution_queue.is_empty() {
            if pass_operation_thumb($cpu, $instruction, $operation, ThumbOpPack {
            op_bitmask: $x,
            opcode_bitmask: $opcode,
            rd_bitmask: $rd,
            rs_bitmask: $rs,
            rn_bitmask: $rn,
            immediate_bitmask: $immediate}) {
                // enqueue functions for next execution
                enqueue_operation!($queue, $func);
            }
        }

        if *$operation {
            return $queue;
        }
    };

    ($cpu: expr,
    $instruction: expr,
    $operation: expr,
    $queue: expr,
    $opcode: expr,
    $rd: expr,
    $rs: expr,
    $rn: expr,
    $immediate: expr,
    $x: expr => $func: block,
    $($xs: expr => $funcs: block),*) => {
        temp_reg_wrap!($cpu, $instruction, $operation, $queue, $opcode, $rd, $rs, $rn, $immediate, $x => $func);
        temp_reg_wrap!($cpu, $instruction, $operation, $queue, $opcode, $rd, $rs, $rn, $immediate, $($xs => $funcs),*)
    };
}

macro_rules! enqueue_operation {
    ($queue: expr,
    $func: expr) => {
        $queue.push_back($func)
    };

    ($queue: expr,
    $func: expr,
    $($funcs: expr),*) => {
        enqueue_operation!($queue, $func);
        enqueue_operation!($queue, $($funcs),*)
    }
}
