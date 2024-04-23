#[derive(PartialEq, Eq, Debug)]
pub enum Op {
  Shift(i64),
  Arith(i64),
  Input,
  Output,
  Loop(Vec<Op>),
}
