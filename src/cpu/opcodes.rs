use std::sync::LazyLock;

#[expect(dead_code)]
#[expect(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NonAddressing,
    Implied,
}

#[expect(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum Mnemonic {
    ADC,
    AND,
    LDA,
    TAX,
    BRK,
    INX,
}

#[derive(Debug, Clone, Copy)]
pub struct OpCode {
    pub mnemonic: Mnemonic,
    pub mode: AddressMode,
    pub _bytes: u8,
    pub _cycles: u8,
}

impl OpCode {
    fn new(mnemonic: Mnemonic, mode: AddressMode, _bytes: u8, _cycles: u8) -> Self {
        Self {
            mnemonic,
            mode,
            _bytes,
            _cycles,
        }
    }
}

pub static OP_TABLE: LazyLock<[Option<OpCode>; 256]> = LazyLock::new(|| {
    let mut table = [None; 256];

    // BRK
    table[0x00] = Some(OpCode::new(Mnemonic::BRK, AddressMode::Implied, 1, 7));

    // INX
    table[0xE8] = Some(OpCode::new(Mnemonic::INX, AddressMode::Implied, 1, 2));

    // TAX
    table[0xAA] = Some(OpCode::new(Mnemonic::TAX, AddressMode::Implied, 1, 2));

    // LDA
    table[0xA9] = Some(OpCode::new(Mnemonic::LDA, AddressMode::Immediate, 2, 2));
    table[0xA5] = Some(OpCode::new(Mnemonic::LDA, AddressMode::ZeroPage, 2, 3));
    table[0xB5] = Some(OpCode::new(Mnemonic::LDA, AddressMode::ZeroPage_X, 2, 4));
    table[0xAD] = Some(OpCode::new(Mnemonic::LDA, AddressMode::Absolute, 3, 4));
    table[0xBD] = Some(OpCode::new(
        Mnemonic::LDA,
        AddressMode::Absolute_X,
        3,
        4, /* +1 if page is crossed */
    ));
    table[0xB9] = Some(OpCode::new(
        Mnemonic::LDA,
        AddressMode::Absolute_Y,
        3,
        4, /* +1 if page is crossed */
    ));
    table[0xA1] = Some(OpCode::new(Mnemonic::LDA, AddressMode::Indirect_X, 2, 6));
    table[0xB1] = Some(OpCode::new(
        Mnemonic::LDA,
        AddressMode::Indirect_Y,
        2,
        6, /* +1 if page is crossed */
    ));

    // ADC
    table[0x69] = Some(OpCode::new(Mnemonic::ADC, AddressMode::Immediate, 2, 2));
    table[0x65] = Some(OpCode::new(Mnemonic::ADC, AddressMode::ZeroPage, 2, 3));
    table[0x75] = Some(OpCode::new(Mnemonic::ADC, AddressMode::ZeroPage_X, 2, 4));
    table[0x6D] = Some(OpCode::new(Mnemonic::ADC, AddressMode::Absolute, 3, 4));
    table[0x7D] = Some(OpCode::new(
        Mnemonic::ADC,
        AddressMode::Absolute_X,
        3,
        4, /* +1 if page is crossed */
    ));
    table[0x79] = Some(OpCode::new(
        Mnemonic::ADC,
        AddressMode::Absolute_Y,
        3,
        4, /* +1 if page is crossed */
    ));
    table[0x61] = Some(OpCode::new(Mnemonic::ADC, AddressMode::Indirect_X, 2, 6));
    table[0x71] = Some(OpCode::new(
        Mnemonic::ADC,
        AddressMode::Indirect_Y,
        2,
        5, /* +1 if page is crossed */
    ));

    // AND
    table[0x29] = Some(OpCode::new(Mnemonic::AND, AddressMode::Immediate, 2, 2));
    table[0x25] = Some(OpCode::new(Mnemonic::AND, AddressMode::ZeroPage, 2, 3));
    table[0x35] = Some(OpCode::new(Mnemonic::AND, AddressMode::ZeroPage_X, 2, 4));
    table[0x2D] = Some(OpCode::new(Mnemonic::AND, AddressMode::Absolute, 3, 4));
    table[0x3D] = Some(OpCode::new(
        Mnemonic::AND,
        AddressMode::Absolute_X,
        3,
        4, /* +1 if page is crossed */
    ));
    table[0x39] = Some(OpCode::new(
        Mnemonic::AND,
        AddressMode::Absolute_Y,
        3,
        4, /* +1 if page is crossed */
    ));
    table[0x21] = Some(OpCode::new(Mnemonic::AND, AddressMode::Indirect_X, 2, 6));
    table[0x31] = Some(OpCode::new(
        Mnemonic::AND,
        AddressMode::Indirect_Y,
        2,
        5, /* +1 if page is crossed */
    ));

    table
});
