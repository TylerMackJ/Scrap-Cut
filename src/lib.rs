mod grid;
mod square;

#[cfg(test)]
mod tests {
    use crate::grid::*;
    use crate::square::*;

    #[test]
    fn taken() {
        assert_eq!(Square::Taken(0).is_taken(), true);
        assert_eq!(Square::Free.is_taken(), false);
    }

    #[test]
    fn option_taken() {
        assert_eq!(Some(Square::Taken(0)).is_taken(), true);
        assert_eq!(Some(Square::Free).is_taken(), false);
        assert_eq!(None.is_taken(), false)
    }

    #[test]
    fn partial_eq_ref_square() {
        assert_eq!(&Square::Free == Square::Free, true);
        assert_eq!(&Square::Free == Square::Scrap, false);
    }

    #[test]
    fn partial_eq_option_ref_square() {
        assert_eq!(Some(&Square::Free) == Square::Free, true);
        assert_eq!(Some(&Square::Scrap) == Square::Free, false);
    }

    #[test]
    fn grid_boundaries() {
        let grid: Grid<bool> = Grid::new(10, 10, false);
        assert_eq!(grid.get(0, 0) == Some(&false), true);
        assert_eq!(grid.get(0, 0) == Some(&true), false);
        assert_eq!(grid.get(9, 9) == Some(&false), true);
        assert_eq!(grid.get(9, 9) == Some&(true), false);
        assert_eq!(grid.get(10, 10) == None, true);
        assert_eq!(grid.get(10, 10) == Some(_), false);
    }
}