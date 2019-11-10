use num_derive::FromPrimitive;
#[allow(unused_imports)]
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Instruction {
    NOP = 0x1,
    HLT = 0x2,
    JMP = 0x3
}
