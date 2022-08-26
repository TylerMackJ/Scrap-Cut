use std::ops::{Index, IndexMut};
use grid::*;
use square::*

struct Sheet {
    width: usize,
    height: usize,
}

struct Vec2 {
    x: f32,
    y: f32,
}

struct Line2D {
    start: Vec2,
    end: Vec2,
}

fn main() {
    let sheet = Sheet {
        width: 48,
        height: 96
    };

    // Create Grid
    let resolution: usize = 2;
    let mut grid: Grid<Square> = Grid::new(sheet.width / resolution, sheet.height / resolution, Square::Free);

    /*
    // Place shapes into grid
    // Loop over gcode
        let mut cutting = 0;
        // if m64
            cutting = 1;
        // if cutting && (G00 || G01)
            // Step over line by 1/2 resolution steps
                // Make stepped over squares Square::Taken(shape)
        // if cutting && G02
            // Step over arc by 1/2 resolution steps
                // Make stepped over squares Square::Taken(shape)
        // if m65
            cuting = 0;
    */
    
    // Find all the Square::Scrap and Square::Good squares
    for x in 0..grid.width {
        for y in 0..grid.height {
            // All Square::Free change
            if grid.get(x, y) == Square::Free {
                // Square::Good if 2 orthogonal squares are not Square::Taken else Square::Scrap
                if (!grid.get(x, y).is_taken() || !grid.get(x - 1, y).is_taken()) && (!grid.get(x, y + 1).is_taken() || !grid.get(x, y - 1).is_taken()) {
                    match grid.get_mut(x, y) {
                        Some(s) => *s = Square::Good,
                        None => panic!(),
                    }
                }
                else {
                    match grid.get_mut(x, y) {
                        Some(s) => *s = Square::Scrap,
                        None => panic!(),
                    }
                }
            }
        }
    }

    // Find all the cuts
    let mut cuts: Vec<Line2D> = Vec::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            // If Square::Scrap is next to Square::Good and all by itself
                // Change to Square::Good

            // Find Square::Good-Square::Scrap cuts
            if grid.get(x, y) == Square::Scrap && (grid.get(x + 1, y) == Square::Good || grid.get(x - 1, y) == Square::Good || grid.get(x, y + 1) == Square::Good || grid.get(x, y - 1) == Square::Good) {
                // Find each shape that has a Square::Taken touching the current square
                    // Make cut from middle of current square to the closest point of each shape
            }

            // Find Taken-Good-Wall Cuts
            // Check top and bottom
            if x == 0 || x == grid.width - 1 {
                if grid.get(x, y).is_taken() && grid.get(x, y + 1) == Square::Good {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }

            // Check left and right
            if y == 0 || y == grid.height - 1 {
                if grid.get(x, y).is_taken() && grid.get(x + 1, y) == Square::Good {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }

            // Find xTaken-yTaken-Good cuts
            // Find 2x2 groups of xTaken-yTaken-Good-Good squares
            let mut good_count = 0;
            let mut x_taken: Option<&Square> = None;
            let mut y_taken: Option<&Square> = None;
            for i in 0..2 {
                for j in 0..2 {
                    match grid.get(x + i, y + j) {
                        Some(&Square::Taken(_)) => {
                            if x_taken.is_none() {
                                x_taken = grid.get(x + i, y + j);
                            }
                            else {
                                y_taken = grid.get(x + i, y + j);
                            }
                        },
                        Some(&Square::Good) => good_count += 1,
                        _ => {}
                    }
                }
            }
            if good_count == 2 && !x_taken.is_none() && !y_taken.is_none() {
                // Cut the thinnest point between xTaken and yTaken
            }
        }
    }

    // Add the cuts to gcode
    for cut in cuts
    {
        
    }
}