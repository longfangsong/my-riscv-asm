#[macro_use]
extern crate lazy_static;
extern crate tinytemplate;

use std::fs::File;

use crate::command::ASMCompiler;

mod immediate;
mod register;
mod command;

fn main() {
    let mut compiler = ASMCompiler::new();
    let mut input_file = File::open("./test.asm").expect("Source file not found!");
    compiler.compile(&mut input_file);
}

