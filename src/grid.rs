use std::ops::{Index, IndexMut};

pub struct Grid<T: Clone> {
    grid: Vec<T>,
    pub width: usize,
    pub height: usize,
    resolution: usize,
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
    pub fn new(width: usize, height: usize, default: T, resolution: usize) -> Grid<T> {
        Grid {
            grid: vec![default; width * height],
            width: width,
            height: height,
            resolution: resolution,
        }
    }

    pub fn get<usize>(&self, x: usize, y: usize) -> Option<&T> {
        return if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(&self[(x, y)])
        } else {
            None
        }
    }

    pub fn get<f32>(&self, x: f32, y: f32) -> Option<&T> {
        return if (0..self.width).contains(&(x as usize)) && (0..self.height).contains(&(y as usize)) {
            Some(&self[(x as usize, y as usize)])
        } else {
            None
        }
    }

    pub fn get_mut<usize>(&mut self, x: usize, y: usize) -> Option<&mut T> {
        return if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(&mut self[(x, y)])
        } else {
            None
        }
    }

    pub fn get_mut<f32>(&mut self, x: f32, y: f32) -> Option<&mut T> {
        return if (0..self.width).contains(&(x as usize)) && (0..self.height).contains(&(y as usize)) {
            Some(&mut self[(x as usize, y as usize)])
        } else {
            None
        }
    }
}