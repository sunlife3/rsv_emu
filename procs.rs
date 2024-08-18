pub mod proc {
    use components::components::{m_cmp, m_mux};
    use registers::Registers;
    
    /*pub fn proc1(reg :&Registers, w_ir: &u32) -> (u32, u32){
        let cmp1 = m_cmp(0b00001, (w_ir >> 15) & 0b11111);
        let cmp2 = m_cmp(0b00001, (w_ir >> 20) & 0b11111);
        let regx1 = reg.get_x(1);
        let w_r1 = m_mux(&0, &regx1, &cmp1);
        let w_r2 = m_mux(&0, &regx1, &cmp2);
        (w_r1, w_r2)
    }
    */
}
