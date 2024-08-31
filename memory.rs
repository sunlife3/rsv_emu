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
    pub iram: Vec<u32>,
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
        //let adr = w_pc>>2 & 0b111111;
        let adr = w_pc/4; // 1 word is 32bit  
        self.iram[adr as usize]

        //self.iram[adr as usize]
        //let w_insn = self.iram[adr as usize];

    }
    
}
