#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldMark {
    X = 1,
    O = 2,
    Empt,
}
impl FieldMark {
    pub fn scores(&self) -> i32 {
        match self {
            FieldMark::X => 1,
            FieldMark::O => -1,
            FieldMark::Empt => 0,
        }
    }
    pub fn equals3(a: FieldMark, b: FieldMark, c: FieldMark) -> bool {
        a == b && b == c && a != FieldMark::Empt
    }
}
