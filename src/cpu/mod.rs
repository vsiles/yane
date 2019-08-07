mod flags;
pub mod cpu;
pub mod opcode;
pub mod nop;
#[macro_use]
mod macros;
#[macro_use]
mod load;
#[macro_use]
mod store;

pub use cpu::*;
pub use opcode::OpCode;

// Dummy Nop instruction used as a fallback
pub use nop::Nop;

// LDA, LDX, LDY
declare_load_imm!(LDAImm, A);
declare_load_imm!(LDXImm, X);
declare_load_imm!(LDYImm, Y);

declare_load_zero_page!(LDAZeroPage, A);
declare_load_zero_page!(LDXZeroPage, X);
declare_load_zero_page!(LDYZeroPage, Y);

declare_load_zero_page_reg!(LDAZeroPageX, A, X);
declare_load_zero_page_reg!(LDYZeroPageX, Y, X);
declare_load_zero_page_reg!(LDXZeroPageY, X, Y);

declare_load_abs!(LDAAbs, A);
declare_load_abs!(LDXAbs, X);
declare_load_abs!(LDYAbs, Y);

declare_load_abs_reg!(LDAAbsX, A, X);
declare_load_abs_reg!(LDAAbsY, A, Y);
declare_load_abs_reg!(LDXAbsY, X, Y);
declare_load_abs_reg!(LDYAbsX, Y, X);

declare_load_ndx_ind!(LDANdxInd, A);
declare_load_ind_ndx!(LDAIndNdx, A);

// STA, STX, STY
declare_store_zero_page!(STAZeroPage, A);
declare_store_zero_page!(STXZeroPage, X);
declare_store_zero_page!(STYZeroPage, Y);

declare_store_zero_page_reg!(STAZeroPageX, A, X);
declare_store_zero_page_reg!(STXZeroPageY, X, Y);
declare_store_zero_page_reg!(STYZeroPageX, Y, X);

declare_store_abs!(STAAbs, A);
declare_store_abs!(STXAbs, A);
declare_store_abs!(STYAbs, A);

declare_store_abs_reg!(STAAbsX, A, X);
declare_store_abs_reg!(STAAbsY, A, Y);

declare_store_ndx_ind!(STANdxInd, A);

declare_store_ind_ndx!(STAIndNdx, A);