use std::path::PathBuf;
use clap::Parser as CParser;
use pest::Parser as PParser;
use pest_derive;
use std::fs;

#[derive(CParser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    file: PathBuf,
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct TyperustParser;

fn main() {
    let cli = Cli::parse();

    let source = fs::read_to_string(cli.file).unwrap();

    let  ast = TyperustParser::parse(Rule::Program, &source).unwrap();

    println!("{}", ast);
}
