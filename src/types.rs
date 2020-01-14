use crate::core::runtime::{Value, ProcExecution};
use std::io::{Error, ErrorKind};
use crate::stdprocs as procs;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ETVoid;
impl Value for ETVoid {
    fn literal(&self) -> String {
        return "".to_owned();
    }
}

#[derive(Copy, Clone)]
pub struct ETInt(pub i32);
impl Value for ETInt {
    fn int(&self) -> Option<Box<Self>> {
        return Some(Box::new(*self));
    }
    fn literal(&self) -> String {
        return self.0.to_string();
    }
}
impl ETInt {
    pub fn new(s : String) -> Result<Self, Error> {
        return match s.parse::<i32>() {
            Ok(n) => Ok(ETInt(n)),
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Error parsing integer")),
        };
    }
}

#[derive(Copy, Clone)]
pub struct ETFloat(pub f64);
impl Value for ETFloat {
    fn float(&self) -> Option<Box<Self>> {
        return Some(Box::new(*self));
    }
    fn literal(&self) -> String {
        return self.0.to_string();
    }
}
impl ETFloat {
    pub fn new(s : String) -> Result<Self, Error> {
        return match s.parse::<f64>() {
            Ok(n) => Ok(ETFloat(n)),
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Error parsing integer")),
        };
    }
}

#[derive(Clone)]
pub struct ETList(pub Vec<Box<dyn Value>>);
impl Value for ETList {
    fn list(&self) -> Option<Box<Self>> {
        return Some(Box::new(ETList(self.0.clone())));
    }
    fn literal(&self) -> String {
        return "TODO: LIST Literal".to_owned();
    }
    fn function(&self) -> Option<Box<dyn ProcExecution>> {
        return Some(Box::new(procs::EPGet{}));
    }
}
impl ETList {
    pub fn new(v : Box<dyn Value>) -> Self {
        return ETList(vec![v]);
    }

    pub fn add(&mut self, val : Box<dyn Value>) {
        self.0.push(val);
    }

    pub fn len(self) -> usize {
        self.0.len()
    }

    pub fn get(&self, idx : usize) -> Result<Box<dyn Value>, Error> {
        return match self.0.get(idx) {
            Some(n) => Ok(n.clone()),
            None => Err(Error::new(ErrorKind::InvalidData, "Index out bounds"))
        }
    }
}

#[derive(Clone)]
pub struct ETMap(pub HashMap<String, Box<dyn Value>>);
impl Value for ETMap {
    fn map(&self) -> Option<Box<Self>> {
        return Some(Box::new(self.clone()));
    }
    fn literal(&self) -> String {
        return "TODO: MAP Literal".to_owned();
    }
    fn function(&self) -> Option<Box<dyn ProcExecution>> {
        return Some(Box::new(procs::EPGet{}));
    }
}
impl ETMap {
    pub fn new(k : String, v : Box<dyn Value>) -> Self {
        return ETMap({
            let mut x = HashMap::new();
            x.insert(k, v);
            x
        });
    }

    pub fn add(&mut self, k : String, v : Box<dyn Value>) {
        self.0.insert(k, v);
    }

    pub fn get<'a>(&self, idx : &'a str) -> Result<Box<dyn Value>, Error> {
        return match self.0.get(idx) {
            Some(n) => Ok(n.clone()),
            None => Err(Error::new(ErrorKind::InvalidData, "Invalid key"))
        }
    }
}

#[derive(Clone)]
pub struct ETString(pub String); //Literal Value
impl Value for ETString {    
    fn literal(&self) -> String {
        return self.0.clone();
    }

    fn stringval(&self) -> Option<Box<Self>> {
        return Some(Box::new(self.clone()));
    }

    fn list(&self) -> Option<Box<ETList>> {
        return Some(Box::new(ETList({
            let mut res = Vec::<Box<dyn Value>>::new();
            for v in self.0.clone().chars() {
                res.push(Box::new(ETString(v.to_string())))
            }
            res
        })))
    }
}

#[derive(Clone)]
pub struct ETLiteral(pub String); //Literal Value(Always typed by the user)
impl Value for ETLiteral {
    fn is_literal(&self) -> bool {
        true
    }

    fn literal(&self) -> String {
        return self.0.clone();
    }
}
impl ETLiteral {
    pub fn literal_array<'a>(data : &'a Vec<String>) -> Vec<Box<dyn Value>> {
        let mut res = Vec::new();
        for x in data {
            res.push(Box::new(ETLiteral(String::from(x))) as Box<dyn Value>);
        }
        return res;
    }
}

#[derive(Clone)]
pub struct ETBlock(pub crate::core::Block);
impl Value for ETBlock {
    fn literal(&self) -> String {
        return "TODO: BLOCK Literal".to_owned();
    }

    fn block(&self) -> Option<Box<ETBlock>> {
        return Some(Box::new(self.clone()));
    }
}

#[derive(Clone)]
pub struct ETAlias(pub Box<dyn Value>, pub Box<dyn Value>); //0 is the mask that works as type but the function will be attached to 1
impl Value for ETAlias {
    fn list(&self) -> Option<Box<ETList>> {
        self.0.list()
    }
    fn map(&self) -> Option<Box<ETMap>> {
        self.0.map()
    }
    fn int(&self) -> Option<Box<ETInt>> {
        self.0.int()
    }
    fn float(&self) -> Option<Box<ETFloat>> {
        self.0.float()
    }
    fn stringval(&self) -> Option<Box<ETString>> {
        self.0.stringval()
    }
    fn literal(&self) -> String {
        self.0.literal()
    }
    fn function(&self) -> Option<Box<dyn ProcExecution>> {
        self.1.function()
    }
    fn block(&self) -> Option<Box<ETBlock>> {
        self.0.block()
    }
    fn target(&self) -> Box<dyn Value> {
        self.1.clone_box()
    }
}

pub fn join_values<'a>(a : Vec<Box<dyn Value>>, b : Vec<Box<dyn Value>>) -> Vec<Box<dyn Value>> {
    let mut res : Vec<Box<dyn Value>> = Vec::new();
    for i in a {
        res.push(i);
    }
    for i in b {
        res.push(i);
    }
    return res;
}