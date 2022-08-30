mod grid;
mod square;
mod vec2;

#[cfg(test)]
mod tests {
    use crate::grid::*;
    use crate::square::*;
    use crate::vec2::*;

    #[test]
    fn taken() {
        assert!(Square::Taken(0).is_taken());
        assert!(!Square::Free.is_taken());
    }

    #[test]
    fn option_taken() {
        assert!(Some(&Square::Taken(0)).is_taken());
        assert!(!Some(&Square::Free).is_taken());
        assert!(!None.is_taken())
    }

    #[test]
    fn partial_eq_ref_square() {
        assert_eq!(&Square::Free, Square::Free);
        assert_eq!(&Square::Free, Square::Scrap);
    }

    #[test]
    fn partial_eq_option_ref_square() {
        assert_eq!(Some(&Square::Free), Square::Free);
        assert_ne!(Some(&Square::Scrap), Square::Free);
    }

    #[test]
    fn grid_boundaries() {
        let grid: Grid<bool> = Grid::new(10, 10, 2, false);
        assert_eq!(grid.get(0, 0), Some(&false));
        assert_ne!(grid.get(0, 0), Some(&true));

        assert_eq!(grid.get(4, 4), Some(&false));
        assert_ne!(grid.get(4, 4), Some(&true));

        assert_eq!(grid.get(5, 5), None);
        assert_ne!(grid.get(5, 5), Some(&true));
        assert_ne!(grid.get(5, 5), Some(&false));
    }

    #[test]
    fn sheet() {
        let grid: Grid<bool> = Grid::new(10, 10, 2, false);
        assert_eq!(grid.get(0.0, 0.0), Some(&false));
        assert_ne!(grid.get(0.0, 0.0), Some(&true));

        assert_eq!(grid.get(9.9, 9.9), Some(&false));
        assert_ne!(grid.get(9.9, 9.9), Some(&true));

        assert_eq!(grid.get(10.0, 10.0), None);
        assert_ne!(grid.get(10.0, 10.0), Some(&true));
        assert_ne!(grid.get(10.0, 10.0), Some(&false));
    }

    #[test]
    fn vec2_move_towards() {
        todo!();
    }

    #[test]
    fn vec2_curve_towards() {
        todo!();
    }
}