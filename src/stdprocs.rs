use crate::core::runtime::{RunningInstance, ProcExecution, Value, Context};
use std::io::{Error, ErrorKind};
use crate::types;
use std::collections::HashMap;

pub enum StrictType {
    Integer,
    Float,
    Char,
    List,
    Map
}

pub enum LiteralParsableType {
    Integer,
    Float,
    Char,
}

pub fn assert_len(act : usize, exp : usize) -> Option<Error> {
    if act != exp {
        let r : String = format!("Expected {} and got {}", exp, act);
        return Some(Error::new(ErrorKind::Other, r));
    }
    return None;
}

pub fn assert_type(data : &Box<dyn Value>, exp : StrictType) -> Option<Error> {
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
    return Some(Error::new(ErrorKind::Other, expected + " type expected"));
}

pub fn assert_type_lit(data : String, exp : LiteralParsableType) -> Option<Error> {
    let mut expected = String::new();
    match exp {
        LiteralParsableType::Integer => if let Err(_) = types::ETInt::new(data) {expected.push_str("Integer")}
        LiteralParsableType::Float => if let Err(_) = types::ETFloat::new(data) {expected.push_str("Float")}
        LiteralParsableType::Char => if data.chars().count() != 1 {expected.push_str("Char")}
    }
    if expected.is_empty() {
        return None;
    }
    return Some(Error::new(ErrorKind::Other, expected + " type expected"));
}

pub fn expect_int(v : &Box<dyn Value>) -> Result<Box<types::ETInt>, Error> {
    if let Some(_) = assert_type(v, StrictType::Integer) {
        if let Some(e) = assert_type_lit(v.literal(), LiteralParsableType::Integer) {
            return Err(e);
        }
        return Ok(Box::new(types::ETInt::new(v.literal())?))
    }
    return Ok(v.int().unwrap());
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct EPMap;
impl ProcExecution for EPMap {
    fn name(&self) -> String {
        "MAP".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 2) {
            if input.len() == 3 {
                let mapc = match c.variables.get(&input[0].literal()) {
                    Some(x) => x.clone(),
                    None => Box::new(types::ETMap(HashMap::new())),
                };
                if let Some(e) = assert_type(&mapc, StrictType::Map) {
                    return Err(e);
                }
                let mut map = mapc.map().unwrap();
                map.add(input[1].literal(), input[2].clone());
                c.variables.insert(input[0].literal(), map.clone());
                return Ok(map);
            } else {
                return Err(n);
            }
        }
        return Ok(Box::new(types::ETMap::new(input[0].literal(), input[1].clone())));
    }
}

#[derive(Clone)]
pub struct EPGet;
impl ProcExecution for EPGet {
    fn name(&self) -> String {
        "GET".to_owned()
    }

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 2) {
            return Err(n);
        }
        return match input[0].list() {
            Some(n) => Ok(n.get(expect_int(&input[1])?.0.clone() as usize)?.clone()),
            None => {
                match input[0].map() {
                    Some(n) => Ok(n.get(&input[1].literal())?),
                    None => Err(Error::new(ErrorKind::InvalidInput, "Expected list or map")),
                }
            }
        }
    }
}

pub fn get_standard_procs() -> Vec<Box<dyn ProcExecution>> {
    return vec![
        Box::new(EPDisplay{}),
        Box::new(EPReturn{}),
        Box::new(EPInt{}),
        Box::new(EPLit{}),
        Box::new(EPFloat{}),
        Box::new(EPLst{}),
        Box::new(EPMap{}),
        Box::new(EPGet{})
    ];
}