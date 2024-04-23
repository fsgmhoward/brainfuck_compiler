use logos::Logos;

// Skip all characters not belonging to Brainfuck's grammar
#[derive(Clone, Logos, Debug, PartialEq, Eq)]
#[logos(skip r"[^<>+\-.,\[\]]")]
pub enum Token {
  #[token("<", priority = 0)]
  Left,
  #[token(">", priority = 0)]
  Right,
  #[token("+", priority = 0)]
  Plus,
  #[token("-", priority = 0)]
  Minus,
  #[token(".", priority = 0)]
  Output,
  #[token(",", priority = 0)]
  Input,
  #[token("[", priority = 0)]
  LoopStart,
  #[token("]", priority = 0)]
  LoopEnd,
}
