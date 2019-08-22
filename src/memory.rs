pub struct Memory {
    data: Vec<u8>,
}

// General info
// $0000-$07FF 	$0800 	2KB internal RAM
// $0800-$0FFF 	$0800 	Mirrors of $0000-$07FF
// $1000-$17FF 	$0800
// $1800-$1FFF 	$0800
// $2000-$2007 	$0008 	NES PPU registers
// $2008-$3FFF 	$1FF8 	Mirrors of $2000-2007 (repeats every 8 bytes)
// $4000-$4017 	$0018 	NES APU and I/O registers
// $4018-$401F 	$0008 	APU and I/O functionality that is normally disabled. See CPU Test Mode.
// $4020-$FFFF 	$BFE0 	Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)
fn get_addr(addr: u16) -> (usize, bool) {
    if addr < 0x2000 {
        let mut local = (addr as usize) & 0xFFF;
        if local > 0x800 {
            local = local - 0x800;
        }
        return (local, true);
    }
    if addr < 0x4000 {
        panic!("Implement PPU memmapped registers: {:#x}", addr)
    }
    if addr < 0x4020 {
        // http://wiki.nesdev.com/w/index.php/2A03
        panic!("Implement APU - I/O memmapped registers: {:#x}", addr)
    }
    // TODO: atm Mapper 0 with 16k duplicated only hardwired :(
    if addr < 0x6000 {
        panic!("Mapper0 deadzone: {:#x}", addr)
    }
    if addr < 0x8000 {
        return (addr as usize, true);
    }
    if addr >= 0xC000 {
        return ((addr as usize) - (0xC000 - 0x8000), false)
    } else {
        return (addr as usize, false)
    }
}

impl Memory {
    pub fn get(&self, addr: u16) -> u8 {
        let real_addr = get_addr(addr);
        // println!("DEBUG reading at {:#X} (which is {:#X})", addr, real_addr.0);
        return self.data[real_addr.0]
    }

    pub fn set(&mut self, addr: u16, val: u8) {
        let (pos, rw) = get_addr(addr);
        if !rw {
            panic!("Trying to write {:#x} to RO address {:#x}", val, addr);
        }
        // println!("DEBUG setting {:#X} to {:#X}", pos, val);
        self.data[pos] = val
    }

    pub fn new(rom: Vec<u8>) -> Memory {
        // Create 16k address space
        let mut raw_data = Vec::with_capacity(0x10000 as usize);
        // Set 0s for the first 32k
        for _ in 0 ..(0x8000 as usize) {
            raw_data.push(0)
        }
        // Populate the Mapper 0 rom  at 0x8000
        for i in 0..rom.len() {
            raw_data.push(rom[i])
        }

        Memory {
            data: raw_data,
        }
    }

    pub fn dummy() -> Memory {
        Memory {
            data: Vec::new(),
        }
    }
}
