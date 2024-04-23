use std::{env, fs, process::exit};

use lex::Token;
use logos::Logos;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod brainfuck;

mod ast;
mod lex;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Usage: ./brainfuck source_file.bf");
    exit(1);
  }

  // Lexing
  let source = fs::read_to_string(&args[1]).unwrap();
  let tokens = Token::lexer(&source)
    .spanned()
    .map(|(t, y)| (y.start, t.unwrap(), y.end));

  // Parsing
  let ops = brainfuck::OpsParser::new().parse(tokens).unwrap();

  println!("{:?}", ops);
}
