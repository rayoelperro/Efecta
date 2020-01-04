use crate::core::runtime::Value;
use std::io::{Error, ErrorKind};

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
pub struct ETFloat(f64);
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
pub struct ETMap(i32);
#[derive(Clone)]
pub struct ETList(i32);

#[derive(Clone)]
pub struct ETString(pub String); //Literal Value
impl Value for ETString {
    fn literal(&self) -> String {
        return self.0.clone();
    }
}
impl ETString {
    pub fn literal_array<'a>(data : &'a Vec<String>) -> Vec<Box<dyn Value>> {
        let mut res = Vec::new();
        for x in data {
            res.push(Box::new(ETString(String::from(x))) as Box<dyn Value>);
        }
        return res;
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