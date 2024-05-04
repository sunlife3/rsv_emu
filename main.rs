mod components;
mod variables;
mod procs;

pub use components::components::*;
use variables::{Registers, Flags};
use procs::proc::proc1;

fn main() {
    
    let mut reg = Registers::new(3,0);

    let mut flags = Flags {
        clk_rising_edge: false,
    };

    let mut w_clk: bool = false;
    let mut before_clk: bool = false;

    let test_imm = 4;

    for i in 1..401 {
        // ===============  Wires ONLY used in single stage =============================
        // Wires don't memorise, so wires have to be declared as local variable in loop.
        // (Clock signal is needed all stages, so is declared in out of loop.)
        let rpc = reg.get_rpc();
        let mut w_npc: u32 = 0;
        let mut w_ir: u32 = m_am_imem(&rpc); 
            //Initial instrucrtion is loaded when instruction memory is connected to PC circuit-wise.
        let mut w_rt: u32 = 0;

        let mut w_r1 = 0;
        let mut w_r2 = 0;
        //==============================================================================

        if 150 < i {
            
            if i%50 == 0 { 
                // Reverse clock 
                w_clk = !w_clk;
            }
            if before_clk != w_clk && w_clk == true{
                // Detect rising edge
                // Rising edge is occured when both of below 2 condition are satisfied.
                //  1 Before clk is different from current clk.
                //  2 Current clk is high.
                flags.clk_rising_edge = true;
            }else {
                flags.clk_rising_edge = false;
            }

            if w_clk {                
                // Instruction Fetch
                w_ir = m_am_imem(&rpc);
                w_npc = m_adder(&test_imm, &rpc);

                // Instruction Decode
                (w_r1, w_r2) = proc1(&reg, &w_ir);

                // Execute
                w_rt = m_adder(&w_r1, &w_r2);
            }

            if flags.clk_rising_edge {
                // Register's value update 
                reg.set_rpc(w_npc);
                reg.set_x1(w_rt);
                
            }

            if i%99 == 0{
                // IF test (before rising edge)
                // println!("{}, Ox{:08x}, 0x{:08x}",i, reg.r_pc, w_ir);
            }
            if i%100 == 0{
                // ID test ()
                println!("{}, {}, {}, {}", i, w_r1, w_r2, w_rt);
            }
            
            before_clk = w_clk; 
        }
    }    
}
