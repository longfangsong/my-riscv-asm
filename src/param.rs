use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ParsedParam {
    Register(u8),
    CSR(u16),
    Immediate(i32),
}

impl ParsedParam {
    pub fn unwrap_immediate(&self) -> i32 {
        match self {
            ParsedParam::Immediate(i) => *i,
            _ => panic!("Expected immediate!"),
        }
    }
    pub fn unwrap_register(&self) -> u8 {
        match self {
            ParsedParam::Register(r) => *r,
            _ => panic!("Expected register!"),
        }
    }
    pub fn unwrap_csr(&self) -> u16 {
        match self {
            ParsedParam::CSR(r) => *r,
            _ => panic!("Expected CSR!"),
        }
    }
}
