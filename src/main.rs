#![feature(once_cell)]

use std::{
    collections::HashMap,
    io::{stdin, Read},
    lazy::SyncLazy,
};
use tera::{Context, Tera};

mod filter;
mod param;
use crate::{filter::*, param::ParsedParam};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnparsedInstruction {
    name: String,
    params: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Line {
    Tag(String),
    Instruction(UnparsedInstruction),
}

fn preprocess(code: &str) -> Vec<Line> {
    let mut result = Vec::new();
    for line in code.lines().map(|it| it.trim()).filter(|it| !it.is_empty()) {
        if line.ends_with(':') {
            result.push(Line::Tag(line.trim_end_matches(':').to_string()));
        } else {
            result.push(parse_instruction_line(line));
        }
    }
    result
}

fn parse_instruction_line(line: &str) -> Line {
    let (name, params) = line.split_once(' ').unwrap_or((line, ""));
    let params = params
        .replace('(', ",")
        .replace(')', " ")
        .split(',')
        .map(|it| it.trim().to_string())
        .collect();
    Line::Instruction(UnparsedInstruction {
        name: name.to_string(),
        params,
    })
}

fn replace_complex_pseudo(preprocessed: &[Line]) -> Vec<Line> {
    let mut result = Vec::new();
    for line in preprocessed {
        match line {
            Line::Tag(tag) => result.push(Line::Tag(tag.to_string())),
            Line::Instruction(UnparsedInstruction { name, params }) => match name.as_str() {
                "li" => {
                    let param: i32 = parse_int::parse(&params[1]).unwrap();
                    let higher = if param & (1 << 11) == 0 {
                        param >> 12
                    } else {
                        (param >> 12) + 1
                    };
                    let lower = param - (higher << 12);
                    if higher == 0 && lower == 0 {
                        result.push(Line::Instruction(UnparsedInstruction {
                            name: "mv".to_string(),
                            params: vec![params[0].clone(), "zero".to_string()],
                        }))
                    } else {
                        if higher != 0 {
                            result.push(Line::Instruction(UnparsedInstruction {
                                name: "lui".to_string(),
                                params: vec![params[0].clone(), format!("0x{:x}", higher)],
                            }));
                        }
                        if lower != 0 {
                            result.push(Line::Instruction(UnparsedInstruction {
                                name: "addi".to_string(),
                                params: vec![
                                    params[0].clone(),
                                    params[0].clone(),
                                    format!("{}", lower),
                                ],
                            }));
                        }
                    }
                }
                _ => result.push(Line::Instruction(UnparsedInstruction {
                    name: name.to_string(),
                    params: params.clone(),
                })),
            },
        }
    }
    result
}

fn replace_simple_pseudo(complex_replaced: &[Line]) -> Vec<Line> {
    static PSEUDO_SIMPLE_INSTRUCTIONS: SyncLazy<Tera> = SyncLazy::new(|| {
        let mut result = Tera::default();
        let templates_str = include_str!("../spec/pseudo_simple.spec").trim();
        let templates = templates_str
            .split('\n')
            .map(|it| it.trim())
            .filter(|it| !it.is_empty());
        for template in templates {
            let (name, template) = template.split_once(' ').unwrap();
            result.add_raw_template(name, template.trim()).unwrap();
        }
        result
    });
    let mut result = Vec::new();
    for line in complex_replaced {
        match line {
            Line::Tag(tag) => result.push(Line::Tag(tag.clone())),
            Line::Instruction(UnparsedInstruction { name, params }) => {
                if PSEUDO_SIMPLE_INSTRUCTIONS
                    .templates
                    .contains_key(name.as_str())
                {
                    let mut context = Context::new();
                    context.insert("params", &params);
                    let new_instruction =
                        PSEUDO_SIMPLE_INSTRUCTIONS.render(name, &context).unwrap();
                    result.push(parse_instruction_line(&new_instruction));
                } else {
                    result.push(Line::Instruction(UnparsedInstruction {
                        name: name.clone(),
                        params: params.clone(),
                    }));
                }
            }
        }
    }
    result
}

fn assign_address(
    fixed_width_instructions: &[Line],
) -> (Vec<UnparsedInstruction>, HashMap<String, i32>) {
    let mut result_instructions = Vec::new();
    let mut result_tags = HashMap::new();
    let mut address = 0;
    for line in fixed_width_instructions {
        match line {
            Line::Tag(tag) => {
                result_tags.insert(tag.clone(), address);
            }
            Line::Instruction(instruction) => {
                result_instructions.push(instruction.clone());
                address += 4;
            }
        }
    }
    (result_instructions, result_tags)
}

fn render(instructions: &[UnparsedInstruction], labels: &HashMap<String, i32>) -> Vec<u32> {
    static COMMANDS_TEMPLATE: SyncLazy<Tera> = SyncLazy::new(|| {
        let mut result = Tera::default();
        let templates_str = include_str!("../spec/instructions.spec").trim();
        let templates = templates_str
            .split('\n')
            .map(|it| it.trim())
            .filter(|it| !it.is_empty());
        for template in templates {
            let (name, template) = template.split_once(' ').unwrap();
            result.add_raw_template(name, template.trim()).unwrap();
        }
        result.register_filter("bits_at", filter_bits_at);
        result.register_filter("imm_jal_form", filter::lift_imm_filter(jal_form));
        result.register_filter("imm_branch_high", filter::lift_imm_filter(branch_high));
        result.register_filter("imm_branch_low", filter::lift_imm_filter(branch_low));
        result.register_filter("register", filter::register_filter);
        result.register_filter("csr", filter::csr_filter);
        result.register_function("offset", |args: &HashMap<String, tera::Value>| {
            let from = serde_json::from_value(args.get("from").unwrap().clone())
                .map(|it: ParsedParam| it.unwrap_immediate())
                .unwrap_or_else(|_| args.get("from").unwrap().as_i64().unwrap() as i32);
            let to = serde_json::from_value(args.get("to").unwrap().clone())
                .map(|it: ParsedParam| it.unwrap_immediate())
                .unwrap_or_else(|_| args.get("to").unwrap().as_i64().unwrap() as i32);
            Ok(serde_json::to_value(ParsedParam::Immediate(to - from)).unwrap())
        });
        result
    });

    let mut result = Vec::new();
    for (index, UnparsedInstruction { name, params }) in instructions.iter().enumerate() {
        let address = index * 4;
        let mut context = Context::new();
        let params: Vec<ParsedParam> = params.iter().map(|it| parse_param(it, labels)).collect();
        context.insert("params", &params);
        context.insert("address", &address);
        let binary_form = COMMANDS_TEMPLATE.render(name, &context).unwrap();
        result.push(u32::from_str_radix(&binary_form, 2).unwrap());
    }
    result
}

fn parse_param(code_param: &str, labels: &HashMap<String, i32>) -> ParsedParam {
    static REGISTERS: SyncLazy<HashMap<&'static str, u8>> = SyncLazy::new(|| {
        let mut registers = HashMap::new();
        let registers_str = include_str!("../spec/registers.spec");
        for line in registers_str
            .lines()
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
        {
            let (index, names) = line.split_once(' ').unwrap();
            let names = names.split(',').map(|it| it.trim());
            for name in names {
                registers.insert(name, index.parse::<u8>().unwrap());
            }
        }
        registers
    });

    static CSRS: SyncLazy<HashMap<&'static str, u16>> = SyncLazy::new(|| {
        let mut csrs = HashMap::new();
        let csrs_str = include_str!("../spec/csr.spec");
        for line in csrs_str
            .lines()
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
        {
            let (name, address) = line.split_once(' ').unwrap();
            csrs.insert(name, parse_int::parse(address).unwrap());
        }
        csrs
    });

    if let Some(register_id) = REGISTERS.get(code_param) {
        ParsedParam::Register(*register_id)
    } else if let Some(csr_id) = CSRS.get(code_param) {
        ParsedParam::Csr(*csr_id)
    } else if let Ok(imm) = parse_int::parse(code_param) {
        ParsedParam::Immediate(imm)
    } else if let Some(imm) = labels.get(code_param) {
        ParsedParam::Immediate(*imm)
    } else {
        panic!("unknown parameter: {}", code_param);
    }
}

