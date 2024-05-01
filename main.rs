mod components;

pub use components::components::{m_adder, m_am_imem};

struct Registers {
    r_pc: u32,
}

struct Wires {
    w_npc: u32,
    w_ir: u32,
    w_clk: bool,
    clk_rising_edge: bool,
}


fn main() {
    let mut wires = Wires {
        w_npc: 0,
        w_ir: 0,
        w_clk: false,
        clk_rising_edge: false,
    };
    
    let mut registers = Registers {
        r_pc: 0,
    };

    let test_imm = 4;
    let mut before_clk: bool = false;

    for i in 0..401 {
        if 150 < i {
            if i%50 == 0 { 
                // Reverse clock 
                wires.w_clk = !wires.w_clk;
            }
            if before_clk != wires.w_clk && wires.w_clk == true{
                // Detect rising edge
                // Rising edge is occured when both of below 2 condition satisfied.
                //  1 Before clk is different from current clk.
                //  2 Current clk is high.
                wires.clk_rising_edge = true;
            }else {
                wires.clk_rising_edge = false;
            }
            if wires.clk_rising_edge {                 
                wires.w_npc = m_adder(&test_imm, &registers.r_pc);
                registers.r_pc = wires.w_npc;
                wires.w_ir = m_am_imem(&registers.r_pc);
                
            }
            if i%100 ==99{println!("{}, Ox{:08x}, Ox{:08x}",i, registers.r_pc, wires.w_ir);}
            before_clk = wires.w_clk; 
        }
    }    
}
