mod components;
mod memory;
mod registers;

pub use components::components::*;
pub use components::InstType;
pub use memory::*;
use memory::{Dram, Iram};
use registers::{Flags, Registers};

fn main() {
    let mut iram = Iram::new();
    let mut dram = Dram::new();

    // ======================== Initial values ==============================
    let init_x: [u32; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    let mut reg = Registers::new(init_x, 0);
    let w_ir_init: u32 = iram.m_am_imem(&reg.get_rpc());
    let wire_init = (
        // wires used across the stage
        0, 0, false, false, false, 0, 0, 0,
    );
    let (
        // wires used across the stage
        mut w_rt,
        mut w_ir,
        mut w_clk,
        mut before_clk,
        mut w_tkn,
        mut w_tpc,
        mut w_alu,
        mut w_npc,
    ) = wire_init;

    w_ir = w_ir_init;

    let mut flags = Flags {
        clk_rising_edge: false,
    };

    let test_imm = 4;
    let mut insttype: Option<InstType> = None;
    // =====================================================================

    for i in 1..30 {
        // ===============  Wires ONLY used in single stage =============================
        // Wires which don't be used across the stage.
        let mut r_pc = 0;
        //Initial instrucrtion is loaded when instruction memory is connected to PC circuit-wise.
        let mut ra1: usize = 0;
        let mut ra2: usize = 0;
        let mut wa: usize = 0;

        let mut w_r1 = 0;
        let mut w_r2 = 0;
        let mut w_s2 = 0;

        let mut w_imm :u32 = 0;
        let mut is_ld = false;

        //==============================================================================

        w_clk = !w_clk;

        if before_clk != w_clk && w_clk == true {
            // Detect rising edge
            // Rising edge is occured when both of below 2 condition are satisfied.
            //  1 Before clk is different from current clk.
            //  2 Current clk is high.
            flags.clk_rising_edge = true;
        } else {
            flags.clk_rising_edge = false;
        }

        if w_clk {
            // Instruction Fetch
            let mut  w_pcin = m_mux(
                &w_npc,
                &w_tpc,
                (insttype == Some(InstType::B)) && w_tkn == true,
            );
            reg.set_rpc(w_pcin);
            r_pc = reg.get_rpc();

            w_npc = m_adder(&test_imm, &r_pc);
            w_ir = iram.m_am_imem(&r_pc);

            // Instruction Decode
            ra1 = (w_ir as usize >> 15) & 0b11111;
            ra2 = (w_ir as usize >> 20) & 0b11111;
            wa = (w_ir as usize >> 7) & 0b11111;
            (w_imm, insttype, is_ld) = m_get_imm(w_ir);

            (w_r1, w_r2) = reg.m_register_file(ra1, ra2, wa, !(insttype == Some(InstType::S)) && !(insttype == Some(InstType::B)), &w_rt);
            w_tpc = m_adder(&w_imm, &r_pc);
            w_s2 = m_mux(&w_r2, &w_imm, !(insttype == Some(InstType::R)) && !(insttype == Some(InstType::B)));
            
            // Execute
            (w_alu, w_tkn) = m_alu(&w_r1, &w_s2);

            //Memory Access
            let w_ldd = dram.m_am_dmem(&w_alu, insttype == Some(InstType::S), &w_r2);

            //Write Back
            w_rt = m_mux(&w_alu, &w_ldd, is_ld);
        }

        if flags.clk_rising_edge {
            // Register's value update
            reg.set_x(w_rt, wa);
            println!(
                "<display> {}: {:8x}, {:8x}, {}, {}, {}\n",
                i, r_pc, w_imm, w_r1, w_s2, w_rt
            );

            if reg.get_x(30) != 0 {
                panic!(" value in x30 is not 0.");
            }
        }
        before_clk = w_clk;
    }
}
