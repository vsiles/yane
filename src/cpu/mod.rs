mod flags;
pub mod cpu;
pub mod opcode;
pub mod nop;
#[macro_use]
mod macros;
#[macro_use]
mod load;

pub use cpu::*;
pub use opcode::OpCode;

// Dummy Nop instruction used as a fallback
pub use nop::Nop;

// LDA, LDX, LDY
declare_load_imm!(LDAImm, a);
declare_load_imm!(LDXImm, x);
declare_load_imm!(LDYImm, y);

declare_load_zero_page!(LDAZeroPage, a);
declare_load_zero_page!(LDXZeroPage, x);
declare_load_zero_page!(LDYZeroPage, y);

declare_load_zero_page_reg!(LDAZeroPageX, a, x);
declare_load_zero_page_reg!(LDYZeroPageX, y, x);
declare_load_zero_page_reg!(LDXZeroPageY, x, y);

declare_load_abs!(LDAAbs, a);
declare_load_abs!(LDXAbs, x);
declare_load_abs!(LDYAbs, y);

declare_load_abs_reg!(LDAAbsX, a, x);
declare_load_abs_reg!(LDAAbsY, a, y);
declare_load_abs_reg!(LDXAbsY, x, y);
declare_load_abs_reg!(LDYAbsX, y, x);

declare_load_ndx_ind!(LDANdxInd, a);
declare_load_ind_ndx!(LDAIndNdx, a);
