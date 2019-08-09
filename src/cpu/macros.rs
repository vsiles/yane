macro_rules! execute_load {
    ($reg:ident, $opcode:ident, $cpu:ident) => {{
        let imm = $opcode.imm;
        $cpu.$reg = imm;
        $cpu.flags.zero = imm == 0;
        $cpu.flags.negative = (imm & (0x80 as u8)) != 0
    }};
}

macro_rules! execute_store {
    ($reg:ident, $addr:expr, $cpu:ident) => {{
        $cpu.mem[$addr as usize] = $cpu.$reg
    }};
}

macro_rules! mk_addr {
    ($low:expr, $high:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}
