use crate::{ir::Instr as IRInstr, x86::AddType, x86::Instr};

pub fn codegen(irs: Vec<IRInstr>) -> Vec<Instr> {
  let mut res = vec![];
  for instr in irs {
    match instr {
      IRInstr::Label(s) => res.push(Instr::Label(s)),
      IRInstr::Input => res.append(&mut vec![Instr::Call("getchar".into()), Instr::Mov]),
      IRInstr::Output => res.append(&mut vec![Instr::Movzx, Instr::Call("putchar".into())]),
      IRInstr::Arith(n) => res.push(Instr::Add(AddType::Data, n)),
      IRInstr::Shift(n) => res.push(Instr::Add(AddType::Pointer, n)),
      IRInstr::JumpZero(_) | IRInstr::JumpNonZero(_) => {
        // If last one is add data, relevant flags are set and we do not
        // need to test again
        if !matches!(res.last(), Some(Instr::Add(AddType::Data, _))) {
          res.push(Instr::Movzx);
          res.push(Instr::Test);
        }
        match instr {
          IRInstr::JumpZero(s) => res.push(Instr::Jz(s)),
          IRInstr::JumpNonZero(s) => res.push(Instr::Jnz(s)),
          _ => unreachable!(),
        }
      }
    }
  }
  res
}
