#[derive(Clone, PartialEq)]
enum Square { Free, Taken(u8), Scrap, Good }

struct Sheet {
    width: usize,
    height: usize
}

struct Vec2 {
    x: f32,
    y: f32
}

struct Line2D {
    start: Vec2,
    end: Vec2
}

fn main() {
    let sheet = Sheet {
        width: 48,
        height: 96
    };

    // Create Grid
    let resolution: usize = 2;
    let mut grid: Vec<Vec<Square>> = vec![Vec::new(); sheet.height / resolution];

    for mut row in grid
    {
        row = vec![Square::Free; sheet.width / resolution];
    }

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
    for i in 0..grid.len()
    {
        for j in 0..grid[0].len()
        {
            // All Square::Free change
            if grid[i][j] == Square::Free
            {
                // Square::Good if 2 orthogonal squares are not Square::Taken else Square::Scrap
                if (grid[i + 1][j] != Square::Taken(_) || grid[i - 1][j] != Square::Taken(_)) && (grid[i][j + 1] != Square::Taken(_) || grid[i][j - 1] != Square::Taken(_))
                {
                    grid[i][j] == Square::Good;
                }
                else
                {
                    grid[i][j] == Square::Scrap;
                }
            }
        }
    }

    // Find all the cuts
    //let mut cuts = Vec::new<Line2D>();
    for i in 0..grid.len()
    {
        for j in 0..grid[0].len()
        {
            // If Square::Scrap is next to Square::Good and all by itself
                // Change to Square::Good

            // Find Square::Good-Square::Scrap cuts
            if grid[i][j] == Square::Scrap && (grid[i + 1][j] == Square::Good || grid[i - 1][j] == Square::Good || grid[i][j + 1] == Square::Good || grid[i][j - 1] == Square::Good)
            {
                // Find each shape that has a Square::Taken touching the current square
                    // Make cut from middle of current square to the closest point of each shape
            }

            // Find Taken-Good-Wall Cuts
            // Check top and bottom
            if i == 0 || i == grid.len() - 1
            {
                if grid[i][j] == Square::Taken(_) && grid[i][j + 1] == Square::Good
                {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }

            // Check left and right
            if j == 0 || j == grid[0].len() - 1
            {
                if grid[i][j] == Square::Taken(_) && grid[i + 1][j] == Square::Good
                {
                    // Cut where the shape belonging to the current square is closest to the wall
                }
            }

            // Find xTaken-yTaken-Good cuts
            // Find 2x2 groups of xTaken-yTaken-Good-Good squares
            let mut goodCount = 0;
            let mut xTaken: Option<Square> = None;
            let mut yTaken: Option<Square> = None;
            for k in 0..2
            {
                for l in 0..2
                {
                    match grid[i + k][j + l]
                    {
                        Square::Taken(_) => {
                            if xTaken.is_none()
                            {
                                xTaken = grid[i + k][j + l];
                            }
                            else
                            {
                                yTaken = grid[i + k][j + l];
                            }
                        },
                        Square::Good => goodCount += 1,
                    }
                }
            }
            if goodCount == 2 && !xTaken.is_none() && !yTaken.is_none()
            {
                // Cut the thinnest point between xTaken and yTaken
            }
        }
    }

    /*
    // Add the cuts to gcode
    for cut in cuts
    {
        
    }
    */
}