pub struct Registers {
    gp_registers: Vec<GpRegister>,
    r_pc: u32,
}

struct GpRegister {
    reg_x: u32,
}

impl Registers {
    pub fn new(init_x1:u32, init_rpc:u32) -> Registers{
        let mut gp_registers = Vec::new();
        for _i in 0..31 {
            gp_registers.push(GpRegister {reg_x: 0})
        }
        let r_pc = init_rpc;
        gp_registers[1].reg_x = init_x1;

        Registers {
            gp_registers,
            r_pc,
        }

    }
    pub fn get_x(&self, index: usize) -> u32 {
        self.gp_registers[index].reg_x
    }
    pub fn set_x(&mut self, value: u32, index: usize) {
        self.gp_registers[index].reg_x = value;
    }
    pub fn get_rpc(&self) -> u32 {
        self.r_pc
    }
    pub fn set_rpc(&mut self, value: u32) {
        self.r_pc = value;
    }
    pub fn m_register_file(&mut self, w_ra1: usize, w_ra2: usize, w_wa: usize, w_we: bool, w_wd: &u32 ) -> (u32, u32) {
        // Actually, u8 arguments are all 5bit values, however, no types for them.
        
        let w_rd1:u32;
        let w_rd2:u32;

        if w_ra1 == 0 {
            w_rd1 = 0;
        }else {
            w_rd1 = self.get_x(w_ra1);
        }

        if w_ra2 == 0 {
            w_rd2 = 0;
        }else {
            w_rd2 = self.get_x(w_ra2);
        }
        if w_we {
            self.set_x(*w_wd, w_wa);
        }
        (w_rd1, w_rd2)
    }
}



pub struct Flags {
    // Actually, Variables in this struct are unnnecessary because electric curcuit detects these parameters automatically. 
    pub clk_rising_edge: bool,
}