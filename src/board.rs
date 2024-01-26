use crate::players::Player;
use ggez::glam::Vec2;
use ggez::graphics::Color;

pub const BOARD_SIZE: f32 = 300.0;
pub const CELL_SIZE: f32 = BOARD_SIZE / 3.0;
pub const MESSAGE_PANEL_SIZE: (f32, f32) = (BOARD_SIZE, 100.0);
pub const WINDOW_SIZE: (f32, f32) = (BOARD_SIZE, BOARD_SIZE + MESSAGE_PANEL_SIZE.1);

#[derive(PartialEq, Clone, Copy)]
pub struct Cell {
    pub player: Option<Player>, // None means empty
    pub row: usize,
    pub col: usize,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Cell {
            player: None,
            row,
            col,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.player.is_none()
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2 {
            x: self.col as f32 * CELL_SIZE,
            y: self.row as f32 * CELL_SIZE,
        }
    }

    pub fn center(&self) -> Vec2 {
        self.top_left() + Vec2::splat(CELL_SIZE / 2.0)
    }

    pub fn color(&self) -> Color {
        match self.player {
            None => Color::WHITE,
            Some(player) => player.color(),
        }
    }
}

pub struct Board {
    pub cells: [[Cell; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [
                [Cell::new(0, 0), Cell::new(0, 1), Cell::new(0, 2)],
                [Cell::new(1, 0), Cell::new(1, 1), Cell::new(1, 2)],
                [Cell::new(2, 0), Cell::new(2, 1), Cell::new(2, 2)],
            ],
        }
    }

    pub fn at(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn set_player(&mut self, row: usize, col: usize, player: Player) {
        self.cells[row][col].player = Some(player);
    }

    pub fn rows(&self) -> [[Cell; 3]; 3] {
        self.cells
    }

    pub fn columns(&self) -> [[Cell; 3]; 3] {
        [
            [self.at(0, 0), self.at(1, 0), self.at(2, 0)],
            [self.at(0, 1), self.at(1, 1), self.at(2, 1)],
            [self.at(0, 2), self.at(1, 2), self.at(2, 2)],
        ]
    }

    pub fn diagonals(&self) -> [[Cell; 3]; 2] {
        [
            [self.at(0, 0), self.at(1, 1), self.at(2, 2)],
            [self.at(0, 2), self.at(1, 1), self.at(2, 0)],
        ]
    }

    pub fn closed_lines(&self) -> Vec<[Cell; 3]> {
        self.rows()
            .iter()
            .chain(self.columns().iter())
            .chain(self.diagonals().iter())
            .filter_map(|line| {
                let pivot_cell = line[0];
                let pivot = pivot_cell.player;

                match pivot {
                    None => None,
                    Some(_) => {
                        if line.iter().any(|cell| cell.player != pivot) {
                            None
                        } else {
                            Some(line.clone())
                        }
                    }
                }
            })
            .collect()
    }
}
