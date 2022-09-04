use crate::cpu::cartridge::*;
use crate::cpu::Mem;

pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: Rom,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut pr_addr = addr - 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && pr_addr >= 0x4000 {
            pr_addr %= 0x4000;
        }
        self.rom.prg_rom[pr_addr as usize]
    }
}

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

impl Mem for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0x7ff;
                self.cpu_vram[mirror_down_addr as usize]
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0x2007;
                todo!("PPU is not supported yet")
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("Ignoring mem access at {:#04x}", addr);
                0
            }
        }
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0x7ff;
                self.cpu_vram[mirror_down_addr as usize] = data;
            }
            PPU_REGISTERS..=PPU_REGISTERS_MIRRORS_END => {
                let _mirror_down_addr = addr & 0x2007;
                todo!("PPU is not supported yet");
            }
            0x8000..=0xFFFF => {
                panic!("Attempt to write to Cartidge ROM space: {:#04x}", addr)
            }
            _ => {
                println!("Ignoring mem write-access at {:#04x}", addr);
            }
        }
    }
}
