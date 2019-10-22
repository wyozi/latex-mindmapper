mod parser;
mod linker;
mod dot;

use std::process;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let input = fs::read_to_string(&opt.file);
    if let Err(e) = input {
        eprintln!("Failed to read input file: {}", e);
        process::exit(1);
    }

    let source_nodes = parser::parse_string(&input.unwrap());
    if let Err(e) = linker::verify_links(&source_nodes) {
        eprintln!("Invalid links: {}", e);
        process::exit(1);
    }
    
    let out = dot::format_dot(&source_nodes);
    match opt.output {
        Some(f) => fs::write(f, &out).unwrap(),
        None => println!("{}", out)
    }
}
