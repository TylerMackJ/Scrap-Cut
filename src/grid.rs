use std::ops::{Index, IndexMut};

pub struct Grid<T: Clone> {
    grid: Vec<T>,
    pub width: usize,
    pub height: usize,
    pub resolution: usize,
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
    pub fn new(width: usize, height: usize, resolution: usize, default: T) -> Grid<T> {
        Grid {
            grid: vec![default; (width / resolution) * (height / resolution)],
            width: width / resolution,
            height: height / resolution,
            resolution,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(&self[(x, y)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(&mut self[(x, y)])
        } else {
            None
        }
    }

    /*
    pub fn sheet_get(&self, x: f32, y: f32) -> Option<&T> {
        let rough_x = (x / self.resolution as f32) as usize;
        let rough_y = (y / self.resolution as f32) as usize;
        return if (0..self.width).contains(&rough_x) && (0..self.height).contains(&rough_y) {
            Some(&self[(rough_x, rough_y)])
        } else {
            None
        }
    }
    */

    pub fn sheet_get_mut(&mut self, x: f32, y: f32) -> Option<&mut T> {
        let rough_x = (x / self.resolution as f32) as usize;
        let rough_y = (y / self.resolution as f32) as usize;
        if (0..self.width).contains(&rough_x) && (0..self.height).contains(&rough_y) {
            Some(&mut self[(rough_x, rough_y)])
        } else {
            None
        }
    }
}