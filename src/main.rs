extern crate tinytemplate;

use serde_derive::Serialize;
use tinytemplate::TinyTemplate;
use std::fs::File;
use std::io::Read;

mod immediate;

#[derive(Serialize, Debug)]
struct Context {
    imm_high_20: String,
    imm_jal_form: String,
    imm_low_12: String,
    imm_branch_high: String,
    imm_branch_low: String,
    imm_store_high: String,
    imm_store_low: String,
    imm_shift_amount: String,
    rs1: String,
    rs2: String,
    rd: String,
}


struct Register {
    id: u8
}

impl Register {
    fn binary_form(&self) -> String {
        return format!("{:05b}", self.id);
    }
}

struct Imm {
    value: [u8; 32]
}

impl Imm {
    fn from_string(string: String) -> Imm {
        if string.len() != 32 {
            panic!("Faq");
        } else {
            let mut value: [u8; 32] = [0; 32];
            for (i, element) in string.into_bytes().iter().rev().enumerate() {
                value[i] = element.clone()
            }
            Imm { value }
        }
    }
    fn from_u32(number: u32) -> Imm {
        Imm::from_string(format!("{:032b}", number))
    }
    fn high_20(&self) -> String {
        String::from_utf8(self.value[12..32].iter()
            .map(|x| x.to_owned())
            .rev()
            .collect::<Vec<u8>>())
            .expect("faq")
    }
    fn jal_form(&self) -> String {
        return format!("{}{}{}{}",
                       self.value[20] as char,
                       String::from_utf8(self.value[1..11].iter().rev().map(|x| x.to_owned()).collect::<Vec<u8>>()).expect("faq"),
                       self.value[11] as char,
                       String::from_utf8(self.value[12..20].iter().rev().map(|x| x.to_owned()).collect::<Vec<u8>>()).expect("faq"));
    }
    fn low_12(&self) -> String {
        return String::from_utf8(self.value[0..12].iter()
            .map(|x| x.to_owned())
            .rev()
            .collect::<Vec<u8>>())
            .expect("faq");
    }
    fn branch_high(&self) -> String {
        return format!("{}{}",
                       self.value[12] as char,
                       String::from_utf8(self.value[5..11].iter().rev().map(|x| x.to_owned()).collect::<Vec<u8>>()).expect("faq"));
    }
    fn branch_low(&self) -> String {
        return format!("{}{}",
                       String::from_utf8(self.value[1..5].iter().rev().map(|x| x.to_owned()).collect::<Vec<u8>>()).expect("faq"),
                       self.value[5] as char);
    }
    fn store_high(&self) -> String {
        return String::from_utf8(self.value[5..12].iter().rev().map(|x| x.to_owned()).collect::<Vec<u8>>()).expect("faq");
    }
    fn store_low(&self) -> String {
        String::from_utf8(self.value[0..5].iter().rev().map(|x| x.to_owned()).collect::<Vec<u8>>()).expect("faq")
    }
    fn shift_amount(&self) -> String {
        self.store_low()
    }
}

fn construct_context(param_list_str: String) -> Context {
    let param_list: Vec<String> = param_list_str
        .replace("(", ",")
        .replace(")", ",")
        .split(',')
        .map(|x| x.trim().to_string())
        .collect();
    let registers: Vec<String> = param_list.iter()
        .filter(|x| x.starts_with('x'))
        .map(|x| x.trim_start_matches('x').to_string())
        .collect();
    let imm_str = param_list.iter().find(|x| x.starts_with("0x")).map(|x| x.to_owned());
    let imm = imm_str.map_or(Imm::from_u32(0), |x| Imm::from_u32(i32::from_str_radix(&x.trim()[2..], 16).expect("faq") as u32));
    let rd = format!("{:05b}", registers[0].parse::<u32>().expect("faq"));
    let rs1 = if registers.len() > 1 {
        format!("{:05b}", registers[1].parse::<u32>().expect("faq"))
    } else {
        "".to_string()
    };
    let rs2 = if registers.len() > 2 {
        format!("{:05b}", registers[2].parse::<u32>().expect("faq"))
    } else {
        "".to_string()
    };
    Context {
        imm_high_20: imm.high_20(),
        imm_jal_form: imm.jal_form(),
        imm_low_12: imm.low_12(),
        imm_branch_high: imm.branch_high(),
        imm_branch_low: imm.branch_low(),
        imm_store_high: imm.store_high(),
        imm_store_low: imm.store_low(),
        imm_shift_amount: imm.shift_amount(),
        rs1,
        rs2,
        rd,
    }
}

fn translate_statement(template: &TinyTemplate, statement: String) -> String {
    let splitted: Vec<&str> = statement.splitn(2, ' ').collect();
    let context = construct_context(splitted[1].to_string());
    template.render(splitted[0], &context).expect("faq")
}

fn main() {
    let mut template = TinyTemplate::new();
    let mut f = File::open("./commands.spec").expect("faq");
    let mut s = String::new();
    f.read_to_string(&mut s);
    s.split('\n')
        .for_each(|x| {
            let splitted = x.split_whitespace().collect::<Vec<_>>();
            template.add_template(splitted[0], splitted[1]).expect("faq");
        });
    let mut input_file = File::open("./test.asm").expect("Fq");
    let mut source = String::new();
    input_file.read_to_string(&mut source);
    source.split("\n").for_each(|x| {
        println!("{}", translate_statement(&template, x.to_string()))
    });
}

