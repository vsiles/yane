macro_rules! execute_load {
    ($reg:ident, $opcode:ident, $cpu:ident) => {{
        let imm = $opcode.imm;
        $cpu.$reg = imm;
        $cpu.flags.zero = imm == 0;
        $cpu.flags.negative = (imm & (0x80 as u8)) != 0
    }};
}

macro_rules! mk_addr {
    ($low:expr, $high:expr) => {
        (($high as u16) << 8) | ($low as u16)
    };
}

macro_rules! push {
    ($cpu:ident, $val:expr) => {{
        let sp: u16 = mk_addr!($cpu.sp, 0x01);
        $cpu.mem.set(sp, $val as u8);
        let (sp, _) = $cpu.sp.overflowing_sub(1);
        $cpu.sp = sp;
    }};
}
