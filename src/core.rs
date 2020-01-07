#[derive(Clone)]
pub struct Block {
    pub subs : Vec<Block>,
    pub data : Vec<String>
}

#[derive(Clone)]
pub struct Proc {
    pub name : String,
    pub mems : Vec<Block>
}

#[derive(Clone)]
pub struct ProgramInstance {
    pub name : String,
    pub entry_point : String,
    pub methods : Vec<Proc>
}

pub mod lexer {

    use std::io::{Read, BufRead, BufReader, Error, ErrorKind};

    pub fn get_tokens(f : Box<dyn Read>) -> Result<Vec<Vec<String>>, Error> {
        let mut result = Vec::<Vec<String>>::new();
        let reader = BufReader::new(f);
        for (_, v) in reader.lines().enumerate() {
            let data = v?;
            let res = line_tokens(data)?;
            if res.len() > 0 {
                result.push(res);
            }
        }
        return Ok(result);
    }

    pub fn line_tokens(ln : String) -> Result<Vec<String>, Error> {
        let mut ret = Vec::<String>::new();
        let mut mxt = false;
        let mut act = String::new();
        let mut lit = false;
        for (l, v) in ln.chars().enumerate() {
            if lit {
                act.push(v);
            } else {
                match v {
                    '\t' => {
                        //TAB: FOR SUB STATEMENTS
                        if !mxt && act.chars().count() == 0 {
                            ret.push(v.to_string());
                        } else {
                            return Err(Error::new(ErrorKind::InvalidData, "The 'Tabs' must be at the beginning of the line"))
                        }
                    }
                    '#' => {
                        //LITERAL (GOOD FOR STRINGS)
                        if act.chars().count() > 0 {
                            ret.push(act.clone());
                        }
                        lit = true;
                    }
                    ';' => {
                        //AVOID NEXT (GOOD FOR COMMENTS)
                        if act.chars().count() > 0 {
                            ret.push(act.clone());
                        }
                        break;
                    }
                    '*' | '$' | '!' => {
                        mxt = true;
                        if act.chars().count() > 0 {
                            ret.push(act);
                            act = String::new();
                        }
                        ret.push(v.to_string());
                    }
                    ' ' => {
                        mxt = true;
                        if act.chars().count() > 0 {
                            ret.push(act);
                            act = String::new();
                        }
                    }
                    _ => {
                        mxt = true;
                        act.push(v);
                    }
                };
            }
            if l == ln.chars().count()-1 && act.chars().count() > 0{
                ret.push(act.clone());
            }
        }
        return Ok(ret);
    }
}

pub mod structure {

    use crate::core::{Block};
    use std::io::{Error, ErrorKind};

    pub fn generate_blocks(lines : Vec<Vec<String>>) -> Result<Vec<Block>, Error> {
        let mut res = Block{subs:Vec::new(),data:Vec::new()};
        for v in lines.iter() {
            let mut act = &mut res;
            let mut inc = 0;
            let mut trimed = Vec::<String>::new();
            for i in v.iter() {
                if i == "\t" {
                    inc += 1;
                } else {
                    trimed.push(String::from(i));
                }
            }
            let mut k = inc;
            while k > 0 {
                let idx = act.subs.len()-1;
                match act.subs.get_mut(idx) {
                    Some(n) => act = n,
                    None => return Err(Error::new(ErrorKind::InvalidData, "Too deep level"))
                };
                k -= 1;
            }
            act.subs.push(Block{subs:Vec::new(), data:trimed})
        }
        return Ok(res.subs);
    }
}

pub mod runtime {
    use std::collections::HashMap;
    use std::io::{Error, ErrorKind};
    use crate::core::{ProgramInstance, Proc, Block};
    use crate::types::{join_values, ETInt, ETFloat, ETList, ETMap, ETLiteral};

    pub struct RunningInstance {
        pub name : String,
        pub entry_point : String,
        pub methods : Vec<Box<dyn ProcExecution>>
    }

    fn parse_params(c : &mut Context, params : Vec<Box<dyn Value>>) -> Result<Vec<Box<dyn Value>>, Error> {
        let mut res = Vec::new();
        let mut nxc = false;
        for p in params.into_iter() {
            if p.is_literal() && (p.literal() == "$" || p.literal() == "!") {
                nxc = true;
            } else if nxc {
                res.push(c.get_var(&p.literal())?);
                nxc = false;
            } else {
                res.push(p);
            }
        }
        return Ok(res);
    }

