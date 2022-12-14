mod grid;
mod square;
mod vec2;
mod cut;

use crate::grid::*;
use crate::square::*;
use crate::vec2::*;
use crate::cut::*;

use std::fs::{self, File};
use std::io::{BufReader, BufRead};

fn main() {
    // Create Grid
    let mut grid: Grid<Square> = Grid::new(48, 96, 2, Square::Free);
    let filename = "./gcode.gm";

    let shape_cuts = find_taken(&mut grid, filename);
    
    find_good_and_scrap(&mut grid);

    let cuts = find_cuts(&grid, shape_cuts);

    // Add the cuts to gcode
    // Open GCode
    let file = File::open(filename).unwrap();
    let mut lines = BufReader::new(file).lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    while !lines.pop().unwrap().starts_with("G00") {}
    for cut in cuts
    {
        lines.push(format!("G00 X{:.3} Y{:.3}", cut.start.x, cut.start.y));
        lines.push("M64".to_string());
        lines.push(format!("G01 X{:.3} Y{:.3} F100.00", cut.end.x, cut.end.y));
        lines.push("M65".to_string());
    }
    lines.push("G00 X0.000 Y0.000".to_string());
    fs::write(filename, lines.join("\n")).unwrap();
}

fn find_taken(grid: &mut Grid<Square>, filename: &str) -> Vec<Vec<Cut>> {
    // Open GCode
    let file = File::open(filename).unwrap();
    let file_buf = BufReader::new(file);

    // Track head status
    let mut cutting = false;
    let mut head = Vec2 {
        x: 0.0,
        y: 0.0,
    };

    // Track the current shape and related cuts
    let mut current_shape: usize = 0;
    let mut shape_cuts: Vec<Vec<Cut>> = Vec::new();

    // Place shapes into grid
    for line in file_buf.lines().map(|line| line.unwrap()) {
        // Check for enable cutting instruction
        if line.starts_with("M64") {
            cutting = true;
            shape_cuts.push(Vec::new());
            if let Some(mut_ref) = grid.sheet_get_mut(head.x, head.y) {
                *mut_ref = Square::Taken(current_shape);
            }
        }
        // Check for linear movement instructions
        if line.starts_with("G00") || line.starts_with("G01") {
            // Capture cut
            let cut: LinearCut = LinearCut::capture(&head, &line[..]);

            if cutting {
                // Cut until we reach the end
                while head != cut.end {
                    head.move_towards(cut.end, 0.5);
                    if let Some(mut_ref) = grid.sheet_get_mut(head.x, head.y) {
                        *mut_ref = Square::Taken(current_shape);
                    }
                }

                // Save cut for later
                shape_cuts.get_mut(current_shape).unwrap().push(Cut::Linear(cut));
            } else {
                // If we are not cutting then we can jump to final position
                head = cut.end;
            }
        } else if line.starts_with("G02") || line.starts_with("G03") { // Check for angular movement instructions
            // Capture cut
            let cut: CurveCut = CurveCut::capture(&head, &line[..], line.starts_with("G02"));

            if cutting {
                // Move along arc and cut
                while head != cut.end {
                    head.curve_towards(cut.end, cut.center, 0.5, cut.clockwise);
                    if let Some(mut_ref) = grid.sheet_get_mut(head.x, head.y) {
                        *mut_ref = Square::Taken(current_shape);
                    }
                }

                // Save cut for later
                shape_cuts.get_mut(current_shape).unwrap().push(Cut::Curve(cut));
            } else {
                // If we are not cutting then we can jump to final position
                //head = cut.end;
                // This case should not occur, non cutting lines should be linear
                panic!("Non linear movement while not cutting found!");
            }
        }
        
        // Check for disable cutting instruction
        if line.starts_with("M65") {
            cutting = false;
            current_shape += 1;
        }
    }
    shape_cuts
}

fn find_good_and_scrap(grid: &mut Grid<Square>) {
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

    // If Square::Scrap is next to Square::Good and all by itself change to Square::Good
    for x in 0..grid.width {
        for y in 0..grid.height {
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
        }
    }
}

