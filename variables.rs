pub struct Registers {
    x1: u32,
    r_pc: u32,
}
impl Registers {
    pub fn new(init_x1:u32, init_rpc:u32) -> Registers{
        Registers {
            x1: init_x1,
            r_pc: init_rpc,
        }
    }
    pub fn get_x1(&self) -> u32 {
        self.x1
    }
    pub fn set_x1(&mut self, value: u32) {
        self.x1 = value;
    }
    pub fn get_rpc(&self) -> u32 {
        self.r_pc
    }
    pub fn set_rpc(&mut self, value: u32) {
        self.r_pc = value;
    }
}

pub struct Flags {
    // Actually, Variables in this struct are unnnecessary because electric curcuit detects these parameters automatically. 
    pub clk_rising_edge: bool,
}