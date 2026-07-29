#[derive(Debug, PartialEq)]
pub struct CPU {
    register_a: u8,
    status: u8,
    program_counter: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        let _program = program;
        todo!()
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn test_contructor() {
        let cpu = Some(CPU::new());
        match cpu {
            Some(_) => (),
            None => panic!("Expected a cpu"),
        }
    }

    #[test]
    fn test_default() {
        let cpu = CPU::default();
        assert!(cpu.program_counter == 0);
    }

    #[test]
    fn test_are_equal() {
        let cpu1 = CPU::default();
        let cpu2 = CPU::new();
        assert_eq!(cpu1, cpu2);
    }
}
