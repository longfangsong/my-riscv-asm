use core::fmt;
use std::fmt::{Error, Formatter};

pub struct Immediate {
    pub content: u32
}

impl fmt::Binary for Immediate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fmt::Binary::fmt(&self.content, f)
    }
}

impl From<i32> for Immediate {
    fn from(content: i32) -> Self {
        Immediate {
            content: content as u32
        }
    }
}

impl From<String> for Immediate {
    fn from(source: String) -> Self {
        let negative = source.starts_with("-");
        let positive_part = if negative {
            source[1..].to_string()
        } else {
            source
        };
        let result = (if negative { -1 } else { 1 }) *
            (if positive_part.len() > 2 {
                match &&positive_part[..2] {
                    &"0x" => i32::from_str_radix(&positive_part[2..], 16),
                    &"0b" => i32::from_str_radix(&positive_part[2..], 2),
                    &"0o" => i32::from_str_radix(&positive_part[2..], 8),
                    _ => i32::from_str_radix(&positive_part, 10),
                }
            } else {
                i32::from_str_radix(&positive_part, 10)
            }).expect("Invalid imm format!");
        Immediate::from(result)
    }
}

impl Immediate {
    fn bit_at(&self, i: usize) -> bool {
        self.content & (1 << i) as u32 != 0
    }
    fn bits_at<I>(&self, range: I) -> u32
        where I: IntoIterator,
              I::Item: Into<usize> {
        let mut result: u32 = 0;
        for (i, bit_id) in range.into_iter().enumerate() {
            result |= (self.bit_at(bit_id.into()) as u32) << i as u32;
        }
        result
    }
    pub fn high_20(&self) -> String {
        format!("{:020b}", self.bits_at(12 as usize..32))
    }
    pub fn low_12(&self) -> String {
        format!("{:012b}", self.bits_at(0 as usize..12))
    }
    pub fn jal_form(&self) -> String {
        let mut bit_select: Vec<usize> = vec![];
        bit_select.extend(12..20);
        bit_select.push(11);
        bit_select.extend(1..11);
        bit_select.push(20);
        format!("{:020b}", self.bits_at(bit_select))
    }
    pub fn branch_high(&self) -> String {
        let mut bit_select: Vec<usize> = vec![];
        bit_select.extend(5..11);
        bit_select.push(12);
        format!("{:07b}", self.bits_at(bit_select))
    }
    pub fn branch_low(&self) -> String {
        let mut bit_select: Vec<usize> = vec![];
        bit_select.push(11);
        bit_select.extend(1..5);
        format!("{:05b}", self.bits_at(bit_select))
    }
    pub fn store_high(&self) -> String {
        format!("{:07b}", self.bits_at(5 as usize..12))
    }
    pub fn store_low(&self) -> String {
        format!("{:05b}", self.bits_at(0 as usize..5))
    }
    pub fn shift_amount(&self) -> String {
        self.store_low()
    }
}