extern crate clap;

use std::process::exit;
use std::fs::File;
use clap::{Arg, App};
use std::io::Error;

mod stdprocs;
mod types;
mod core;

fn main() {
    let matches = App::new("Efecta Interpreter").version("0.1").author("Alberto Elorza")
        .about("Efecta is a simple programming language oriented to communication between
processes, influenced by COBOl syntax, with stack based design and functional programming attributes")
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .help("Efecta source file (.esf)")
            .takes_value(true)
        ).get_matches();
    match matches.value_of("file") {
        Some(n) => match run_program(n) {
            Ok(x) => exit(x),
            Err(r) => eprintln!("Error: {}", r)
        },
        None => {
            eprintln!("ERROR!: No input file");
            exit(1);
        }
    };
}

/*fn iterblock(f : core::Block, lev : i32) {
    for _ in 0..lev {
        print!("()");
    }
    for v in f.data.into_iter() {
        print!("{} ", v);
    }
    println!("");
    for x in f.subs.into_iter() {
        iterblock(x, lev+1);
    }
}*/

fn run_program(src_file : &str) -> Result<i32, Error>{
    //1st LEXER
    //2nd GENERATE BLOCK STRUCTURE
    //3rd EXECUTE THE MAIN BLOCK
    let source = File::open(src_file)?;
    let tokens = core::lexer::get_tokens(Box::from(source))?;
    let blocks = core::structure::generate_blocks(tokens)?;
    /*for t in blocks.clone().into_iter() {
        iterblock(t, 0);
    }*/
    use crate::core::ProgramInstance;
    let instance = ProgramInstance::from(blocks)?;
    //println!("COMPILED DATA:\n{}\n{}", instance.name, instance.entry_point);
    return instance.run();
}