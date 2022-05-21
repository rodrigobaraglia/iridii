use crate::vm::{VM, Stdin};
use core::fmt::Debug;
use std;
use std::io;
use std::io::Write;
use std::iter::{once, Peekable};

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    fn read<'a>(&mut self, buffer: &'a mut String, stdin: &io::Stdin) -> io::Result<&'a str> {
        print!(">>>");
        io::stdout().flush()?;
        stdin.read_line(buffer)?;
        let command = buffer.trim();
        Ok(command)
    }

    fn listings<T: Debug, U: Iterator<Item = T>>(&self, items: U) {
        items.for_each(|item| println!("{:?}", item))
    }

    fn show_history(&self) {
        println!("Listing command history:");
        self.listings(self.command_buffer.iter());
        println!("End of Command Listing");
    }

    fn show_program(&self) {
        println!("Listing program instructions:");
        self.listings(self.vm.program());
        println!("End of Instruction Listing");
    }

    fn show_registers(&self) {
        println!("Listing registers and all contents:");
        self.listings(self.vm.registers());
        println!("End of Register Listing");
    }

    fn parse_hex<'a>(&mut self, i: &'a str) -> Peekable<impl Iterator<Item = u8> + 'a> {
        i.split_whitespace()
            .filter_map(|entry| u8::from_str_radix(&entry, 16).ok())
            .peekable()
    }

    fn eval(&mut self, input: &str) {
        let mut bytes = self.parse_hex(input);
        if bytes.peek().is_none() {
            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.");
        } else {
            self.vm.stdin(bytes);
            self.vm.run_once();
            println!("{}", self.vm.stdout())
        }
    }

    fn should_stop_on_command(&mut self, input: &str) -> bool {
        let mut should_stop = false;
        match input {
            ":quit" | ":q" => should_stop = true,
            ":history" | ":h" => self.show_history(),
            ":program" | ":p" => self.show_program(),
            ":registers" | ":r" => self.show_registers(),
            _ => self.eval(input),
        }
        should_stop
    }

    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut buffer = String::new();
        loop {
            let command = self.read(&mut buffer, &stdin)?;
            self.command_buffer.extend(once(command.to_string()));
            if self.should_stop_on_command(command) {
                break;
            }

            buffer.clear();
        }

        Ok(())
    }
}
