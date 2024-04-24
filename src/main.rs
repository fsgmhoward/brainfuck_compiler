use std::{env, fs, process::exit};

use lex::Token;
use logos::Logos;

use crate::codegen::codegen;
use crate::genir::genir;
use crate::x86::DATA_PTR;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod brainfuck;

mod ast;
mod codegen;
mod genir;
mod ir;
mod lex;
mod x86;

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

  // IR Generation
  let irs = genir(ops);

  // X86 Codegen
  let asm = codegen(irs);

  println!(".globl main");
  println!("main:");
  // Prologue and stack alignment
  println!("  pushq %rbp");
  println!("  movq %rsp, %rbp");
  println!("  andq $-16, %rbp");
  // Allocate data for number slots
  // calloc(30000, 1);
  println!("  movq $30000, %rdi");
  println!("  movq $1, %rsi");
  println!("  call calloc");
  println!("  movq %rax, {}", DATA_PTR);
  // Also save it in %r14
  println!("  movq %rax, %r14");

  for instr in asm {
    if matches!(instr, crate::x86::Instr::Label(_)) {
      println!("{}", instr);
    } else {
      println!("  {}", instr);
    }
  }

  // Epilogue - leave and return 0
  println!("  movq %r14, %rdi");
  println!("  call free");
  println!("  movq $0, %rax");
  println!("  leave");
  println!("  ret");
}
