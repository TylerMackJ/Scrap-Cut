mod grid;
mod square;
mod vec2;

use crate::grid::*;
use crate::square::*;
use crate::vec2::*;

use std::f64::consts::PI;

struct Sheet {
    width: usize,
    height: usize,
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
    let mut grid: Grid<Square> = Grid::new(sheet.width / resolution, sheet.height / resolution, Square::Free, resolution);

    // Place shapes into grid
    let filename = "./gcode.gm";
    let file = File::open(filename)?;
    let file_buf = BufReader::new(file);
    // Loop over gcode
    let mut cutting = false;
    let mut head = Vec2 {
        x: 0,
        y: 0,
    };
    let mut current_shape = 0;
    for line in file_buf.lines() {
        // Check for enable cutting instruction
        if line.starts_with("M64") {
            cutting = true;
            grid.get_mut<f32>(head.x, head.y) = Square::Taken(current_shape);
        }
        // Check for linear movement instructions
        if line.starts_with("G00") || line.starts_with("G01") {
            // Capture X and Y
            let regex = Regex::new(r"X(\d+.\d+)\sY(\d+.\d+)").unwrap();
            let captures = regex.captures(line).unwrap();
            let end_pos = Vec2 {
                x: caps.get(1).map_or("Panic", |m| m.as_str().parse::<f32>().unwrap()),
                y: caps.get(2).map_or("Panic", |m| m.as_str().parse::<f32>().unwrap()),
            };

            if cutting {
                head.move_towards(end_pos, 0.5);
                grid.get_mut<f32>(head.x, head.y) = Square::Taken(current_shape);
            } else {
                // If we are not cutting then we can jump to final position
                head = end_pos;
            }
        } else if line.starts_with("G02") || line.starts_with("G03") { // Check for angular movement instructions
            // Capture X, Y, I, and J
            let regex = Regex::new(r"X(\d+.\d+)\sY(\d+.\d+)\sI(\d+.\d+)\sJ(\d+.\d+)").unwrap();
            let captures = regex.captures(line).unwrap();
            let end_pos = Vec2 {
                x: caps.get(1).map_or("Panic", |m| m.as_str().parse::<f32>().unwrap()),
                y: caps.get(2).map_or("Panic", |m| m.as_str().parse::<f32>().unwrap()),
            };

            if cutting {
                // Get the center point of the arc
                let center_point = Vec2 {
                    x: caps.get(3).map_or("Panic", |m| m.as_str().parse::<f32>().unwrap()),
                    y: caps.get(4).map_or("Panic", |m| m.as_str().parse::<f32>().unwrap()),
                };

                // G02 = clockwise, G03 = counterclockwise
                let clockwise = line.starts_with("G02");

                head.curve_towards(end_pos, center_point, 0.5, clockwise);
                grid.get_mut<f32>(head.x, head.y) = Square::Taken(current_shape);
            } else {
                // If we are not cutting then we can jump to final position
                head = end_pos;
                // This case should not occur, non cutting lines should be linear
                panic!("Non linear movement while not cutting found!");
            }
        }
        
        // Check for disable cutting instruction
        if line.starts_with("M65") {
            cutting = false;
            current_shape++;
        }
    }
    
    // Find all the Square::Scrap and Square::Good squares
    for x in 0..grid.width {
        for y in 0..grid.height {
            // All Square::Free change
            if grid.get<usize>(x, y) == Square::Free {
                // Square::Good if 2 orthogonal squares are not Square::Taken else Square::Scrap
                if (!grid.get<usize>(x, y).is_taken() || !grid.get<usize>(x - 1, y).is_taken()) && (!grid.get<usize>(x, y + 1).is_taken() || !grid.get(x, y - 1).is_taken()) {
                    match grid.get_mut<usize>(x, y) {
                        Some(s) => *s = Square::Good,
                        None => panic!(),
                    }
                }
                else {
                    match grid.get_mut<usize>(x, y) {
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
            if grid.get<usize>(x, y) == Square::Scrap && (grid.get<usize>(x + 1, y) == Square::Good || grid.get<usize>(x - 1, y) == Square::Good || grid.get(x, y + 1) == Square::Good || grid.get(x, y - 1) == Square::Good) {
                // Find each shape that has a Square::Taken touching the current square
                    // Make cut from middle of current square to the closest point of each shape
            }

            // Find Taken-Good-Wall Cuts
            // Check top and bottom
            if x == 0 || x == grid.width - 1 {
                if grid.get<usize>(x, y).is_taken() && grid.get<usize>(x, y + 1) == Square::Good {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }

            // Check left and right
            if y == 0 || y == grid.height - 1 {
                if grid.get<usize>(x, y).is_taken() && grid.get<usize>(x + 1, y) == Square::Good {
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
                    match grid.get<usize>(x + i, y + j) {
                        Some(&Square::Taken(_)) => {
                            if x_taken.is_none() {
                                x_taken = grid.get<usize>(x + i, y + j);
                            }
                            else {
                                y_taken = grid.get<usize>(x + i, y + j);
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