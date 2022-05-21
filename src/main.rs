use std::io;
pub mod assembler;
pub mod instructions;
pub mod repl;
pub mod vm;

fn main() -> io::Result<()> {
    let mut r = repl::REPL::new();
    r.run()
}
