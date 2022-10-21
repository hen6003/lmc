mod asm;
mod lmc;

use std::io::Read;

use asm::*;
use lmc::*;

fn main() {
    let file = std::env::args().nth(1).expect("Please provide a file");
    let mut file = std::fs::File::open(file).expect("Failed to open file");

    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)
        .expect("Failed to read file");

    let data = IR::parse_lines(&file_contents);

    let memory = data.assemble();
    let mut lmc = LMC::new(IO::new(), memory);

    lmc.run();
}
