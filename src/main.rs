use clap::Parser as CParser;
use pest::Parser as PParser;
use pest::iterators::Pairs;
use std::path::PathBuf;
use pest_derive;
use std::fs;

#[derive(CParser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    file: PathBuf,
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct SyssyParser;

#[derive(Clone, PartialEq, Debug)]
struct Identifier(String);

#[derive(Clone, PartialEq, Debug)]
enum Mutability {
    Mut,
    Immut
}

#[derive(Clone, PartialEq, Debug)]
enum Substructure {
    Affine,
    Linear,
    Relevant
}

#[derive(Clone, PartialEq, Debug)]
struct Uses(Vec::<Identifier>);

#[derive(Clone, PartialEq, Debug)]
enum Ioi {
    Identifier(Identifier),
    Int(u128)
}

#[derive(Clone, PartialEq, Debug)]
struct Path(Vec::<Ioi>);

#[derive(Clone, PartialEq, Debug)]
struct Struct(Vec::<(Identifier, Value)>);

#[derive(Clone, PartialEq, Debug)]
struct Tuple(Vec::<Value>);

#[derive(Clone, PartialEq, Debug)]
struct Function {
    once: bool,
    arg: Identifier,
    body: Box<Value>
}

#[derive(Clone, PartialEq, Debug)]
struct Expression(Vec::<Expressions>);

#[derive(Clone, PartialEq, Debug)]
enum Expressions {
    Statement(Statement),
    Expression(Expression)
}

#[derive(Clone, PartialEq, Debug)]
enum Statement {
    Expression(Expression),
    Let(Let),
    Return(Return)
}

#[derive(Clone, PartialEq, Debug)]
struct Let(Identifier, Value);

#[derive(Clone, PartialEq, Debug)]
struct Return(Value);

#[derive(Clone, PartialEq, Debug)]
enum Values {
    Identifier(Identifier),
    Path(Path),
    Struct(Struct),
    Tuple(Tuple),
    Function(Function),
    Expression(Expression),
    String(String),
    Char(char),
    IdentifierValue(Identifier),
    Int(u128),
    Float(f64)
}

#[derive(Clone, PartialEq, Debug)]
struct Value {
    mutability: Mutability,
    substructure: Substructure,
    value: Values
}

impl Value {
    fn new(ast: &Pairs<Rule>) -> Self {
        Value {
            mutability: Mutability::Immut,
            substructure: Substructure::Linear,
            value: Values::Int(0)
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let source = fs::read_to_string(cli.file).unwrap();

    let  ast = Value::new(&SyssyParser::parse(Rule::Program, &source).unwrap());

    println!("{:?}", ast);
}
