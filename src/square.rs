#[derive(Copy, Clone, PartialEq)]
pub enum Square { Free, Taken(u8), Scrap, Good }

trait IsTaken {
    pub fn is_taken(&self) -> bool;
}

impl IsTaken for Square {
    pub fn is_taken(&self) -> bool {
        matches!(self, Square::Taken(_))
    }
}

impl IsTaken for Option<&Square> {
    pub fn is_taken(&self) -> bool {
        matches!(self, Some(Square::Taken(_)))
    }
}


impl PartialEq<Square> for &Square {
    fn eq(&self, other: &Square) -> bool {
        *self == other
    }
}

impl PartialEq<Square> for Option<&Square> {
    fn eq(&self, other: &Square) -> bool {
        return match self {
            Some(s) => *s == other,
            None => false,
        }
    }
}