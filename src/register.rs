use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct Register {
    id: u8
}

impl From<String> for Register {
    fn from(source: String) -> Self {
        lazy_static! {
            static ref NAME_ID_MAP: HashMap<String, Register> = {
                let mut result = HashMap::new();
                let mut spec_file = File::open("./spec/registers.spec").expect("No registers.spec found!");
                let mut file_content = String::new();
                spec_file.read_to_string(&mut file_content).expect("Cannot read source file");
                file_content.split('\n')
                    .for_each(|register_line| {
                        let splitted = register_line.splitn(2, ' ').collect::<Vec<_>>();
                        let id = u8::from_str_radix(&splitted[0], 10).expect("Invalid registers.spec!");
                        splitted[1].split(',')
                            .for_each(|register_name| {
                                result.insert(register_name.trim().to_string(), Register {id});
                            });
                    });
                result
            };
        }
        return NAME_ID_MAP.get(&source).expect("Invalid register name!").clone();
    }
}

impl Register {
    pub fn binary_form(&self) -> String {
        return format!("{:05b}", self.id);
    }
}