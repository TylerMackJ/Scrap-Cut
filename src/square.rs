#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square { Free, Taken(usize), Scrap, Good }

pub trait IsTaken {
    fn is_taken(&self) -> bool;
}

impl IsTaken for Square {
    fn is_taken(&self) -> bool {
        matches!(self, Square::Taken(_))
    }
}

impl IsTaken for Option<&Square> {
    fn is_taken(&self) -> bool {
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
        match self {
            Some(s) => *s == other,
            None => false,
        }
    }
}