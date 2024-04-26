use std::io::{BufWriter, Write};
use std::process::Command;
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

  // Emit
  let asm_file = format!("{}.s", &args[1]);
  let f = fs::File::create(&asm_file).unwrap();
  let mut writer = BufWriter::new(f);
  writeln!(writer, ".globl main").unwrap();
  writeln!(writer, "main:").unwrap();
  // Prologue and stack alignment
  writeln!(writer, "  pushq %rbp").unwrap();
  writeln!(writer, "  movq %rsp, %rbp").unwrap();
  writeln!(writer, "  andq $-16, %rbp").unwrap();
  // Allocate data for number slots
  // calloc(30000, 1);
  writeln!(writer, "  movq $30000, %rdi").unwrap();
  writeln!(writer, "  movq $1, %rsi").unwrap();
  writeln!(writer, "  call calloc").unwrap();
  writeln!(writer, "  movq %rax, {}", DATA_PTR).unwrap();
  // Also save it in %r14
  writeln!(writer, "  movq %rax, %r14").unwrap();

  for instr in asm {
    if matches!(instr, crate::x86::Instr::Label(_)) {
      writeln!(writer, "{}", instr).unwrap();
    } else {
      writeln!(writer, "  {}", instr).unwrap();
    }
  }

  // Epilogue - leave and return 0
  writeln!(writer, "  movq %r14, %rdi").unwrap();
  writeln!(writer, "  call free").unwrap();
  writeln!(writer, "  movq $0, %rax").unwrap();
  writeln!(writer, "  leave").unwrap();
  writeln!(writer, "  ret").unwrap();

  writer.flush().unwrap();

  // Assemble and link, with cc
  let bin_file = format!("{}.out", &args[1]);
  Command::new("cc")
    .args(vec!["-o", &bin_file, &asm_file])
    .status()
    .unwrap();

  // Debug message
  println!("Output assembly: {}", &asm_file);
  println!("Output binary: {}", &bin_file);
}
