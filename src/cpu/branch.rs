macro_rules! declare_branch {
    ($name:ident, $flag:ident, $val:expr, $mnemo:ident) => {
        pub struct $name {
            imm: i8,
            state: usize,
        }

        impl OpCode for $name {
            fn new() -> $name {
                $name {
                    imm: 0,
                    state: 0,
                }
            }
            //   2     PC      R  fetch operand, increment PC
            //   3     PC      R  Fetch opcode of next instruction,
            //                    If branch is taken, add operand to PCL.
            //                    Otherwise increment PC.
            //   4+    PC*     R  Fetch opcode of next instruction.
            //                    Fix PCH. If it did not change, increment PC.
            //   5!    PC      R  Fetch opcode of next instruction,
            //                    increment PC.

            //  Notes: The opcode fetch of the next instruction is included to
            //         this diagram for illustration purposes. When determining
            //         real execution times, remember to subtract the last
            //         cycle.

            //         * The high byte of Program Counter (PCH) may be invalid
            //           at this time, i.e. it may be smaller or bigger by $100.

            //         + If branch is taken, this cycle will be executed.

            //         ! If branch occurs to different page, this cycle will be
            //           executed.

            fn decode(&mut self, cpu: &mut Cpu) -> bool {
                if self.state == 0 {
                    self.imm = cpu.read_from_pc() as i8;
                    self.state = 1;
                    // if  test is false, stop right here (cycle 2)
                    cpu.flags.$flag != $val
                } else if self.state == 1 {
                    self.state = 2;
                    let pc : i32 = cpu.pc as i32;
                    let target = (pc + (self.imm as i32)) as u16;
                    let p0 = (cpu.pc >> 8) & 0x1;
                    let p1 = (target >> 8) & 0x1;
                    cpu.pc = target;
                    // if no page crossing, stop right here (cycle 3)
                    p0 == p1
                } else {
                    // test is taken  _and_ page crossing (cycle 4)
                    true
                }
            }

            fn log(&self, cpu: &Cpu) {
                let pc = cpu.pc - 1;
                let code = cpu.mem.get(pc);
                let imm = cpu.mem.get(pc + 1) as i8;
                let addr : i32 = (pc as i32) + (imm as i32) + 2;
                print!(
                    "{:04X}  {:02X} {:02X}     {} ${:04X}",
                    pc,
                    code,
                    imm,
                    stringify!($mnemo),
                    addr as u16
                );
                print!("{: <23}{}", "", cpu)
            }
        }
    };
}
