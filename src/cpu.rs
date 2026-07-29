mod opcodes;
mod status;

#[cfg(test)]
mod tests;

use opcodes::{AddressMode, Mnemonic::*, OP_TABLE};
use status::*;

#[derive(Debug, PartialEq)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0x10000],
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | lo
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0x10000],
        }
    }

    /// Loads the program in the ROM memory space and sets the
    /// 0xFFFC memory address to the address of the start of the ROM space
    pub fn load(&mut self, program: &[u8]) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    /// Resets all registers and status bits to 0 and sets the program
    /// counter to the value in 0xFFFC
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        // Reset interrupt address
        self.program_counter = self.mem_read_u16(0xFFFC)
    }

    pub fn load_and_run(&mut self, program: &[u8]) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.fetch8();
            let opcode = OP_TABLE[opcode as usize]
                .unwrap_or_else(|| panic!("Invalid opcode {:02X}", opcode));

            match opcode.mnemonic {
                ADC => self.adc(opcode.mode),
                AND => self.and(opcode.mode),
                LDA => self.lda(opcode.mode),
                TAX => self.tax(),
                INX => self.inx(),
                BRK => return,
            }
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

/// Helpers
impl CPU {
    fn fetch8(&mut self) -> u8 {
        let u8 = self.mem_read(self.program_counter);
        self.program_counter += 1;
        u8
    }

    fn fetch16(&mut self) -> u16 {
        let u16 = self.mem_read_u16(self.program_counter);
        self.program_counter += 2;
        u16
    }

    fn get_operand_address(&mut self, mode: AddressMode) -> u16 {
        match mode {
            AddressMode::Immediate => {
                let addr = self.program_counter;
                self.program_counter += 1;
                addr
            }
            AddressMode::ZeroPage => self.fetch8() as u16,
            AddressMode::ZeroPage_X => self.fetch8().wrapping_add(self.register_x) as u16,
            AddressMode::ZeroPage_Y => self.fetch8().wrapping_add(self.register_y) as u16,
            AddressMode::Absolute => self.fetch16(),
            AddressMode::Absolute_X => self.fetch16().wrapping_add(self.register_x as u16),
            AddressMode::Absolute_Y => self.fetch16().wrapping_add(self.register_y as u16),
            AddressMode::Indirect_X => {
                let look_up_address = self.fetch8().wrapping_add(self.register_x);
                let lo = self.mem_read(look_up_address as u16) as u16;
                let hi = self.mem_read(look_up_address.wrapping_add(1) as u16) as u16;
                (hi << 8) | lo
            }
            AddressMode::Indirect_Y => {
                let look_up_address = self.fetch8();
                let lo = self.mem_read(look_up_address as u16) as u16;
                let hi = self.mem_read(look_up_address.wrapping_add(1) as u16) as u16;
                let deref_base = (hi << 8) | lo;
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressMode::Implied => unreachable!(),
            AddressMode::NonAddressing => panic!("mode {:?} is not supported", mode),
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // Clear zero and negative flags
        self.status &= !(ZERO | NEGATIVE);

        // Set zero flag if result is zero
        if result == 0 {
            self.status |= ZERO;
        }

        // Set negative flag if result is negative
        self.status |= result & NEGATIVE;
    }
}

/// Instructions
impl CPU {
    fn adc(&mut self, mode: AddressMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let carry = if self.status & CARRY != 0 { 1 } else { 0 };

        let sum = self.register_a as u16 + value as u16 + carry;
        let result = sum as u8;

        if sum > 0xFF {
            self.status |= CARRY
        } else {
            self.status &= !CARRY;
        }

        if (!(self.register_a ^ value) & (self.register_a ^ result) & 0b1000_0000) != 0 {
            self.status |= OVERFLOW;
        } else {
            self.status &= !OVERFLOW;
        }

        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn and(&mut self, mode: AddressMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn lda(&mut self, mode: AddressMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
}
