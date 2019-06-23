#[macro_use]
extern crate lazy_static;
extern crate tinytemplate;

use std::fs::File;
use std::io::Read;

use crate::command::ASMCompiler;

mod immediate;
mod register;
mod command;

fn main() {
    let compiler = ASMCompiler::new();
    let mut input_file = File::open("./test.asm").expect("Source file not found!");
    let mut source = String::new();
    input_file.read_to_string(&mut source).expect("Cannot read sourcefile!");
    source.split("\n").for_each(|x| {
        println!("{}", compiler.translate_command(x.to_string()))
    });
}

