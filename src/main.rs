use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser, sema::SymbolTable};

mod interpreter;
mod lexer;
mod native_functions;
mod parser;
mod sema;
mod token;

#[cfg(test)]
mod tests;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        anyhow::bail!("Usage: {} <source_file>", args[0]);
    }

    run_file(&args[1])
}

fn run_file(name: &str) -> anyhow::Result<()> {
    let mut lexer = Lexer::new(std::fs::read_to_string(name)?);
    lexer
        .tokenize()
        .map_err(|e| anyhow::anyhow!("{e} at {:?}", lexer.location()))?;

    let mut parser = Parser::new(lexer.tokens().clone());
    parser.parse()?;

    let mut sym_table = SymbolTable::new();
    sym_table.build(&parser.global_scope)?;

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&parser.global_scope)
}