fn compile(code: &str) -> Vec<u32> {
    let preprocessed = preprocess(code);
    let replace_complex_pseudo_done = replace_complex_pseudo(&preprocessed);
    let replace_simple_pseudo_done = replace_simple_pseudo(&replace_complex_pseudo_done);
    let (instructions, labels) = assign_address(&replace_simple_pseudo_done);
    render(&instructions, &labels)
}

fn main() {
    let mut input = Vec::new();
    stdin().lock().read_to_end(&mut input).unwrap();
    compile(&String::from_utf8(input).unwrap())
        .iter()
        .for_each(|it| println!("{:x}", it));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_cases(code: &'static str) {
        struct Case {
            expected: u32,
            code: &'static str,
        }
        let cases = code
            .split('\n')
            .map(|it| it.trim())
            .filter(|it| !it.is_empty())
            .map(|it| it.split_once(' ').unwrap())
            .map(|(expected, code)| Case {
                expected: u32::from_str_radix(expected, 16).unwrap(),
                code,
            });
        for Case { expected, code } in cases {
            assert_eq!(compile(code), vec![expected]);
        }
    }
    
    #[test]
    fn test_simple_instructions() {
        check_cases(include_str!("../test_cases/instructions.cases"));
    }

    #[test]
    fn test_simple_pseudo_instructions() {
        check_cases(include_str!("../test_cases/pseudo_simple.cases"));
    }
}
