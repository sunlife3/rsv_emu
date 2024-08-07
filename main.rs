mod components;
mod registers;
mod procs;

pub use components::components::*;
use registers::{Registers, Flags};

struct Wires {
    w_rt: u32,
    w_ir: u32,
    w_clk: bool,
    before_clk: bool,
} 


fn main() {

    // Initial value of resisters
    let init_x: [u32; 32] = 
        [0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
         0, 0];

    let mut reg = Registers::new(init_x,0);
    let w_ir_init: u32 = m_am_imem(&reg.get_rpc()); 
    let mut wire = Wires{
        w_rt: 0,
        w_ir: 0, 
        w_clk: false,
        before_clk: false,
        
    };
    wire.w_ir = w_ir_init;


    let mut flags = Flags {
        clk_rising_edge: false,
    };

    

    let test_imm = 4;

    for i in 1..158 {
        // ===============  Wires ONLY used in single stage =============================
        // Wires don't memorise, so wires have to be declared as local variable in loop.
        // (Clock signal is needed all stages, so is declared in out of loop.)
        let rpc = reg.get_rpc();
        let mut w_npc: u32 = 0;
        
            //Initial instrucrtion is loaded when instruction memory is connected to PC circuit-wise.

        let mut ra1: usize = 0;
        let mut  ra2: usize = 0;
        let mut  wa: usize = 0;

        let mut w_r1 = 0;
        let mut w_r2 = 0;
        let mut w_s2 = 0;
        //==============================================================================

        if 150 < i {
            
            wire.w_clk = !wire.w_clk;
        
            if wire.before_clk != wire.w_clk && wire.w_clk == true{
                // Detect rising edge
                // Rising edge is occured when both of below 2 condition are satisfied.
                //  1 Before clk is different from current clk.
                //  2 Current clk is high.
                flags.clk_rising_edge = true;
            }else {
                flags.clk_rising_edge = false;
            }

            if wire.w_clk {                
                // Instruction Fetch
                wire.w_ir = m_am_imem(&rpc);
                w_npc = m_adder(&test_imm, &rpc);

                // Instruction Decode
                //(w_r1, w_r2) = proc1(&reg, &w_ir);
                ra1 = (wire.w_ir as usize >> 15) & 0b11111;
                ra2 = (wire.w_ir as usize >> 20) & 0b11111;
                wa  = (wire.w_ir as usize >> 7) & 0b11111;

                let (w_imm, insttype) = m_get_imm(wire.w_ir);
                println!("imm:{}, inst:{:?}",w_imm,insttype);
                (w_r1, w_r2) = reg.m_register_file(ra1, ra2, wa, true, &wire.w_rt);
                w_s2 = m_mux(&w_r2, &w_imm, &(insttype == InstType::I));

                // Execute
                wire.w_rt = m_adder(&w_r1, &w_s2);
                               
            }

            if flags.clk_rising_edge {
                // Register's value update 
                reg.set_rpc(w_npc);
                reg.set_x(wire.w_rt, wa);
                println!("{}, {}, {}, {}", i, w_r1, w_s2, wire.w_rt);

                if reg.get_x(30) != 0 {
                    panic!(" value in x30 is not 0.");
                }
            }           
            wire.before_clk = wire.w_clk; 
        }
    }    
}
