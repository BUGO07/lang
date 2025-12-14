use std::time::Instant;

use crate::{lexer::Lexer, syntax::Parser};

mod lexer;
mod syntax;
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
    dbg!(parser.global_scope);

    println!("Total time: {:?}", total_time.elapsed());
    Ok(())
}
