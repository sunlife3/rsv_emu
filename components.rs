#[derive(Debug)]
#[derive(PartialEq)]
pub enum InstType {
    J,
    B,
    S,
    R,
    U,
    I,
}

pub mod components {
    use components::InstType;
    
    pub trait CustomAdd<T, U> {
        type Output;
        fn custom_add(imm: &T, arg: &U) -> Self::Output;
    }

    // add i32 (i32, i32 -> i32)
    impl CustomAdd<i32, i32> for i32 {
        type Output = i32;
        fn custom_add(imm: &i32, arg: &i32) -> i32 {
            *imm + *arg
        }
    }

    // add i32 u32 (i32, u32 -> u32)
    impl CustomAdd<u32, u32> for i32 {
        type Output = i32;
        fn custom_add(imm_u32: &u32, arg: &u32) -> i32 {
            *imm_u32 as i32 + *arg as i32
        }
    }

    impl CustomAdd<u32, u32> for u32 {
        type Output = u32;
        fn custom_add(imm_u32: &u32, arg: &u32) -> u32 {
             (*imm_u32 as i32 + *arg as i32) as u32
        }
    }  

    pub fn m_adder<T, U>(imm: &T, arg: &U) -> <T as CustomAdd<T, U>>::Output
    where
        T: CustomAdd<T, U>,
    {
        T::custom_add(imm, arg)
    }
  
    pub fn m_alu(w_in1: &u32, w_in2: &u32) -> (u32, bool){
        return  (m_adder(w_in1 ,w_in2), *w_in1 != *w_in2)
    }
    
    pub fn m_cmp(w_in1: u32, w_in2: u32) -> bool{
        return w_in1 == w_in2;
    }
 
    
    pub fn m_mux(w_in1: &u32, w_in2: &u32, w_s: bool) -> u32 {
        //println!("w_in1{}, w_in2:{}, w_s:{}",w_in1, w_in2, w_s);
        if w_s {
            return *w_in2;
        }else{
            return *w_in1;
        }
    }

    fn  m_get_type(opecode: u32) -> Option<InstType> {
        match opecode {
            0b11011 => Some(InstType::J),
            0b11000 => Some(InstType::B),
            0b01000 => Some(InstType::S),
            0b01100 => Some(InstType::R),
            0b01101 | 0b00101 => Some(InstType::U),
            _ => Some(InstType::I)
        }
    
    }

    pub fn m_get_imm(ir: u32) -> (u32, Option<InstType>, bool){
        let opecode = ir>>2 & 0b11111;
        let insttype = m_get_type(opecode);
    
        let repeat: u32 =
            match  insttype {
                Some(InstType::J)  if ((ir>>31)&1) == 1 => {
                    0xFFF //12{ir[31]}
                },
                Some(InstType::B)  if ((ir>>31)&1) == 1 =>{
                    0xFFFFF //20{ir[31]}
                },
                Some(InstType::S)  if ((ir>>31)&1) == 1 =>{
                    0xFFFFF //20{ir[31]}
                },
                Some(InstType::I)  if ((ir>>31)&1) == 1 =>{
                    0xFFFFF //20{ir[31]}
                },
                _ => 0
            };
        
        let immediate :u32=
            match insttype {
                Some(InstType::J) => repeat << 20 | ((ir>>12)&0xff) << 12 | ((ir>>20)&1) << 11 | ((ir>>21)&0x3ff),
                Some(InstType::B) => repeat << 12 | ((ir>>7)&1) << 11 | ((ir>>25)&0x3f) << 5 | ((ir>>8)&0xf) << 1 ,
                Some(InstType::S) => repeat << 12 | ((ir>>25)&0x7f) << 5 | ((ir>>7) & 0x1f),
                Some(InstType::I) => repeat << 12 | ((ir>>20)&0xfff),
                Some(InstType::U) =>((ir>>12)&0xfffff) << 12 | 0,
                _ => 0
            };
        
        let ld = ((ir>>2)&0b11111) == 0;
    
        (immediate, insttype, ld)
    }

}