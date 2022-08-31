mod grid;
mod square;
mod vec2;
mod cut;

use crate::grid::*;
use crate::square::*;
use crate::vec2::*;
use crate::cut::*;

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn main() {
    // Create Grid
    let mut grid: Grid<Square> = Grid::new(48, 96, 2, Square::Free);
    let mut shape_lines: Vec<Vec<String>> = Vec::new();

    // Place shapes into grid
    let filename = "./gcode.gm";
    let file = File::open(filename).unwrap();
    let file_buf = BufReader::new(file);
    // Loop over gcode
    let mut cutting = false;
    let mut head = Vec2 {
        x: 0.0,
        y: 0.0,
    };
    let mut current_shape: usize = 0;
    let linear_regex = Regex::new(r"X(\d+.\d+)\sY(\d+.\d+)").unwrap();
    let curve_regex = Regex::new(r"X(\d+.\d+)\sY(\d+.\d+)\sI(\d+.\d+)\sJ(\d+.\d+)").unwrap();
    for line in file_buf.lines().map(|line| line.unwrap()) {
        // Check for enable cutting instruction
        if line.starts_with("M64") {
            cutting = true;
            shape_lines.push(Vec::new());
            if let Some(mut_ref) = grid.sheet_get_mut(head.x, head.y) {
                *mut_ref = Square::Taken(current_shape);
            }
        }
        // Check for linear movement instructions
        if line.starts_with("G00") || line.starts_with("G01") {
            // Capture X and Y
            let captures = linear_regex.captures(&line).unwrap();
            let end_pos = Vec2 {
                x: captures.get(1).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
                y: captures.get(2).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
            };

            if cutting {
                while head != end_pos {
                    head.move_towards(end_pos, 0.5);
                    if let Some(mut_ref) = grid.sheet_get_mut(head.x, head.y) {
                        *mut_ref = Square::Taken(current_shape);
                    }
                }
            } else {
                // If we are not cutting then we can jump to final position
                head = end_pos;
            }
        } else if line.starts_with("G02") || line.starts_with("G03") { // Check for angular movement instructions
            // Capture X, Y, I, and J
            let captures = curve_regex.captures(&line).unwrap();
            let end_pos = Vec2 {
                x: captures.get(1).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
                y: captures.get(2).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
            };

            if cutting {
                // Get the center point of the arc
                let center_point = Vec2 {
                    x: captures.get(3).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
                    y: captures.get(4).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
                };

                // G02 = clockwise, G03 = counterclockwise
                let clockwise = line.starts_with("G02");

                while head != end_pos {
                    head.curve_towards(end_pos, center_point, 0.5, clockwise);
                    if let Some(mut_ref) = grid.sheet_get_mut(head.x, head.y) {
                        *mut_ref = Square::Taken(current_shape);
                    }
                }
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
            current_shape += 1;
        }

        if cutting {
            shape_lines.get_mut(current_shape).unwrap().push(line);
        }
    }
    
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
            // If Square::Scrap is next to Square::Good and all by itself change to Square::Good
            if grid.get(x, y) == Square::Scrap {
                let mut found_good = false;
                let mut found_scrap = false;
                if grid.get(x + 1, y) == Square::Good || grid.get(x - 1, y) == Square::Good || grid.get(x, y + 1) == Square::Good || grid.get(x, y - 1) == Square::Good {
                    found_good = true;
                }
                if grid.get(x + 1, y) == Square::Scrap || grid.get(x - 1, y) == Square::Scrap || grid.get(x, y + 1) == Square::Scrap || grid.get(x, y - 1) == Square::Scrap {
                    found_scrap = true;
                }
                if found_good && !found_scrap {
                    if let Some(mut_ref) = grid.get_mut(x, y) {
                        *mut_ref = Square::Good;
                    }
                }
            }

            // Find Square::Good-Square::Scrap cuts
            if grid.get(x, y) == Square::Scrap && (grid.get(x + 1, y) == Square::Good || grid.get(x - 1, y) == Square::Good || grid.get(x, y + 1) == Square::Good || grid.get(x, y - 1) == Square::Good) {
                // Find each shape that has a Square::Taken touching the current square
                let mut taken_shapes = Vec::new();
                if let Some(Square::Taken(s)) = grid.get(x + 1, y) {
                    taken_shapes.push(s)
                }
                if let Some(Square::Taken(s)) = grid.get(x - 1, y) {
                    taken_shapes.push(s)
                }
                if let Some(Square::Taken(s)) = grid.get(x, y + 1) {
                    taken_shapes.push(s)
                }
                if let Some(Square::Taken(s)) = grid.get(x, y - 1) {
                    taken_shapes.push(s)
                }
                
                // Make cut from middle of current square to the closest point of each shape
                for shape in taken_shapes {
                    todo!();
                }
            }

            // Find Taken-Good-Wall Cuts
            // Check top and bottom
            if x == 0 || x == grid.width - 1 {
                if grid.get(x, y).is_taken() && grid.get(x, y + 1) == Square::Good {
                    // Cut where the shape belonging to the current square is closest to the wall
                    todo!();
                }
            }

            // Check left and right
            if y == 0 || y == grid.height - 1 {
                if grid.get(x, y).is_taken() && grid.get(x + 1, y) == Square::Good {
                    // Cut where the shape belonging to the current square is closest to the wall
                    todo!();
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
                todo!();
            }
        }
    }

    // Add the cuts to gcode
    for cut in cuts
    {
        todo!();
    }
}