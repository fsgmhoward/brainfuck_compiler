use std::fmt::Display;

// Because it is highly simplified, we do not even need to define the
// set of registers we have
/// Using a callee-saved register for data pointer
pub const DATA_PTR: &str = "%r15";

pub enum AddType {
  Pointer,
  Data,
}

/// Very simplified X86-64 ISA
/// Since part of the instruction is mostly fixed, we will hard code
/// them in Display implementation
pub enum Instr {
  /// Move with zero extension
  /// Used to move data from the slot to %rdi for putchar()
  Movzx,
  /// Regular move
  /// Used to move data from %ax to the slot after getchar()
  Mov,
  /// Arithmetic
  /// Used to move data pointer or change slot value
  Add(AddType, i64),
  /// Test Zero
  /// Test data in RDI, so need to move byte to RDI first
  Test,
  /// Jump Zero
  Jz(String),
  /// Jump Not Zero
  Jnz(String),
  /// Label
  Label(String),
  /// External Call
  Call(String),
}

impl Display for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instr::Movzx => write!(f, "movzx ({}), %rdi", DATA_PTR),
      Instr::Mov => write!(f, "movb %al, ({})", DATA_PTR),
      Instr::Add(AddType::Pointer, n) => write!(f, "addq ${}, {}", n, DATA_PTR),
      Instr::Add(AddType::Data, n) => write!(f, "addb ${}, ({})", n, DATA_PTR),
      Instr::Test => write!(f, "testq %rdi, %rdi"),
      Instr::Jz(label) => write!(f, "jz .{}", label),
      Instr::Jnz(label) => write!(f, "jnz .{}", label),
      Instr::Label(label) => write!(f, ".{}:", label),
      Instr::Call(func) => write!(f, "call {}", func),
    }
  }
}
