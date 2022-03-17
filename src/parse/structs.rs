use bitflags::bitflags;

pub enum Instruction {
    AInstruction(Ident),
    CInstruction {
        source: Src,
        dst: Dst,
        jump: JumpCondition,
    }
}

pub enum Ident {
    Name(String),
    Addr(u16),
}

pub enum Src {
    Register,
    Memory,
}

bitflags! {
    pub struct Dst: u8 {
        const A = 0b001;
        const D = 0b010;
        const MEM = 0b100;
    }
}

pub enum JumpCondition {
    Never,
    Always,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
    Equal,
    NEqual,
}
