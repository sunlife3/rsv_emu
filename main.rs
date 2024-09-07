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
/* 
    iram.iram[0] = 0b00000000001000001000001010110011; //add x5, x1, x2
    iram.iram[1] = 0b00000000010000011000001100110011; //add x6, x3, x4
    iram.iram[2] = 0b00000000011000101000001110110011; //addi x1, x1, 1
*/
/*
    iram.iram[0] = 0b000000000101_00000_000_00001_0001101; //addi x1, x0, 5
    iram.iram[1] = 0b0000000_00001_00001_000_00010_0110011; //add x2, x1,x1
    iram.iram[2] = 0b000000000001_00001_000_00001_0001101; //addi x1, x1, 1
    iram.iram[3] = 0b1111111_00010_00001_001_11101_1100011; //bne x1,x2, L
    iram.iram[4] = 0b000000001001_00001_000_01010_0001101; //addi x10,x1,9
*/
/*
    iram.iram[0] = 0b000000000011_00000_000_00001_0001101; //addi x1, x0, 3
    iram.iram[1] = 0b000000000100_00001_000_00010_0001101; //addi x2, x1, 4
    iram.iram[2] = 0b000000000101_00010_000_01010_0001101; //addi x1, x1, 1
*/

    iram.iram[0] = 0b000000000000_00000_000_01010_0001101; // 00 addi x10, x0, 0
    iram.iram[1] = 0b000000000000_00000_000_00011_0001101; // 04 addi x3, x0, 0
    iram.iram[2] = 0b000001100101_00000_000_00001_0001101; // 08 addi x1, x0, 101
    iram.iram[3] = 0b0000000_00011_01010_000_01010_0110011; //0c add x10, x10, x3
    iram.iram[4] = 0b000000000001_00011_000_00011_0001101; // 10 addi x3, x3, 1
    iram.iram[5] = 0b1111111_00001_00011_001_11001_1100011; //14 bne x3, x1, L
    iram.iram[6] = 0b000000000000_00000_000_00000_0001101; // 18 addi x0, x0, 0(NOP)
    iram.iram[7] = 0x50f13; //HALT

/*
    iram.iram[0] = 0b000000000000_00000_000_01010_0001101; //  00 addi x10, x0, 0
    iram.iram[1] = 0b000000000000_00000_000_00011_0001101; //  04 addi x3, x0, 0
    iram.iram[2] = 0b000100000000_00000_000_00001_0001101; //  08 addi x1, x0, 256
    iram.iram[3] = 0b0000000_00011_00011_010_00000_0100011; //  0c L:sw x3,0(x3)
    iram.iram[4] = 0b000000000100_00011_000_00011_0001101; //  10 addi x3, x3, 4
    iram.iram[5] = 0b1111111_00001_00011_001_11001_1100011; // 14 bne x3, x1, L
    iram.iram[6] = 0b000000000000_00000_000_00011_0001101; //  18 addi x3, x0, 0
    iram.iram[7] = 0b000000000000_00011_010_00100_0000011; //  1c M:lw x4, 0(x3)
    iram.iram[8] = 0b000000000100_00011_000_00011_0001101; //  20 addi x3, x3, 4
    iram.iram[9] = 0b0000000_00100_01010_000_01010_0110011; // 24 add x10, x10, x4
    iram.iram[10] = 0b1111111_00001_00011_001_10101_1100011;// 28 bne x3, x1, L
    iram.iram[11] = 0x50f13; //HALT
*/
    let mut reg = Registers::new(init_x, 0);
    let w_ir_init: u32 =0;
    let wire_init = (
        // wires used across the stage
        0, 0, false, false, false, 0, 0, 0, 0, false, 0, 0, 0,
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
        mut r_pc,
        mut w_miss,
        mut w_r1 ,
        mut w_r2 ,
        mut w_s2 ,
    ) = wire_init;

    w_ir = w_ir_init;

    //Pipeline registers
    let mut p1_ir:u32 = 0x13;
    let mut p1_pc:u32 = 0;
    let mut p1_v:bool = false;

    let mut flags = Flags {
        clk_rising_edge: false,
    };

    let test_imm = 4;
    let mut insttype: Option<InstType> = None;
    // =====================================================================

    for i in 1..1000 {
        // ===============  Wires ONLY used in single stage =============================
        // Wires which don't be used across the stage.
        //Initial instrucrtion is loaded when instruction memory is connected to PC circuit-wise.

        let mut w_imm :u32 = 0;
        let mut is_ld = false;

        let mut w_pcin:u32 = 0; 
        
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
            w_miss = (insttype == Some(InstType::B)) && (w_tkn == true) && (p1_v == true);
            //println!("w_miss:{},  w_tkn:{},  p1_v:{}", w_miss,w_tkn,p1_v);
            w_pcin = m_mux(
                &w_npc,
                &w_tpc,
                w_miss,
            );

            reg.set_rpc(w_pcin);
            r_pc = reg.get_rpc();

            w_npc = m_adder(&test_imm, &r_pc);
            w_ir = iram.m_am_imem(&r_pc);

            
            // Instruction Decode
            (w_imm, insttype, is_ld) = m_get_imm(p1_ir);
            

            (w_r1, w_r2) = reg.m_register_file(
                (p1_ir as usize >> 15) & 0b11111,
                (p1_ir as usize >> 20) & 0b11111, 
                (p1_ir as usize >> 7) & 0b11111,
                !(insttype == Some(InstType::S)) && !(insttype == Some(InstType::B)) && (p1_v == true),
                &w_rt);
            w_tpc = m_adder(&w_imm, &p1_pc);
            w_s2 = m_mux(&w_r2, &w_imm, !(insttype == Some(InstType::R)) && !(insttype == Some(InstType::B)));

            // Execute
            (w_alu, w_tkn) = m_alu(&w_r1, &w_s2);          

            //Memory Access
            let w_ldd = dram.m_am_dmem(&w_alu, (insttype == Some(InstType::S)) && (p1_v == true), &w_r2);

            //Write Back
            // Register's value update
            // All modules works with posedge in circuit, so output w_rt from EX/WB and setting it to reg[wa] in RF at same timing. 
            // reg.set_x in this section is set to adapt this behavior.
            // It's correction logical behavior of the program and circuit behavior.  
            w_rt = m_mux(&w_alu, &w_ldd, is_ld);
            reg.set_x(w_rt, (p1_ir as usize >> 7) & 0b11111);

            if (i+1)/2 == 404{
                for i in 0..31 {
                    println!("register{}: {}",i, reg.get_x(i));
                } 
            }

            println!(
                "<display> CC{}: {:5x}, {:5x}, {:5}, {:5}, {:5}, w_miss:{}, p1_v:{}, tkn:{}",
                (i+1)/2, r_pc, p1_pc, w_r1, w_s2, w_rt, w_miss, p1_v, w_tkn
            ); 
            // Pipeline register
            r_pc = w_pcin.clone();
            p1_ir = w_ir.clone();
            p1_pc = r_pc.clone();
            p1_v = !w_miss;

        }

        if flags.clk_rising_edge {
            if (p1_ir as usize >> 7) & 0b11111 == 30 {
                println!("[HALT] x30 is written.");
                break;
            }
        }
        before_clk = w_clk;
    }
}
