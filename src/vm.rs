use crate::instructions::Opcode;
use std::slice::Iter;

pub struct VM {
    registers: [i32; 32],
    remainder: u32,
    eq_flag: bool,
    stdout: usize,
    heap: Vec<u8>,
    pc: usize,
    program: Vec<u8>,
  
    
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            eq_flag: false,
            stdout: 0,
            heap: vec![],
        }
    }

    // quizás estaría bueno ocultar estas implementaciones detras de un trait Stdout
    pub fn stdout(&self) -> i32 {
        if self.stdout == 33 {
            self.eq_flag as i32
        } else {
            self.registers[self.stdout]
        }
    }

    pub fn sdtout_string(&self) -> String {
        if self.stdout == 33 {
            self.eq_flag.to_string()
        } else {
            self.registers[self.stdout].to_string()
        }
    }

    fn opcode(&mut self) -> Opcode {
        let op = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        op
    }

    fn next_8_bits(&mut self) -> u8 {
        let bits = self.program[self.pc];
        self.pc += 1;
        bits
    }

    fn next_16_bits(&mut self) -> u16 {
        let leftmost_8_bits = (self.program[self.pc] as u16) << 8;
        let rightmost_8_bits = self.program[self.pc + 1] as u16;
        let bits = leftmost_8_bits | rightmost_8_bits;
        self.pc += 2;
        bits
    }

    fn load(&mut self) {
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as i32;
        self.registers[register] = number;
        self.stdout = register; // this should be segregated to a single method such as self.write
    }

    fn read(&mut self) -> i32 {
        self.registers[self.next_8_bits() as usize]
    }

    fn read_two(&mut self) -> (i32, i32) {
        let register_x = self.read();
        let register_y = self.read();
        (register_x, register_y)
    }

    fn write(&mut self, v: i32) {
        let register = self.next_8_bits() as usize;
        self.registers[register] = v;
        self.stdout = register;
    }

    fn add(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.write(register_x + register_y);
    }

    fn sub(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.write(register_x - register_y);
    }

    fn mul(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.write(register_x * register_y);
    }

    fn div(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.write(register_x / register_y);
        self.remainder = (register_x % register_y) as u32;
    }

    fn jump(&mut self) {
        self.pc = self.read() as usize;
    }

    fn jump_forward(&mut self) {
        self.pc += self.read() as usize;
    }

    fn jump_back(&mut self) {
        self.pc -= self.read() as usize;
    }

    fn eq(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.eq_flag = register_x == register_y;
        self.next_8_bits();
    }

    fn neq(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.eq_flag = register_x != register_y;
        self.next_8_bits();
    }

    fn gt(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.eq_flag = register_x > register_y;
        self.next_8_bits();
    }
    fn lt(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.eq_flag = register_x < register_y;
        self.next_8_bits();
    }

    fn gteq(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.eq_flag = register_x >= register_y;
        self.next_8_bits();
    }

    fn lteq(&mut self) {
        let (register_x, register_y) = self.read_two();
        self.eq_flag = register_x <= register_y;
        self.next_8_bits();
    }

    fn jeq(&mut self) {
        if self.eq_flag {
            self.pc = self.read() as usize
        }
    }

    fn alloc(&mut self) {
        let register = self.next_8_bits() as usize;
        let bytes = self.registers[register];
        let new_end = self.heap.len() as i32 + bytes;
        self.heap.resize(new_end as usize,0);
    }

    fn continue_after(&mut self, f: impl Fn(&mut Self)) -> bool {
        f(self);
        true
    }

    fn stop(&self, message: &str) -> bool {
        println!("{message}");
        false
    }

    fn execute_instruction(&mut self) -> bool {
        match self.opcode() {
            Opcode::HLT => self.stop("halt!"),
            Opcode::LOAD => self.continue_after(Self::load),
            Opcode::ADD => self.continue_after(Self::add),
            Opcode::SUB => self.continue_after(Self::sub),
            Opcode::MUL => self.continue_after(Self::mul),
            Opcode::DIV => self.continue_after(Self::div),
            Opcode::JMP => self.continue_after(Self::jump),
            Opcode::JMPF => self.continue_after(Self::jump_forward),
            Opcode::JMPB => self.continue_after(Self::jump_back),
            Opcode::EQ => self.continue_after(Self::eq),
            Opcode::NEQ => self.continue_after(Self::neq),
            Opcode::GT => self.continue_after(Self::gt),
            Opcode::LT => self.continue_after(Self::lt),
            Opcode::GTEQ => self.continue_after(Self::gteq),
            Opcode::LTEQ => self.continue_after(Self::lteq),
            Opcode::JEQ => self.continue_after(Self::jeq),
            _ => self.stop(
                "unknown opcode\nthink about what you want to do and come back later\nsee ya!",
            ),
        }
    }

    fn is_not_done(&self) -> bool {
        self.pc < self.program.len()
    }

    fn should_run(&mut self) -> bool {
        self.is_not_done() && self.execute_instruction()
    }

    pub fn run(&mut self) {
        let mut run = true;
        while run {
            run = self.should_run()
        }
    }

    pub fn run_once(&mut self) {
        self.should_run();
    }

    pub fn program(&self) -> Iter<u8> {
        self.program.iter()
    }

    pub fn registers(&self) -> Iter<i32> {
        self.registers.iter()
    }
}

