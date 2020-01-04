use crate::core::runtime::{RunningInstance, ProcExecution, Value, Context};
use std::io::{Error, ErrorKind};
use crate::types;

enum StrictType {
    Integer,
    Float,
    Char,
    List,
    Map
}

enum LiteralParsableType {
    Integer,
    Float,
    Char,
}

fn assert_len(act : usize, exp : usize) -> Option<Error> {
    if act != exp {
        let r : String = format!("Expected {} and got {}", exp, act);
        return Some(Error::new(ErrorKind::Other, r));
    }
    return None;
}

fn assert_type(data : Box<dyn Value>, exp : StrictType) -> Option<Error> {
    let mut expected = String::new();
    match exp {
        StrictType::Integer => if let None = data.int() {expected.push_str("Integer")}
        StrictType::Float => if let None = data.float() {expected.push_str("Float")}
        StrictType::Char => if data.literal().chars().count() != 1 {expected.push_str("Char")}
        StrictType::List => if let None = data.list() {expected.push_str("List")}
        StrictType::Map => if let None = data.map() {expected.push_str("Map")}
    }
    if expected.is_empty() {
        return None;
    }
    return Some(Error::new(ErrorKind::Other, expected + " type expected"));;
}

fn assert_type_lit(data : String, exp : LiteralParsableType) -> Option<Error> {
    let mut expected = String::new();
    match exp {
        LiteralParsableType::Integer => if let Err(_) = types::ETInt::new(data) {expected.push_str("Integer")}
        LiteralParsableType::Float => if let Err(_) = types::ETFloat::new(data) {expected.push_str("Float")}
        LiteralParsableType::Char => if data.chars().count() != 1 {expected.push_str("Char")}
    }
    if expected.is_empty() {
        return None;
    }
    return Some(Error::new(ErrorKind::Other, expected + " type expected"));;
}

pub struct EPDisplay;
impl ProcExecution for EPDisplay {
    fn name(&self) -> String {
        "DISPLAY".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        println!("{}", input[0].literal());
        return Ok(Box::new(types::ETVoid));
    }
}

pub struct EPReturn;
impl ProcExecution for EPReturn {
    fn name(&self) -> String {
        "RETURN".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        c.ret = input[0].clone();
        return Ok(Box::new(types::ETVoid));
    }
}

pub struct EPArg;
impl ProcExecution for EPArg {
    fn name(&self) -> String {
        "ARG".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        let mut iv = input[0].clone();
        if let Some(_) = assert_type(input[0].clone(), StrictType::Integer) {
            if let Some(n) = assert_type_lit(input[0].literal(), LiteralParsableType::Integer) {
                return Err(n);
            } else {
                iv = Box::new(types::ETInt::new(input[0].literal())?);
            }
        }
        let x = iv.int().unwrap();
        return Ok(c.args[x.0 as usize].clone());
    }
}

pub struct EPInt;
impl ProcExecution for EPInt {
    fn name(&self) -> String {
        "INT".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        return Ok(Box::new(types::ETInt::new(input[0].literal())?));
    }
}

pub struct EPLit;
impl ProcExecution for EPLit {
    fn name(&self) -> String {
        "LIT".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        return Ok(Box::new(types::ETString(input[0].literal())));
    }
}

pub struct EPFloat;
impl ProcExecution for EPFloat {
    fn name(&self) -> String {
        "FLOAT".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        return Ok(Box::new(types::ETFloat::new(input[0].literal())?));
    }
}

pub struct EPLst;
impl ProcExecution for EPLst {
    fn name(&self) -> String {
        "LST".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            return Err(n);
        }
        return Ok(Box::new(types::ETList::new(input[0].clone())));
    }
}

pub fn get_standard_procs() -> Vec<Box<dyn ProcExecution>> {
    return vec![
        Box::new(EPDisplay{}),
        Box::new(EPReturn{}),
        Box::new(EPArg{}),
        Box::new(EPInt{}),
        Box::new(EPLit{}),
        Box::new(EPFloat{}),
        Box::new(EPLst{})
    ];
}