    impl<'a> Block {
        pub fn head_is(&self, name : &'a str) -> bool {
            self.data[0] == name
        }

        pub fn cut_head(&self) -> (Self, usize, bool) {
            let mut data = self.data.clone();
            if data.len() < 1 {
                return (self.clone(), data.len(), false);
            }
            return (Block{subs:self.subs.clone(), data:data.drain(1..).collect()}, data.len()-1, true);
        }

        fn run(&self, c : &mut Context, proc_scope : bool) -> Result<Vec<Vec<Box<dyn Value>>>, Error> {
            let x = if self.data[0] == "*" || self.data[0] == "$" {1} else {0};
            if x == 1 && proc_scope {
                return Err(Error::new(ErrorKind::InvalidData, "Not necessary execution specifier"));
            } else if proc_scope || x == 1 {
                if let Some(n) = c.get_proc(self.data[0] == "$", &self.data[x])? {
                    let pr : Box<dyn ProcExecution> = n;
                    let mut result : Vec<Vec<Box<dyn Value>>> = Vec::new();
                    let mut args : Vec<Box<dyn Value>> =
                        ETLiteral::literal_array(&self.data.clone().drain((x+1)..).collect());
                    if self.data[0] == "$" {
                        if let Some(n) = c.variables.get(&self.data[x]) {
                            args.insert(0, n.clone());
                        }
                    }
                    if self.subs.len() > 0 {
                        for x in self.subs.iter() {
                            for v in x.run(c, false)? {
                                let res = parse_params(c, join_values(args.clone(), v.clone()))?;
                                let ret = pr.run(c.instance, res, c)?;
                                result.push(vec![ret]);
                            }
                        }
                    } else {
                        let ret = pr.run(c.instance, parse_params(c, args)?, c)?;
                        result.push(vec![ret])
                    }
                    return Ok(result);
                } else {
                    return Err(Error::new(ErrorKind::NotFound, self.data[x].clone() + " proc not found"))
                }
            } else {
                let mut total : Vec<Vec<Box<dyn Value>>> = Vec::new();
                let local : Vec<Box<dyn Value>> = ETLiteral::literal_array(&self.data.clone());
                if self.subs.len() > 0 {
                    for x in self.subs.iter() {
                        for v in x.run(c, false)? {
                            let res = join_values(local.clone(), v.clone());
                            total.push(res);
                        }
                    }
                } else {
                    total.push(local);
                }
                return Ok(total);
            }
        }
    }
    
    impl ProcExecution for Proc {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn run(&self, ins : &RunningInstance, input : Vec<Box<dyn Value>>, _ : &mut Context) -> Result<Box<dyn Value>, Error> {
            let mut context = Context::new(ins, input);
            for b in self.mems.iter() {
                if let Err(e) = b.run(&mut context, true) {
                    return Err(e);
                } else if !context.running {
                    break;
                }
            }
            return Ok(context.ret);
        }
    }

    impl RunningInstance {
        pub fn from(program : ProgramInstance, include : Vec<Box<dyn ProcExecution>>) -> Self {
            let mut allm : Vec<Box<dyn ProcExecution>> = include;
            for x in program.methods.into_iter() {
                allm.push(Box::new(x));
            }
            return RunningInstance{name:program.name, entry_point:program.entry_point, methods:allm};
        }
    }

    pub struct Context<'a> {
        pub instance : &'a RunningInstance,
        pub stack : Vec<String>,
        pub variables : HashMap<String, Box<dyn Value>>,
        pub ret : Box<dyn Value>,
        pub running : bool
    }

    impl<'a> Context<'a> {
        pub fn new(ins : &'a RunningInstance, input : Vec<Box<dyn Value>>) -> Self {
            let mut c = Context{instance:ins, stack:Vec::new(), variables:HashMap::new(),
                ret:Box::new(crate::types::ETVoid{}), running:true};
            c.variables.insert("ARGS".to_owned(), Box::new(ETList(input)));
            return c;
        }

        pub fn get_proc(&self, variable : bool, name : &'a str) -> Result<Option<Box<dyn ProcExecution>>, Error> {
            if variable {
                if let Some(r) = (self.get_var(name)?).function() {
                    return Ok(Some(r));
                }
            } else {
                for x in 0..self.instance.methods.len() {
                    if self.instance.methods[x].name() == name {
                        return Ok(Some(self.instance.methods[x].clone()));
                    }
                }
            }
            return Ok(None);
        }

        pub fn get_var(&self, name : &'a str) -> Result<Box<dyn Value>, Error> {
            if let Some(n) = self.variables.get(name) {
                return Ok(n.clone());
            }
            return Err(Error::new(ErrorKind::InvalidInput, "Error searching variable"));
        }
    }

    pub trait CloneValue {
        fn clone_box(&self) -> Box<dyn Value>;
    }

    pub trait Value : CloneValue {
        fn list(&self) -> Option<Box<ETList>> {
            None
        }
        fn map(&self) -> Option<Box<ETMap>> {
            None
        }
        fn int(&self) -> Option<Box<ETInt>> {
            None
        }
        fn float(&self) -> Option<Box<ETFloat>> {
            None
        }
        fn literal(&self) -> String;
        fn is_literal(&self) -> bool {
            false
        }
        fn function(&self) -> Option<Box<dyn ProcExecution>> {
            None
        }
    }

    impl<T> CloneValue for T where T : 'static + Value + Clone {
        fn clone_box(&self) -> Box<dyn Value> {
            return Box::new(self.clone());
        }
    }

    impl Clone for Box<dyn Value> {
        fn clone(&self) -> Box<dyn Value> {
            return self.clone_box();
        }
    }

    pub trait CloneProc {
        fn clone_box(&self) -> Box<dyn ProcExecution>;
    }

    pub trait ProcExecution : CloneProc {
        fn name(&self) -> String;
        fn run(&self, ins : &RunningInstance, input : Vec<Box<dyn Value>>, context : &mut Context) -> Result<Box<dyn Value>, Error>;
    }

    impl<T> CloneProc for T where T : 'static + ProcExecution + Clone {
        fn clone_box(&self) -> Box<dyn ProcExecution> {
            return Box::new(self.clone());
        }
    }

    impl Clone for Box<dyn ProcExecution> {
        fn clone(&self) -> Box<dyn ProcExecution> {
            return self.clone_box();
        }
    }
}

