pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    // takes the result of an operation and checks if
    // zero and negative flags should be updated
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        // if register a is zero, set zero flag
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status |= 0b1111_1101;
        }

        // if bit 7 (sign bit) is set in register a...
        if result & 0b1000_0000 != 0 {
            // enable the negative flag
            self.status |= 0b1000_0000;
        } else {
            // otherwise, disable it
            self.status &= 0b0111_1111;
        }
    }
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x += 1;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        // reset program counter before looping
        self.program_counter = 0;

        loop {
            // next instruction
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            // execute instruction
            match opscode {
                0x00 => {
                    return;
                }
                // LDA
                // - immediate
                0xA9 => {
                    // loads the parameter for immediate mode
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param)
                }
                // TAX
                // - implied
                0xAA => self.tax(),
                0xE8 => self.inx(),
                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }
}
