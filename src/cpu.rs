#[expect(dead_code)]
pub struct CPU {
    register_a: u8,
    status: u8,
    program_counter: u8,
}

#[expect(dead_code)]
impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        todo!()
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
}
