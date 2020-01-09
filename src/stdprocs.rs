use crate::core::runtime::{RunningInstance, ProcExecution, Value, Context};
use std::io::{Error, ErrorKind};
use crate::types;
use std::collections::HashMap;

pub enum StrictType {
    Integer,
    Float,
    Char,
    List,
    Map,
    Literal
}

pub enum LiteralParsableType {
    Integer,
    Float,
    Char,
}

fn value_from_type(stype : StrictType) -> Box<dyn Value> {
    match stype {
        StrictType::Integer => Box::new(types::ETInt(0)),
        StrictType::Float => Box::new(types::ETFloat(0.0)),
        StrictType::Char => Box::new(types::ETString(std::char::from_u32(0).unwrap().to_string())),
        StrictType::List => Box::new(types::ETList(Vec::new())),
        StrictType::Map => Box::new(types::ETMap(HashMap::new())),
        StrictType::Literal => Box::new(types::ETString(String::new())),
    }
}

impl<'a> Context<'a> {    
    fn expect_variable(&self, name : String, stype : StrictType) -> Result<Box<dyn Value>, Error> {
        match self.get_var(&name) {
            Ok(v) => if let Some(_) = assert_type(&v, stype) {
                Err(Error::new(ErrorKind::InvalidInput, "The variable does not match the expected type"))
            } else {
                Ok(v)
            }
            Err(_) => Ok(value_from_type(stype))
        }
    }
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
        StrictType::Literal => {}
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

pub fn expect_float(v : &Box<dyn Value>) -> Result<Box<types::ETFloat>, Error> {
    if let Some(_) = assert_type(v, StrictType::Float) {
        if let Some(e) = assert_type_lit(v.literal(), LiteralParsableType::Float) {
            return Err(e);
        }
        return Ok(Box::new(types::ETFloat::new(v.literal())?))
    }
    return Ok(v.float().unwrap());
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

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            if input.len() == 2 {
                let mut i = c.expect_variable(input[0].literal(), StrictType::Integer)?.int().unwrap();
                i.0 += expect_int(&input[1])?.0;
                c.variables.insert(input[0].literal(), i.clone());
                return Ok(i);
            }
            return Err(n);
        }
        if let None = assert_type(&input[0], StrictType::Float) {
            return Ok(Box::new(types::ETInt(input[0].float().unwrap().0 as i32)));
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

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            if input.len() == 2 {
                let mut e = c.expect_variable(input[0].literal(), StrictType::Literal)?.stringval().unwrap();
                e.0.push_str(&input[1].literal());
                c.variables.insert(input[0].literal(), e.clone());
                return Ok(e);
            }
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

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            if input.len() == 2 {
                let mut f = c.expect_variable(input[0].literal(), StrictType::Float)?.float().unwrap();
                f.0 += expect_float(&input[1])?.0;
                c.variables.insert(input[0].literal(), f.clone());
                return Ok(f);
            }
            return Err(n);
        }
        if let None = assert_type(&input[0], StrictType::Integer) {
            return Ok(Box::new(types::ETFloat(input[0].int().unwrap().0 as f64)));
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

    fn run(&self, _ : &RunningInstance, input : Vec<Box<dyn Value>>, c : &mut Context) -> Result<Box<dyn Value>, Error> {
        if let Some(n) = assert_len(input.len(), 1) {
            if input.len() == 2 {
                let mut list = c.expect_variable(input[0].literal(), StrictType::List)?.list().unwrap();
                list.add(input[1].clone());
                c.variables.insert(input[0].literal(), list.clone());
                return Ok(list);
            }
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
                let mut map = c.expect_variable(input[0].literal(), StrictType::Map)?.map().unwrap();
                map.add(input[1].literal(), input[2].clone());
                c.variables.insert(input[0].literal(), map.clone());
                return Ok(map);
            }
            return Err(n);
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