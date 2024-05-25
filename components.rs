pub mod components {
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