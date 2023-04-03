use std::fmt::Display;

use termion::clear;
mod screen;
use screen::{Render, Screen};

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
}

impl Cell {
    fn new(alive: bool) -> Self {
        Self { alive }
    }
    fn analyze_neighbors(&self, alive_neighbor_count: usize) -> bool {
        // live cell:
        if self.alive {
            match alive_neighbor_count {
                // dies if less than 2 living neighbors
                0..=1 => false,
                // 3 + live neighbors dies
                3.. => false,
                _ => true,
            }
        } else {
            if alive_neighbor_count == 3 {
                // dead cell:
                // exactly 3 live neighbors: comes to life
                true
            } else {
                false
            }
        }
    }
}

impl Display for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", if self.alive { "x" } else { " " })
    }
}

struct Grid {
    size: usize, //size x size
    contents: Vec<Cell>,
}

impl Grid {
    fn new(size: usize, probability_of_alive: f32) -> Self {
        let mut contents = vec![];
        for _ in 0..size * size {
            contents.push(Cell::new(rand::random::<f32>() < probability_of_alive))
        }
        Self { size, contents }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.size) + x
    }
    fn generate_mut(&mut self) {
        let mut contents = vec![];
        for y in 0..self.size {
            for x in 0..self.size {
                contents.push(Cell::new(self.cycle(x, y)))
            }
        }
        self.contents = contents
    }
    fn generate(self) -> Self {
        let mut contents = vec![];
        for y in 0..self.size {
            for x in 0..self.size {
                contents.push(Cell::new(self.cycle(x, y)))
            }
        }
        Self {
            contents,
            size: self.size,
        }
    }
    fn cycle(&self, x: usize, y: usize) -> bool {
        let center_index = self.get_index(x, y);
        let neighbors = self.get_neighbors(x, y);
        let alive_neighbor_count = neighbors.iter().filter(|x| x.alive).count();
        self.contents
            .get(center_index)
            .unwrap()
            .analyze_neighbors(alive_neighbor_count)
    }
    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&Cell> {
        let mut neighbors = vec![];
        // get left neighbor
        if x > 0 {
            let left_index = self.get_index(x - 1, y);
            neighbors.push(&self.contents[left_index]);
        }
        // get right neighbor
        if x < (self.size - 1) {
            let right_index = self.get_index(x + 1, y);
            neighbors.push(&self.contents[right_index]);
        }
        // get top neighbor
        if y > 0 {
            let top_index = self.get_index(x, y - 1);
            neighbors.push(&self.contents[top_index]);
            //get top left diagonal
            if x > 0 {
                let diag_l_index = self.get_index(x - 1, y - 1);
                neighbors.push(&self.contents[diag_l_index]);
            }
            // get top right diagonal
            if x < (self.size - 1) {
                let diag_r_index = self.get_index(x + 1, y - 1);
                neighbors.push(&self.contents[diag_r_index]);
            }
        }
        // get bottom neighbor
        if y < (self.size - 1) {
            let bottom_index = self.get_index(x, y + 1);
            neighbors.push(&self.contents[bottom_index]);
            //get bottom left diagonal
            if x > 0 {
                let diag_l_index = self.get_index(x - 1, y + 1);
                neighbors.push(&self.contents[diag_l_index]);
            }
            // get top right diagonal
            if x < (self.size - 1) {
                let diag_r_index = self.get_index(x + 1, y + 1);
                neighbors.push(&self.contents[diag_r_index]);
            }
        }

        neighbors
    }
}

impl Display for Grid {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut rows = String::new();
        for y in 0..self.size {
            let mut row = String::new();
            for x in 0..self.size {
                row.push_str(&format!("{} ", self.contents[self.get_index(x, y)]))
            }
            rows.push_str(&format!("{}\n", row))
        }
        write!(formatter, "{}", rows)
    }
}

fn main() {
    let mut grid = Grid::new(50, 0.3);
    println!("{}", grid);
    println!("{}", grid);
    let screen = Screen::new(4);
    let mut count = 0;
    screen.do_loop(|| {
        println!("{}", grid);
        grid.generate_mut();
        count += 1;
        count == 50
    })
}

#[cfg(test)]
#[test]
fn get_index() {
    let grid = Grid::new(10, 0.3);
    assert_eq!(grid.get_index(1, 0), 1);
    assert_eq!(grid.get_index(1, 1), 11);
    assert_eq!(grid.get_index(2, 1), 12);
    assert_eq!(grid.get_index(0, 0), 0);
}

#[test]
fn get_neighbors() {
    let grid = Grid::new(10, 0.5);
    assert_eq!(grid.get_neighbors(0, 0).len(), 3);
    assert_eq!(grid.get_neighbors(1, 1).len(), 8);
    assert_eq!(grid.get_neighbors(2, 9).len(), 5);
    assert_eq!(grid.get_neighbors(9, 2).len(), 5);
    assert_eq!(grid.get_neighbors(9, 9).len(), 3);
}
#[test]
fn analyze_neighbors() {
    let mut grid = Grid::new(10, 0.5);
    let mut cell_coords = grid.get_index(0, 0);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(1, 0);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(2, 0);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(0, 1);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(2, 1);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(0, 2);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(1, 2);
    grid.contents[cell_coords].alive = false;
    cell_coords = grid.get_index(2, 2);
    grid.contents[cell_coords].alive = false;

    assert_eq!(grid.cycle(1, 1), false);
}
