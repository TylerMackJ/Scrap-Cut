pub struct Grid<T: Clone> {
    grid: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Clone> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[x + (y * self.width)]
    }
}

impl<T: Clone> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[x + (y * self.width)]
    }
}

impl<T: Clone> Grid<T> {
    fn new(width: usize, height: usize, default: T) -> Grid<T> {
        Grid {
            grid: vec![default; width * height],
            width: width,
            height: height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&T> {
        return if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(&self[(x, y)])
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        return if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(&mut self[(x, y)])
        } else {
            None
        }
    }
}