fn find_cuts(grid: &Grid<Square>, shape_cuts: Vec<Vec<Cut>>) -> Vec<LinearCut> {
    // Find all the cuts
    let mut cuts: Vec<LinearCut> = Vec::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
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
                    // Get the middle of current square
                    let start = Vec2 {
                        x: (x * grid.resolution) as f32 + 0.5,
                        y: (y * grid.resolution) as f32 + 0.5,
                    };

                    let mut closest_end = Vec2 {
                        x: 0.0,
                        y: 0.0,
                    };

                    // Loop over all cuts related the shape
                    for cut_type in shape_cuts.get(*shape).unwrap() {
                        // Determine type of cut
                        match cut_type {
                            Cut::Linear(cut) => {
                                // Start at the beginning of the cut
                                let mut current_pos = Vec2 {
                                    x: cut.start.x,
                                    y: cut.start.y,
                                };

                                // Step through cut
                                while current_pos != cut.end {
                                    // If current position makes smaller cut save it
                                    if Vec2::distance(&start, &current_pos) < Vec2::distance(&start, &closest_end) {
                                        closest_end.x = current_pos.x;
                                        closest_end.y = current_pos.y;
                                    }
                            
                                    // Continue cut
                                    current_pos.move_towards(cut.end, 0.1);
                                }
                            },
                            Cut::Curve(cut) => {
                                // Same as above but curved
                                let mut current_pos = Vec2 {
                                    x: cut.start.x,
                                    y: cut.start.y,
                                };

                                while current_pos != cut.end {
                                    if Vec2::distance(&start, &current_pos) < Vec2::distance(&start, &closest_end) {
                                        closest_end.x = current_pos.x;
                                        closest_end.y = current_pos.y;
                                    }

                                    current_pos.curve_towards(cut.end, cut.center, 0.1, cut.clockwise)
                                }
                            }
                        }
                    }
                    // Save the smallest found cut
                    cuts.push(LinearCut::new(start, closest_end));
                }
            }

            // Find Taken-Good-Wall Cuts
            // Check top and bottom
            if (x == 0 || x == grid.width - 1) && (grid.get(x, y).is_taken() && grid.get(x, y + 1) == Square::Good) {
                // Cut where the shape belonging to the current square is closest to the wall
                if let Some(Square::Taken(s)) = grid.get(x, y) {    
                    let mut closest_point = Vec2 {
                        x: 0.0,
                        y: 0.0,
                    };
                    // Find out if cut should be left or right
                    let x_end = if x == 0 { 0.0 } else { (grid.width * grid.resolution) as f32 };
                    
                    // Loop over all cuts related the shape
                    for cut_type in shape_cuts.get(*s).unwrap() {
                        // Determine type of cut
                        match cut_type {
                            Cut::Linear(cut) => {
                                // Start at the beginning of the cut
                                let mut current_pos = Vec2 {
                                    x: cut.start.x,
                                    y: cut.start.y,
                                };

                                // Step through cut
                                while current_pos != cut.end {
                                    // If current position makes smaller cut save it
                                    if Vec2::distance(&Vec2 { x: x_end, y: current_pos.y }, &current_pos) < Vec2::distance(&Vec2 { x: x_end, y: closest_point.y }, &closest_point) {
                                        closest_point.x = current_pos.x;
                                        closest_point.y = current_pos.y;
                                    }
                            
                                    // Continue cut
                                    current_pos.move_towards(cut.end, 0.1);
                                }
                            },
                            Cut::Curve(cut) => {
                                // Same as above but curved
                                let mut current_pos = Vec2 {
                                    x: cut.start.x,
                                    y: cut.start.y,
                                };

                                while current_pos != cut.end {
                                    if Vec2::distance(&Vec2 { x: x_end, y: current_pos.y }, &current_pos) < Vec2::distance(&Vec2 { x: x_end, y: closest_point.y }, &closest_point) {
                                        closest_point.x = current_pos.x;
                                        closest_point.y = current_pos.y;
                                    }

                                    current_pos.curve_towards(cut.end, cut.center, 0.1, cut.clockwise)
                                }
                            }
                        }
                    }
                    // Save the smallest found cut
                    cuts.push(LinearCut::new(closest_point, Vec2 { x: x_end, y: closest_point.y }));
                }
            }

            // Check left and right
            if (y == 0 || y == grid.height - 1) && (grid.get(x, y).is_taken() && grid.get(x + 1, y) == Square::Good) {
                // Cut where the shape belonging to the current square is closest to the wall
                if let Some(Square::Taken(s)) = grid.get(x, y) {    
                    let mut closest_point = Vec2 {
                        x: 0.0,
                        y: 0.0,
                    };
                    // Find out if cut should be up or down
                    let y_end = if y == 0 { 0.0 } else { (grid.height * grid.resolution) as f32 };
                    
                    // Loop over all cuts related the shape
                    for cut_type in shape_cuts.get(*s).unwrap() {
                        // Determine type of cut
                        match cut_type {
                            Cut::Linear(cut) => {
                                // Start at the beginning of the cut
                                let mut current_pos = Vec2 {
                                    x: cut.start.x,
                                    y: cut.start.y,
                                };

                                // Step through cut
                                while current_pos != cut.end {
                                    // If current position makes smaller cut save it
                                    if Vec2::distance(&Vec2 { x: current_pos.x, y: y_end }, &current_pos) < Vec2::distance(&Vec2 { x: closest_point.x, y: y_end }, &closest_point) {
                                        closest_point.x = current_pos.x;
                                        closest_point.y = current_pos.y;
                                    }
                            
                                    // Continue cut
                                    current_pos.move_towards(cut.end, 0.1);
                                }
                            },
                            Cut::Curve(cut) => {
                                // Same as above but curved
                                let mut current_pos = Vec2 {
                                    x: cut.start.x,
                                    y: cut.start.y,
                                };

                                while current_pos != cut.end {
                                    if Vec2::distance(&Vec2 { x: current_pos.x, y: y_end }, &current_pos) < Vec2::distance(&Vec2 { x: closest_point.x, y: y_end }, &closest_point) {
                                        closest_point.x = current_pos.x;
                                        closest_point.y = current_pos.y;
                                    }

                                    current_pos.curve_towards(cut.end, cut.center, 0.1, cut.clockwise)
                                }
                            }
                        }
                    }
                    // Save the smallest found cut
                    cuts.push(LinearCut::new(closest_point, Vec2 { x: closest_point.x, y: y_end }));
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
            if good_count == 2 {
                if let Some(Square::Taken(x_s)) = x_taken {
                    if let Some(Square::Taken(y_s)) = y_taken {
                        let mut x_point = Vec2 {
                            x: 0.0,
                            y: 0.0,
                        };
                        let mut y_point = Vec2 {
                            x: 0.0,
                            y: 0.0,
                        };

                        for x_cut_type in shape_cuts.get(*x_s).unwrap() {
                            // Determine type of cut
                            match x_cut_type {
                                Cut::Linear(x_cut) => {
                                    // Start at the beginning of the cut
                                    let mut x_current_pos = Vec2 {
                                        x: x_cut.start.x,
                                        y: x_cut.start.y,
                                    };

                                    // Step through cut
                                    while x_current_pos != x_cut.end {
                                        // Loop over all cuts related the shape
                                        for y_cut_type in shape_cuts.get(*y_s).unwrap() {
                                            // Determine type of cut
                                            match y_cut_type {
                                                Cut::Linear(y_cut) => {
                                                    // Start at the beginning of the cut
                                                    let mut y_current_pos = Vec2 {
                                                        x: y_cut.start.x,
                                                        y: y_cut.start.y,
                                                    };

                                                    // Step through cut
                                                    while y_current_pos != y_cut.end {
                                                        // If current position makes smaller cut save it
                                                        if Vec2::distance(&Vec2 { x: x_current_pos.x, y: x_current_pos.y }, &Vec2 { x: y_current_pos.x, y: y_current_pos.y }) < Vec2::distance(&Vec2 { x: x_point.x, y: x_point.y }, &Vec2 { x: y_point.x, y: y_point.y }) {
                                                            x_point.x = x_current_pos.x;
                                                            x_point.y = x_current_pos.y;
                                                            y_point.x = y_current_pos.x;
                                                            y_point.y = y_current_pos.y;
                                                        }
                                                
                                                        // Continue cut
                                                        y_current_pos.move_towards(y_cut.end, 0.1);
                                                    }
                                                },
                                                Cut::Curve(y_cut) => {
                                                    // Same as above but curved
                                                    let mut y_current_pos = Vec2 {
                                                        x: y_cut.start.x,
                                                        y: y_cut.start.y,
                                                    };

                                                    // Step through cut
                                                    while y_current_pos != y_cut.end {
                                                        // If current position makes smaller cut save it
                                                        if Vec2::distance(&Vec2 { x: x_current_pos.x, y: x_current_pos.y }, &Vec2 { x: y_current_pos.x, y: y_current_pos.y }) < Vec2::distance(&Vec2 { x: x_point.x, y: x_point.y }, &Vec2 { x: y_point.x, y: y_point.y }) {
                                                            x_point.x = x_current_pos.x;
                                                            x_point.y = x_current_pos.y;
                                                            y_point.x = y_current_pos.x;
                                                            y_point.y = y_current_pos.y;
                                                        }

                                                        y_current_pos.curve_towards(y_cut.end, y_cut.center, 0.1, y_cut.clockwise)
                                                    }
                                                }
                                            }
                                        }
                                
                                        // Continue cut
                                        x_current_pos.move_towards(x_cut.end, 0.1);
                                    }
                                },
                                Cut::Curve(x_cut) => {
                                    // Same as above but curved
                                    // Start at the beginning of the cut
                                    let mut x_current_pos = Vec2 {
                                        x: x_cut.start.x,
                                        y: x_cut.start.y,
                                    };

                                    // Step through cut
                                    while x_current_pos != x_cut.end {
                                        // Loop over all cuts related the shape
                                        for y_cut_type in shape_cuts.get(*y_s).unwrap() {
                                            // Determine type of cut
                                            match y_cut_type {
                                                Cut::Linear(y_cut) => {
                                                    // Start at the beginning of the cut
                                                    let mut y_current_pos = Vec2 {
                                                        x: y_cut.start.x,
                                                        y: y_cut.start.y,
                                                    };

                                                    // Step through cut
                                                    while y_current_pos != y_cut.end {
                                                        // If current position makes smaller cut save it
                                                        if Vec2::distance(&Vec2 { x: x_current_pos.x, y: x_current_pos.y }, &Vec2 { x: y_current_pos.x, y: y_current_pos.y }) < Vec2::distance(&Vec2 { x: x_point.x, y: x_point.y }, &Vec2 { x: y_point.x, y: y_point.y }) {
                                                            x_point.x = x_current_pos.x;
                                                            x_point.y = x_current_pos.y;
                                                            y_point.x = y_current_pos.x;
                                                            y_point.y = y_current_pos.y;
                                                        }
                                                
                                                        // Continue cut
                                                        y_current_pos.move_towards(y_cut.end, 0.1);
                                                    }
                                                },
                                                Cut::Curve(y_cut) => {
                                                    // Same as above but curved
                                                    let mut y_current_pos = Vec2 {
                                                        x: y_cut.start.x,
                                                        y: y_cut.start.y,
                                                    };

                                                    // Step through cut
                                                    while y_current_pos != y_cut.end {
                                                        // If current position makes smaller cut save it
                                                        if Vec2::distance(&Vec2 { x: x_current_pos.x, y: x_current_pos.y }, &Vec2 { x: y_current_pos.x, y: y_current_pos.y }) < Vec2::distance(&Vec2 { x: x_point.x, y: x_point.y }, &Vec2 { x: y_point.x, y: y_point.y }) {
                                                            x_point.x = x_current_pos.x;
                                                            x_point.y = x_current_pos.y;
                                                            y_point.x = y_current_pos.x;
                                                            y_point.y = y_current_pos.y;
                                                        }

                                                        y_current_pos.curve_towards(y_cut.end, y_cut.center, 0.1, y_cut.clockwise)
                                                    }
                                                }
                                            }
                                        }

                                        x_current_pos.curve_towards(x_cut.end, x_cut.center, 0.1, x_cut.clockwise)
                                    }
                                }
                            }
                        }
                        // Save the smallest found cut
                        cuts.push(LinearCut::new(x_point, y_point));
                    }
                }
            }
        }
    }
    cuts
}