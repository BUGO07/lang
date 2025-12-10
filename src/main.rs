use std::time::Instant;

use crate::lexer::Lexer;

mod lexer;
mod token;

const CODE: &str = r#"
6____7 5us.32
"#;

fn main() {
    let mut lexer = Lexer::new(CODE.to_string());

    let time = Instant::now();
    let res = lexer.tokenize();
    println!("Lexing took {:?}", time.elapsed());
    if let Err(err) = res {
        println!("Lexer error: {:?} at {:?}", err, lexer.location());
        return;
    }

    println!(
        "{:?}",
        lexer
            .tokens()
            .iter()
            .map(|x| &x.token_type)
            .collect::<Vec<_>>(),
    );
}
