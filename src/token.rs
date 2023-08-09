#[derive(PartialEq)]
pub enum Token {
    Integer(i64),
    Real(f64),
    Single(u8),
    Double(u8, u8),
}

pub type Pos = core::ops::Range<usize>;

pub struct TokenAndPos {
    pub token: Token,
    pub pos: Pos,
}
