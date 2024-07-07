
pub mod components {
    #[derive(Debug)]
    pub enum InstType {
        J,
        B,
        S,
        R,
        U,
        I,
    }
    
    pub fn m_adder(w_in1: &u32, w_in2: &u32) -> u32{
        return  *w_in1 + *w_in2;
    }
    
    pub fn m_cmp(w_in1: u32, w_in2: u32) -> bool{
        return w_in1 == w_in2;
    }
    
    pub fn m_mux(w_in1: &u32, w_in2: &u32, w_s: &bool) -> u32 {
        if *w_s {
            return *w_in2;
        }else{
            return *w_in1;
        }
    }

    fn  m_get_type(opecode: u32) -> InstType {
        match opecode {
            0b11011 => InstType::J,
            0b11000 => InstType::B,
            0b01000 => InstType::S,
            0b01100 => InstType::R,
            0b01101 | 0b00101 => InstType::U,
            _ => InstType::I
        }
    
    }

    pub fn m_get_imm(ir: u32) -> (u32, InstType){
        let opecode = ir>>2 & 0b11111;
        let insttype = m_get_type(opecode);
    
        let repeat: u32 =
            match  insttype {
                InstType::J  if ((ir>>31)&1) == 1 => {
                    0xFFF //12{ir[31]}
                },
                InstType::B  if ((ir>>31)&1) == 1 =>{
                    0xFFFFF //20{ir[31]}
                },
                InstType::S  if ((ir>>31)&1) == 1 =>{
                    0xFFFFF //20{ir[31]}
                },
                InstType::I  if ((ir>>31)&1) == 1 =>{
                    0xFFFFF //20{ir[31]}
                },
                _ => 0
            };
        
        let immediate :u32=
            match insttype {
                InstType::J => repeat << 20 | ((ir>>12)&0xff) << 12 | ((ir>>20)&1) << 11 | ((ir>>21)&0x3ff),
                InstType::B => repeat << 12 | ((ir>>7)&1) << 11 | ((ir>>25)&0x3f) << 5 | ((ir>>8)&0xf) << 1 | 1,
                InstType::S => repeat << 12 | ((ir>>25)&0x7f) << 5 | ((ir>>7) & 0x1f),
                InstType::I => repeat << 12 | ((ir>>20)&0xfff),
                InstType::U =>((ir>>12)&0xfffff) << 12 | 0,
                _ => 0
            };
    
        (immediate, insttype)
    }

    pub fn m_am_imem(w_adr: &u32) -> u32 {
        /*
        if *w_adr == 0 {
            return 0b00000000000100000000000010110011; //add x1, x0, x1
        } else if *w_adr == 4 {
            return 0b00000000000000001000000010110011; //add x1, x1, x0
        } else {
            return 0b00000000000100001000000010110011; //add x1, x1, x1
        }
         */
        if *w_adr == 0 {
            return 0b00000000001000001000001010110011; //add x5, x1, x2
        } else if *w_adr == 4 {
            return 0b00000000010000011000001100110011; //add x6, x3, x4
        } else {
            return 0b00000000011000101000001110110011; //add x7, x5, x6
        }
    }  
   

}