use std::time::Instant;

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser, sema::SymbolTable};

mod interpreter;
mod lexer;
mod parser;
mod sema;
mod token;

fn main() -> anyhow::Result<()> {
    let total_time = Instant::now();
    let mut lexer = Lexer::new(std::fs::read_to_string("res/code.shit")?);

    let time = Instant::now();
    lexer.tokenize()?;
    println!(
        "Lexical analysis of {} tokens took {:?}",
        lexer.tokens().len(),
        time.elapsed()
    );

    dbg!(lexer.tokens());

    let mut parser = Parser::new(lexer.tokens().clone());
    let time = Instant::now();
    parser.parse()?;
    println!("Syntactic analysis took {:?}", time.elapsed());
    dbg!(&parser.global_scope);

    let mut sym_table = SymbolTable::new();
    let time = Instant::now();
    sym_table.build(&parser.global_scope)?;
    println!("Semantic analysis took {:?}", time.elapsed());
    dbg!(&sym_table);

    let mut interpreter = Interpreter::new();
    let time = Instant::now();
    interpreter.interpret(&parser.global_scope)?;
    println!("Interpretation took {:?}", time.elapsed());

    println!("Total time: {:?}", total_time.elapsed());
    Ok(())
}