// this is just another name for the "extend" trait
pub trait Stdin<A> {
    fn stdin<T: IntoIterator<Item = A>>(&mut self, program: T);
}

impl Stdin<u8> for VM {
    fn stdin<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        self.program.extend(iter);
    }
}

impl Stdin<[u8; 4]> for VM {
    fn stdin<T: IntoIterator<Item = [u8; 4]>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|instruction| self.stdin(instruction));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn new_test_vm() -> VM {
        VM::new()
    }

    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0);
    }

    #[test]
    fn test_hlt() {
        let mut vm = new_test_vm();
        let bytes = vec![0, 0, 0, 0];
        vm.program = bytes;
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }
    #[test]
    fn test_ilgl() {
        let mut vm = new_test_vm();
        let bytes = vec![200, 0, 0, 0];
        vm.program = bytes;
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn test_load_to_0_ok() {
        let mut vm = new_test_vm();
        let bytes = vec![1, 0, 1, 244];
        vm.program = bytes;
        vm.run_once();
        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn test_load_to_0_fail() {
        let mut vm = new_test_vm();
        let bytes = vec![0, 0, 1, 244];
        vm.program = bytes;
        vm.run_once();
        assert_ne!(vm.registers[0], 500);
    }

    #[test]
    fn test_add() {
        let mut vm = new_test_vm();
        let bytes = vec![
            1, 0, 1, 244, // load 500 to register 0
            1, 1, 1, 244, // load 500 to register 1
            2, 0, 1, 2, // add load the sum of r0 and r1 to r2
        ];
        vm.program = bytes;
        vm.run();
        println!("registers = {:?}", vm.registers);
        assert_eq!(vm.registers[2], 1000);
    }

    #[test]
    fn test_sub() {
        let mut vm = new_test_vm();
        let bytes = vec![
            1, 0, 1, 244, // load 500 to register 0
            1, 1, 0, 244, // load 244 to register 1
            3, 0, 1, 2, // load the sum of r0 and r1 to r2
        ];
        vm.program = bytes;
        vm.run();
        println!("registers = {:?}", vm.registers);
        assert_eq!(vm.registers[2], 256);
    }

    #[test]
    fn test_mul() {
        let mut vm = VM::new();
        let bytes = vec![
            1, 0, 0, 2, // load 2 to register 0
            1, 1, 0, 255, // load 255 to register 1
            4, 0, 1, 2, // load the product of r0 and r1 to r2
        ];
        vm.program = bytes;
        vm.run();
        println!("registers = {:?}", vm.registers);
        assert_eq!(vm.registers[2], 255 * 2);
    }

    #[test]
    fn test_div() {
        let mut vm = new_test_vm();
        let bytes = vec![
            1, 0, 0, 244, // load 244 to register 0
            1, 1, 0, 2, // load 2 to register 1
            5, 0, 1, 2, // load the quotient of r0 and r1 to r2
        ];
        vm.program = bytes;
        vm.run();
        println!("registers = {:?}", vm.registers);
        assert_eq!(vm.registers[2], 122);
    }
    #[test]
    fn test_div_with_remainder() {
        let mut vm = new_test_vm();
        let bytes = vec![
            1, 0, 0, 245, // load 245 to register 0
            1, 1, 0, 2, // load 2 to register 1
            5, 0, 1, 2, // load the quotient of r0 and r1 to r2
        ];
        vm.program = bytes;
        vm.run();
        println!("registers = {:?}", vm.registers);
        assert_eq!(vm.registers[2], 122);
        assert_eq!(vm.remainder, 1);
    }

    #[test]
    fn test_jump() {
        let mut vm = new_test_vm();
        vm.registers[0] = 3;
        vm.program = vec![6, 0, 0, 0];
        vm.run_once();
        println!("registers = {:?}", vm.registers);
        println!("program_counter = {:?}", vm.pc);
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn test_jump_forward() {
        let mut vm = new_test_vm();
        vm.registers[0] = 4;
        vm.program = vec![7, 0, 0, 0];
        vm.run_once();
        println!("registers = {:?}", vm.registers);
        println!("program_counter = {:?}", vm.pc);
        assert_eq!(vm.pc, 6);
    }

    #[test]
    fn test_jump_back() {
        let mut vm = new_test_vm();
        vm.registers[1] = 2;

        vm.program = vec![8, 1, 0, 0];
        vm.run_once();
        vm.run_once();
        println!("registers = {:?}", vm.registers);
        println!("program_counter = {:?}", vm.pc);
        assert_eq!(vm.pc, 0);
    }

    #[test]
    fn test_jump_forward_then_back() {
        let mut vm = new_test_vm();
        vm.registers[0] = 2;
        vm.registers[1] = 4;

        vm.program = vec![
            7, 0, 0, 0, // jump forward as many steps as the value of r0
            8, 1, 0, 0, // jump backward as many steps as the value of r1
        ];
        vm.run_once();
        vm.run_once();
        println!("registers = {:?}", vm.registers);
        println!("program_counter = {:?}", vm.pc);
        assert_eq!(vm.pc, 2);
    }

    #[test]
    fn test_eq() {
        let mut vm = new_test_vm();
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.registers[2] = 5;
        vm.program = vec![
            9, 0, 1, 0, // compare r0 and r1 for equality
            9, 0, 2, 0, // compare r0 and r2 for equality
        ];
        println!("registers = {:?}", vm.registers);
        vm.run_once();
        println!("equality flag = {}", vm.eq_flag);
        assert_eq!(vm.eq_flag, true);
        vm.run_once();
        println!("equality flag = {}", vm.eq_flag);
        assert_eq!(vm.eq_flag, false);
    }

    #[test]
    fn test_neq() {
        let mut vm = new_test_vm();
        vm.registers[0] = 10;
        vm.registers[1] = 10;
        vm.registers[2] = 5;
        vm.program = vec![
            10, 0, 1, 0, // compare r0 and r1 for inequality
            10, 0, 2, 0, // compare r0 and r2 for inequality
        ];
        println!("registers = {:?}", vm.registers);
        vm.run_once();
        println!("equality flag = {}", vm.eq_flag);
        assert_eq!(vm.eq_flag, false);
        vm.run_once();
        println!("equality flag = {}", vm.eq_flag);
        assert_eq!(vm.eq_flag, true);
    }

    #[test]
    fn test_jeq() {
        let mut vm = new_test_vm();
        vm.registers[0] = 7;
        vm.eq_flag = true;
        vm.program = vec![15, 0, 0, 0];
        println!("registers = {:?}", vm.registers);
        println!("program_counter before = {}", vm.pc);
        vm.run_once();
        println!("equality flag = {}", vm.eq_flag);
        println!("program_counter after = {}", vm.pc);
        assert_eq!(vm.pc, 7);
    }
}
