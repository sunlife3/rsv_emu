pub struct Dram {
    dram: Vec<u32>,
    size: u32,
}

impl Dram {
    pub fn new() -> Dram {
        // recommend: size should be immediate.
        let mut dram = Vec::new();
        let size = 64;
        for _i in 0..size {
            dram.push(0);
        }
        Dram { dram, size }
    }

    pub fn m_am_dmem(&mut self, adr: &u32, w_we: bool, w_wd: &u32) -> u32 {
        let address = (adr>>2)&0b111111;
        let w_rd = self.dram[address as usize];
        if w_we {
            self.dram[address as usize] = *w_wd;
        }

        w_rd
    }
}

pub struct Iram {
    iram: Vec<u32>,
    size: u32,
}

impl Iram {
    pub fn new() -> Iram {
        // recommend: size should be immediate.
        let mut iram = Vec::new();
        let size = 64;
        for _i in 0..size {
            iram.push(0);
        }
        Iram { iram, size }
    }


    pub fn m_am_imem(&mut self, w_pc: &u32) -> u32 {
        let adr = w_pc>>2 & 0b111111;
        //let w_insn = self.iram[adr as usize];

        /*
        if *w_adr == 0 {
            return 0b00000000000100000000000010110011; //add x1, x0, x1
        } else if *w_adr == 4 {
            return 0b00000000000000001000000010110011; //add x1, x1, x0
        } else {
            return 0b00000000000100001000000010110011; //add x1, x1, x1
        }
         */
        /*
        if *w_adr == 0 {
            return 0b00000000001000001000001010110011; //add x5, x1, x2
        } else if *w_adr == 4 {
            return 0b00000000010000011000001100110011; //add x6, x3, x4
        } else {
            return 0b00000000011000101000001110110011; //add x7, x5, x6
                     0000000000110000000000001001101
        }
         */
        /*
        if *w_adr == 0 {
            // 5-5
            return 0b000000000011_00000_000_00001_0001101; //addi x1, x0, 3
        } else if *w_adr == 4 {
            return 0b000000000100_00001_000_00010_0001101; //addi x2, x1, 4
        } else {
            return 0b000000000101_00010_000_00011_0001101; //addi x3, x2, 5
        }
        */
    
        if *w_pc == 0 {
            // 5-6
            return 0b000000000111_00000_000_00001_0001101; //addi x1, x0, 7
        } else if *w_pc == 4 {
            return 0b0000000_00001_00000_010_01000_0100011; //sw x1, 8(x0)
        } else {
            return 0b000000001000_00000_010_00010_0000011; //lw x2, 8(x0)
        }
    }
    
}
