pub(crate) enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub(crate) enum R16 {
    BC,
    DE,
    HL,
}

impl R16 {
    pub(crate) fn get_upper(&self) -> R8 {
        match self {
            R16::BC => R8::B,
            R16::DE => R8::D,
            R16::HL => R8::H,
        }
    }
    pub(crate) fn get_lower(&self) -> R8 {
        match self {
            R16::BC => R8::C,
            R16::DE => R8::E,
            R16::HL => R8::L,
        }
    }
}

pub(crate) enum Condition {
    Z,
    NZ,
    NC,
    C,
}

pub(crate) enum Vec {
    X00,
    X08,
    X10,
    X18,
    X20,
    X28,
    X30,
    X38,
}

impl Vec {
    pub(crate) fn get(&self) -> u16 {
        match self {
            Vec::X00 => 0x00,
            Vec::X08 => 0x08,
            Vec::X10 => 0x10,
            Vec::X18 => 0x18,
            Vec::X20 => 0x20,
            Vec::X28 => 0x28,
            Vec::X30 => 0x30,
            Vec::X38 => 0x38,
        }
    }
}

pub(crate) enum U3 {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl U3 {
    pub(crate) fn get(&self) -> u8 {
        match self {
            U3::Zero => 0,
            U3::One => 1,
            U3::Two => 2,
            U3::Three => 3,
            U3::Four => 4,
            U3::Five => 5,
            U3::Six => 6,
            U3::Seven => 7,
        }
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub(crate) enum Instruction {
    ADC_A_R8 { r8: R8 },
    ADC_A_HL_PNTR,
    ADC_A_N8 { n8: u8 },
    ADD_A_R8 { r8: R8 },
    ADD_A_HL_PNTR,
    ADD_A_N8 { n8: u8 },
    ADD_HL_R16 { r16: R16 },
    ADD_HL_SP,
    ADD_SP_E8 { e8: i8 },
    AND_A_R8 { r8: R8 },
    AND_A_HL_PNTR,
    AND_A_N8 { n8: u8 },
    BIT_U3_R8 { u3: U3, r8: R8 },
    BIT_U3_HL_PNTR { u3: U3 },
    CALL_N16 { n16: u16 },
    CALL_CC_N16 { condition: Condition, n16: u16 },
    CCF,
    CP_A_R8 { r8: R8 },
    CP_A_N8 { n8: u8 },
    CP_A_HL_PNTR,
    CPL,
    DAA,
    DEC_R8 { r8: R8 },
    DEC_HL_PNTR,
    DEC_R16 { r16: R16 },
    DEC_SP,
    DI,
    EI,
    HALT,
    INC_R8 { r8: R8 },
    INC_HL_PNTR,
    INC_R16 { r16: R16 },
    INC_SP,
    JP_N16 { n16: u16 },
    JP_CC_N16 { condition: Condition, n16: u16 },
    JP_HL,
    JR_N16 { offset: i8 },
    JR_CC_N16 { condition: Condition, offset: i8 },
    LD_R8_R8 { dest: R8, src: R8 },
    LD_R8_N8 { dest: R8, n8: u8 },
    LD_R16_R16 { dest: R16, src: R16 },
    LD_HL_PNTR_R8 { r8: R8 },
    LD_HL_PNTR_N8 { n8: u8 },
    LD_R8_HL_PNTR { r8: R8 },
    LD_R16_PNTR_A { r16: R16 },
    LD_N16_PNTR_A { n16: u16 },
    LDH_N16_PNTR_A { n16: u16 },
    LDH_C_PNTR_A,
    LD_A_R16_PNTR { r16: R16 },
    LD_A_N16_PNTR { n16: u16 },
    LDH_A_N16_PNTR { n16: u16 },
    LDH_A_C_PNTR,
    LD_HLI_PNTR_A,
    LD_HLD_PNTR_A,
    LD_A_HLI_PNTR,
    LD_A_HLD_PNTR,
    LD_SP_N16 { n16: u16 },
    LD_N16_PNTR_SP { n16: u16 },
    LD_HL_SPE8 { e8: i8 },
    LD_SP_HL,
    NOP,
    OR_A_R8 { r8: R8 },
    OR_A_HL_PNTR,
    OR_A_N8 { n8: u8 },
    POP_AF,
    POP_R16 { r16: R16 },
    PUSH_AF,
    PUSH_R16 { r16: R16 },
    RES_U3_R8 { u3: U3, r8: R8 },
    RES_U3_HL_PNTR { u3: U3 },
    RET,
    RET_CC { condition: Condition },
    RETI,
    RL_R8 { r8: R8 },
    RL_HL_PNTR,
    RLA,
    RLC_R8 { r8: R8 },
    RLC_HL_PNTR,
    RLCA,
    RR_R8 { r8: R8 },
    RR_HL_PNTR,
    RRA,
    RRC_R8 { r8: R8 },
    RRC_HL_PNTR,
    RRCA,
    RST { vec: Vec },
    SBC_A_R8 { r8: R8 },
    SBC_A_HL_PNTR,
    SBC_A_N8 { n8: u8 },
    SCF {},
    SET_U3_R8 { u3: U3, r8: R8 },
    SET_U3_HL_PNTR { u3: U3 },
    SLA_R8 { r8: R8 },
    SLA_HL_PNTR,
    SRA_R8 { r8: R8 },
    SRA_HL_PNTR,
    SRL_R8 { r8: R8 },
    SRL_HL_PNTR,
    STOP,
    SUB_A_R8 { r8: R8 },
    SUB_A_HL_PNTR,
    SUB_A_N8 { n8: u8 },
    SWAP_R8 { r8: R8 },
    SWAP_HL_PNTR,
    XOR_A_R8 { r8: R8 },
    XOR_A_HL_PNTR,
    XOR_A_N8 { n8: u8 },
}
