mod parser;
mod linker;
mod dot;

use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: latex-mindmapper <filename.tex>");
        process::exit(1);
    }

    let input = fs::read_to_string(&args[1]);
    if let Err(e) = input {
        eprintln!("Failed to read input file: {}", e);
        process::exit(1);
    }

    let source_nodes = parser::parse_string(&input.unwrap());
    if let Err(e) = linker::verify_links(&source_nodes) {
        eprintln!("Invalid links: {}", e);
        process::exit(1);
    }
    dot::print_dot(&source_nodes);
}