pub mod execution {
    use std::io::{Error, ErrorKind};
    use crate::core::{ProgramInstance, Block, Proc};
    use crate::core::runtime::{RunningInstance, ProcExecution, Context};
    use crate::stdprocs::get_standard_procs;
    use crate::types::ETLiteral;

    impl ProgramInstance {
        pub fn from(global : Vec<Block>) -> Result<Self, Error> {
            let mut name = String::new();
            let mut entry = String::new();
            let mut procs = Vec::<Proc>::new();
            for b in global.into_iter() {
                if name.is_empty() || entry.is_empty() {
                    let id = if name.is_empty() {"PROGRAM-ID"} else {"ENTER-IN"};
                    if b.head_is(&id) {
                        let (x, i, b) = b.cut_head();
                        if !b || i != 0  {
                            return Err(Error::new(ErrorKind::InvalidData, id.to_owned() + " must be followed just by one argument"));
                        }
                        if name.is_empty() {
                            name = x.data[0].clone();
                        } else {
                            entry = x.data[0].clone();
                        }
                    } else {
                        return Err(Error::new(ErrorKind::InvalidData, id.to_owned() + " expected"));
                    }
                } else {
                    if b.head_is("PROC") {
                        let (x, i, b) = b.cut_head();
                        if !b || i != 0  {
                            return Err(Error::new(ErrorKind::InvalidData, "PROC must be followed just by one argument"));
                        }
                        procs.push(Proc{name:x.data[0].clone(), mems:x.subs});
                    } else {
                        return Err(Error::new(ErrorKind::InvalidData, "PROC expected"));
                    }
                }
            }
            return Ok(ProgramInstance{name:name, entry_point:entry, methods:procs});
        }

        pub fn run(self) -> Result<i32, Error> {
            if let Some(x) = self.search_func(&self.entry_point) {
                let standard = get_standard_procs();
                let r = RunningInstance::from(self.clone(), standard);
                let args = ETLiteral::literal_array(&string_args(std::env::args()));
                return match x.run(&r, args.clone(), &mut Context::new(&r, args)) {
                    Ok(_) => Ok(0),
                    Err(e) => Err(e),
                }
            }
            return Err(Error::new(ErrorKind::NotFound, self.entry_point + " proc not found"));
        }

        fn search_func<'a>(&self, name : &'a str) -> Option<&Proc> {
            for x in self.methods.iter() {
                if x.name == name {
                    return Some(x);
                }
            }
            return None;
        }
    }

    fn string_args(args : std::env::Args) -> Vec<String> {
        let mut res = Vec::new();
        for a in args {
            res.push(a);
        }
        return res;
    }
}