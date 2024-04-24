#[derive(Debug, Clone)]
pub enum Instr {
  Shift(i64),
  Arith(i64),
  Input,
  Output,
  // For loops:
  // JZ END_LABEL
  // BEGIN_LABEL:
  // ...
  // JNZ BEGIN_LABEL
  // END_LABEL:
  JumpZero(String),
  JumpNonZero(String),
  Label(String),
}
