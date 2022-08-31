pub mod grid;
pub mod square;
pub mod vec2;
pub mod cut;

#[cfg(test)]
mod tests {
    use crate::grid::*;
    use crate::square::*;
    use crate::vec2::*;
    use crate::cut::*;

    use std::f32::consts::PI;

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
        assert_ne!(&Square::Free, Square::Scrap);
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

    /*
    #[test]
    fn sheet_boundaries() {
        let grid: Grid<bool> = Grid::new(10, 10, 2, false);

        assert_eq!(grid.sheet_get(0.0, 0.0), Some(&false));
        assert_ne!(grid.sheet_get(0.0, 0.0), Some(&true));

        assert_eq!(grid.sheet_get(9.9, 9.9), Some(&false));
        assert_ne!(grid.sheet_get(9.9, 9.9), Some(&true));

        assert_eq!(grid.sheet_get(10.0, 10.0), None);
        assert_ne!(grid.sheet_get(10.0, 10.0), Some(&true));
        assert_ne!(grid.sheet_get(10.0, 10.0), Some(&false));
    }
    */

    #[test]
    fn grid_mut() {
        let mut grid: Grid<bool> = Grid::new(10, 10, 2, false);

        assert_eq!(grid.get(2, 2), Some(&false));
        assert_ne!(grid.get(2, 2), Some(&true));
        if let Some(mut_ref) = grid.get_mut(2, 2) {
            *mut_ref = true;
        }
        assert_eq!(grid.get(2, 2), Some(&true));
        assert_ne!(grid.get(2, 2), Some(&false));

        //assert_eq!(grid.sheet_get(4.9, 4.0), Some(&true));
        //assert_ne!(grid.sheet_get(4.9, 4.0), Some(&false));
        if let Some(mut_ref) = grid.sheet_get_mut(4.9, 4.0) {
            *mut_ref = false;
        }
        assert_eq!(grid.get(2, 2), Some(&false));
        assert_ne!(grid.get(2, 2), Some(&true));
    }

    #[test]
    fn vec2_move_towards() {
        let mut vector = Vec2 {
            x: 0.0,
            y: 0.0
        };

        let mut end_position = Vec2 {
            x: 2.0,
            y: 0.0,
        };

        vector.move_towards(end_position, 1.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 0.0);
        vector.move_towards(end_position, 1.0);
        assert_eq!(vector.x, 2.0);
        assert_eq!(vector.y, 0.0);

        end_position.x = 2.0;
        end_position.y = 2.0;

        vector.move_towards(end_position, 1.0);
        assert_eq!(vector.x, 2.0);
        assert_eq!(vector.y, 1.0);
        vector.move_towards(end_position, 1.0);
        assert_eq!(vector.x, 2.0);
        assert_eq!(vector.y, 2.0);
    }

    #[test]
    fn vec2_curve_towards() {
        let mut vector = Vec2 {
            x: 1.0,
            y: 2.0
        };

        let end_position = Vec2 {
            x: 1.0,
            y: 0.0,
        };

        let center_point = Vec2 {
            x: 1.0,
            y: 1.0,
        };

        vector.curve_towards(end_position, center_point, PI / 2.0, true);
        assert!(vector.x > 1.9 && vector.x < 2.1, "{:?}", vector);
        assert!(vector.y > 0.9 && vector.y < 1.1, "{:?}", vector) ;
        vector.curve_towards(end_position, center_point, PI / 2.0, false);
        assert!(vector.x > 0.9 && vector.x < 1.1, "{:?}", vector);
        assert!(vector.y > 1.9 && vector.y < 2.1, "{:?}", vector);
        vector.curve_towards(end_position, center_point, PI / 2.0, false);
        assert!(vector.x > -0.1 && vector.x < 0.1, "{:?}", vector);
        assert!(vector.y > 0.9 && vector.y < 1.1, "{:?}", vector);
        vector.curve_towards(end_position, center_point, PI / 2.0, false);
        assert!(vector.x > 0.9 && vector.x < 1.1, "{:?}", vector);
        assert!(vector.y > -0.1 && vector.y < 0.1, "{:?}", vector);
    }

    #[test]
    fn move_until_end() {
        let mut vector = Vec2 {
            x: 0.0,
            y: 0.0,
        };

        let linear_end = Vec2 {
            x: 1.0,
            y: 0.0,
        };

        let curve_end = Vec2 {
            x: -1.0,
            y: 0.0,
        };

        let center = Vec2 {
            x: 0.0,
            y: 0.0,
        };

        for _ in 0..10 {
            if vector == linear_end { break; }
            vector.move_towards(linear_end, 0.15);
        }
        assert_eq!(vector, linear_end);

        for _ in 0..25 {
            if vector == curve_end { break; }
            vector.curve_towards(curve_end, center, 0.15, false);
        }
        assert_eq!(vector, curve_end);

        for _ in 0..25 {
            if vector == linear_end { break; }
            vector.curve_towards(linear_end, center, 0.15, true);
        }
        assert_eq!(vector, linear_end);
    }

    #[test]
    fn distance() {
        let d = Vec2::distance(&Vec2 { x: 0.0, y: 0.0 }, &Vec2 { x: 1.0, y: 0.0 });
        assert!(d > 0.9 && d < 1.1, "{:?}", d);

        let d = Vec2::distance(&Vec2 { x: 2.0, y: 0.0 }, &Vec2 { x: 0.0, y: 0.0 });
        assert!(d > 1.9 && d < 2.1, "{:?}", d);

        let d = Vec2::distance(&Vec2 { x: 0.0, y: 0.0 }, &Vec2 { x: 1.0, y: 1.0 });
        assert!(d > 1.4 && d < 1.5, "{:?}", d);

        let d = Vec2::distance(&Vec2 { x: -1.0, y: -1.0 }, &Vec2 { x: 1.0, y: 1.0 });
        assert!(d > 1.9 && d < 2.1, "{:?}", d);
    }

    #[test]
    fn cut_capture() {
        let start = Vec2 {
            x: 1.23,
            y: 4.56,
        };

        let captured = LinearCut::capture(&start, "G00 X4.567 Y5.678 F100.000");
        let created = LinearCut::new(start, Vec2 { x: 4.567, y: 5.678 });

        assert_eq!(captured.start.x, created.start.x);
        assert_eq!(captured.start.y, created.start.y);
        assert_eq!(captured.end.x, created.end.x);
        assert_eq!(captured.end.y, created.end.y);

        let start = Vec2 {
            x: 1.23,
            y: 4.56,
        };

        let captured = CurveCut::capture(&start, "G03 X4.567 Y5.678 I1.234 J2.345 F100.000", false);
        let created = CurveCut::new(start, Vec2 { x: 4.567, y: 5.678 }, Vec2 { x: 1.234, y: 2.345 }, false);

        assert_eq!(captured.start.x, created.start.x);
        assert_eq!(captured.start.y, created.start.y);
        assert_eq!(captured.end.x, created.end.x);
        assert_eq!(captured.end.y, created.end.y);
        assert_eq!(captured.center.x, created.center.x);
        assert_eq!(captured.center.y, created.center.y);
        assert_eq!(captured.clockwise, created.clockwise);
    }
}