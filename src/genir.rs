use crate::{ast::Op, ir::Instr};

pub fn genir(ops: Vec<Op>) -> Vec<Instr> {
  let mut cnt = 0;
  genir_priv(ops, &mut cnt)
}

fn genir_priv(ops: Vec<Op>, cnt: &mut i32) -> Vec<Instr> {
  let mut res = vec![];
  for op in ops {
    match op {
      Op::Arith(n) => res.push(Instr::Arith(n)),
      Op::Shift(n) => res.push(Instr::Shift(n)),
      Op::Input => res.push(Instr::Input),
      Op::Output => res.push(Instr::Output),
      Op::Loop(loop_body) => {
        let begin_label = format!("L_begin_{}", cnt);
        let end_label = format!("L_end_{}", cnt);
        *cnt += 1;
        res.push(Instr::JumpZero(end_label.clone()));
        res.push(Instr::Label(begin_label.clone()));
        res.append(&mut genir_priv(loop_body, cnt));
        res.push(Instr::JumpNonZero(begin_label));
        res.push(Instr::Label(end_label));
      }
    }
  }
  res
}
