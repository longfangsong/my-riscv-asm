use std::collections::HashMap;

use serde_json::Value;

use crate::param::ParsedParam;

fn bit_at(num: u32, i: usize) -> bool {
    num & (1 << i) as u32 != 0
}

fn bits_at<I>(num: u32, range: I) -> u32
where
    I: IntoIterator,
    I::Item: Into<usize>,
{
    let mut result: u32 = 0;
    for (i, bit_id) in range.into_iter().enumerate() {
        result |= (bit_at(num, bit_id.into()) as u32) << i as u32;
    }
    result
}

pub fn filter_bits_at(n: &Value, params: &HashMap<String, Value>) -> tera::Result<Value> {
    let n: ParsedParam = serde_json::from_value(n.clone()).unwrap();
    let n = n.unwrap_immediate() as u32;
    if let Some(indexes) = params.get("indexes") {
        let bits = indexes
            .as_array()
            .unwrap()
            .iter()
            .map(|it| it.as_u64().unwrap() as usize);
        let width = bits.len();
        let result = bits_at(n, bits);
        Ok(Value::String(format!("{result:0width$b}")))
    } else if let (Some(start), Some(end)) = (params.get("start"), params.get("end")) {
        let start = start.as_u64().unwrap() as usize;
        let end = end.as_u64().unwrap() as usize;
        let bits = start..end;
        let width = bits.len();
        let result = bits_at(n, bits);
        Ok(Value::String(format!("{result:0width$b}")))
    } else {
        Err(tera::Error::msg(
            "indexes or start and end must be provided",
        ))
    }
}

pub fn lift_imm_filter(
    f: impl Fn(u32) -> String,
) -> impl Fn(&Value, &HashMap<String, Value>) -> tera::Result<Value> {
    move |num: &Value, _: &HashMap<String, Value>| {
        let num: ParsedParam = serde_json::from_value(num.clone()).unwrap();
        Ok(Value::String(f(num.unwrap_immediate() as u32)))
    }
}

pub fn register_filter(i: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let param: ParsedParam = serde_json::from_value(i.clone()).unwrap();
    Ok(Value::String(format!("{:05b}", param.unwrap_register())))
}

pub fn csr_filter(i: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let param: ParsedParam = serde_json::from_value(i.clone()).unwrap();
    Ok(Value::String(format!("{:012b}", param.unwrap_csr())))
}

pub fn jal_form(n: u32) -> String {
    let mut bit_select: Vec<usize> = vec![];
    bit_select.extend(12..20);
    bit_select.push(11);
    bit_select.extend(1..11);
    bit_select.push(20);
    format!("{:020b}", bits_at(n, bit_select))
}

pub fn branch_high(n: u32) -> String {
    let mut bit_select: Vec<usize> = vec![];
    bit_select.extend(5..11);
    bit_select.push(12);
    format!("{:07b}", bits_at(n, bit_select))
}

pub fn branch_low(n: u32) -> String {
    let mut bit_select: Vec<usize> = vec![11];
    bit_select.extend(1..5);
    format!("{:05b}", bits_at(n, bit_select))
}
