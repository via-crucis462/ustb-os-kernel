// ref: https://elixir.bootlin.com/linux/v6.12.6/source/include/uapi/linux/auxvec.h
pub const AT_NULL: u8 = 0;
pub const AT_PHDR: u8 = 3;
pub const AT_PHENT: u8 = 4;
pub const AT_PHNUM: u8 = 5;
pub const AT_PAGESZ: u8 = 6;
pub const AT_BASE: u8 = 7;
pub const AT_ENTRY: u8 = 9;
pub const AT_RANDOM: u8 = 25;

// elf loader
pub const REL_GOT: u32 = 6;
pub const REL_PLT: u32 = 7;
pub const REL_RELATIVE: u32 = 8;
pub const R_RISCV_64: u32 = 2;
pub const R_RISCV_RELATIVE: u32 = 3;

