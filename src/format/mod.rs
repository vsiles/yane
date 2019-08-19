pub mod ines {
    enum Mirroring {
        Horizontal,
        Vertival,
    }

    impl std::fmt::Display for Mirroring {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Mirroring::Horizontal => write!(f, "Horizontal"),
                Mirroring::Vertival => write!(f, "Vertical"),
            }
        }
    }

    enum TvSystem {
        NTSC,
        #[allow(dead_code)]
        PAL,
    }

    pub struct Header {
        pub prg_rom_size: usize,
        chr_rom_size: usize,
        // Flags 6
        mirroring: Mirroring,
        has_battery: bool,
        has_trainer: bool,
        ignore_mirroring: bool,
        pub mapper: usize,
        // Flags 7
        #[allow(dead_code)]
        vs_unisystem: bool,
        #[allow(dead_code)]
        play_choice_10: bool,
        // Flags 8
        #[allow(dead_code)]
        prg_ram_size: usize,
        // Flags 9
        #[allow(dead_code)]
        tv_system: TvSystem,
        // Flags 10
        // TODO check secondary TV System 
        #[allow(dead_code)]
        prg_ram: bool,
        #[allow(dead_code)]
        bus_conflicts: bool,
    }

    impl std::fmt::Display for Header {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  PRG ROM SIZE: {:#x} bytes\n  CHR ROM SIZE: {:#x} bytes\n  M: {} B: {} T: {} IM: {} Mapper: {}",
               self.prg_rom_size,
               self.chr_rom_size,
               self.mirroring,
               self.has_battery,
               self.has_trainer,
               self.ignore_mirroring,
               self.mapper)
        }
    }

    pub fn new(bytes: &Vec<u8>) -> Option<Header> {
        if bytes.len() < 16 {
            return None 
        }
        // Check if it starts with NES + msdos eol
        if bytes[0] != 0x4E ||
            bytes[1] != 0x45 ||
                bytes[2] != 0x53 ||
                bytes[3] != 0x1A {
                    return None
                }
        let prg_rom_size = (bytes[4] as usize) * 16 * 1024;
        let chr_rom_size = (bytes[5] as usize) * 8 * 1024;
        // Flags 6
        let b6 = bytes[6];
        let mirroring = if (b6 & 0x1) != 0 { Mirroring::Vertival } else { Mirroring::Horizontal };
        let has_battery = (b6 & 0x2) != 0;
        let has_trainer = (b6 & 0x4) != 0;
        let ignore_mirroring = (b6 & 0x8) != 0;
        let mapper_low = (b6 >> 4) & 0xf;

        // Flags 7
        let b7 = bytes[7];
        let vs_unisystem = (b7 & 0x1) != 0;
        let play_choice_10 = (b7 & 0x2) != 0;

        let nes2 = (b7 >> 3) & 0x3;
        if nes2 == 2 {
            println!("Spotted Nes 2.0 format...");
            return None 
        }
 
        let mapper_high = (b7 >> 4) & 0xf;
        let mapper = (mapper_low as usize) | ((mapper_high as usize) << 4);

        // TODO: support Flags 8-15
        let header = Header {
            prg_rom_size: prg_rom_size,
            chr_rom_size: chr_rom_size,
            mirroring: mirroring,
            has_battery: has_battery,
            has_trainer: has_trainer,
            ignore_mirroring: ignore_mirroring,
            mapper: mapper,
            vs_unisystem: vs_unisystem,
            play_choice_10: play_choice_10,
            prg_ram_size: 0,
            tv_system: TvSystem::NTSC,
            prg_ram: false,
            bus_conflicts: false
        };
        Some(header)
    }
}
