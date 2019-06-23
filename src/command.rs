use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_derive::Serialize;
use tinytemplate::TinyTemplate;

use crate::immediate::Immediate;
use crate::register::Register;

pub struct ASMCompiler<'a> {
    template: TinyTemplate<'a>
}

#[derive(Serialize, Debug)]
struct Context {
    imm_high_20: String,
    imm_low_12: String,
    imm_jal_form: String,
    imm_branch_high: String,
    imm_branch_low: String,
    imm_store_high: String,
    imm_store_low: String,
    imm_shift_amount: String,
    register_0: String,
    register_1: String,
    register_2: String,
}

impl<'a> ASMCompiler<'a> {
    pub fn new() -> ASMCompiler<'a> {
        lazy_static! {
            static ref FILE_CONTENT: String = {
                let mut spec_file = File::open("./spec/commands.spec").expect("No commands.spec found!");
                let mut result = String::new();
                spec_file.read_to_string(&mut result).expect("Read commands.spec failed!");
                result
            };
        };
        let mut result = TinyTemplate::new();
        FILE_CONTENT.trim().split('\n').for_each(|x| {
            let splitted = x.splitn(2, ' ').collect::<Vec<_>>();
            result.add_template(splitted[0], splitted[1].trim()).expect("Invalid commands.spec!");
        });
        ASMCompiler {
            template: result
        }
    }
    fn translate_command(&mut self, command: String, labels: &mut HashMap<String, u32>, program_counter: &mut u32) -> String {
        if command.ends_with(":") {
            labels.insert(command.trim_end_matches(":").to_string(), *program_counter);
            return "".to_string();
        }
        let splitted: Vec<String> = command
            .replace("(", ",")
            .replace(")", ",")
            .splitn(2, " ")
            .map(|x| x.to_string())
            .collect();
        let op = splitted[0].to_owned();
        let param_list = splitted[1]
            .split(",")
            .map(|x| x.trim())
            .filter(|x| x != &"");
        let (label_params, rest_params): (Vec<&str>, Vec<&str>) = param_list
            .partition(|param: &&str| labels.get(&param.to_string()).is_some());
        let (immediate_params, register_params): (Vec<&str>, Vec<&str>) =
            rest_params.iter()
                .partition(|param| param.as_bytes()[0] as char == '-' || '0' <= param.as_bytes()[0] as char && param.as_bytes()[0] as char <= '9');
        let imm = if !label_params.is_empty() {
            let offset = *labels.get(label_params[0]).unwrap() as i64 - *program_counter as i64;
            Immediate::from(format!("{}", offset))
        } else {
            Immediate::from(if immediate_params.len() > 0 { immediate_params[0].to_string() } else { "0".to_string() })
        };
        let registers: Vec<Register> = register_params
            .iter()
            .map(|x| Register::from(x.to_string()))
            .collect();
        let context = Context {
            imm_high_20: imm.high_20(),
            imm_low_12: imm.low_12(),
            imm_jal_form: imm.jal_form(),
            imm_branch_high: imm.branch_high(),
            imm_branch_low: imm.branch_low(),
            imm_store_high: imm.store_high(),
            imm_store_low: imm.store_low(),
            imm_shift_amount: imm.shift_amount(),
            register_0: if registers.len() > 0 { registers[0].binary_form() } else { "No such command!".to_string() },
            register_1: if registers.len() > 1 { registers[1].binary_form() } else { "No such command!".to_string() },
            register_2: if registers.len() > 2 { registers[2].binary_form() } else { "No such command!".to_string() },
        };
        if !command.ends_with(":") {
            *program_counter += 4;
        }
        self.template.render(op.as_str(), &context).expect("Failed to render")
    }
    pub fn compile(&mut self, source_file: &mut File) {
        let mut source = String::new();
        source_file.read_to_string(&mut source).expect("Cannot read sourcefile!");
        let mut program_counter: u32 = 0;
        let mut labels = HashMap::new();
        source.split("\n").for_each(|x| {
            println!("{}", self.translate_command(x.trim().to_string(), &mut labels, &mut program_counter))
        });
    }
}

