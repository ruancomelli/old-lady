use crate::players::{Player, PlayerDrawer};
use ggez::context::Has;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, GraphicsContext, Mesh, Rect};

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

    pub fn bounding_box(&self) -> Rect {
        Rect::new(
            self.col as f32 * CELL_SIZE,
            self.row as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        )
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

struct CellDrawer<'a, 'b> {
    pub ctx: &'a GraphicsContext,
    pub cell: &'b Cell,
    pub is_hovered: bool,
    pub is_winner: bool,
    pub line_width: f32,
}

impl Drawable for CellDrawer<'_, '_> {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        let param = param.into();

        let color = match self.cell.player {
            None => Color::WHITE,
            Some(player) => {
                if self.is_winner {
                    Color::YELLOW
                } else {
                    player.color()
                }
            }
        };

        if let Some(player) = self.cell.player {
            let player_drawer = PlayerDrawer {
                ctx: self.ctx,
                player,
                bounding_box: self.cell.bounding_box(),
                color,
                line_width: self.line_width,
            };

            player_drawer.draw(canvas, param);
        }

        let cell_mesh = Mesh::new_rectangle(
            self.ctx,
            if self.is_hovered {
                DrawMode::stroke(self.line_width)
            } else {
                DrawMode::fill()
            },
            self.cell.bounding_box(),
            set_color_transparency(self.cell.color(), 255.0 / 4.0),
        )
        .expect("Failed to create cell mesh");

        canvas.draw(&cell_mesh, param);
    }

    fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        Some(self.cell.bounding_box())
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

pub struct BoardDrawer<'a, 'b> {
    pub ctx: &'a GraphicsContext,
    pub board: &'b Board,
    pub mouse_on_cell: Option<Cell>,
    pub winner_cells: Vec<Cell>,
    pub line_width: f32,
}

impl Drawable for BoardDrawer<'_, '_> {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        let param = param.into();

        let board_mesh = Mesh::new_rectangle(
            self.ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, BOARD_SIZE, BOARD_SIZE),
            Color::BLACK,
        )
        .expect("Failed to create board mesh");

        canvas.draw(&board_mesh, param);

        for cell in self.board.cells.iter().flatten() {
            let cell_drawer = CellDrawer {
                ctx: self.ctx,
                cell,
                is_hovered: self.mouse_on_cell == Some(*cell),
                is_winner: self.winner_cells.contains(cell),
                line_width: self.line_width,
            };

            cell_drawer.draw(canvas, param);
        }

        for i in 1..3 {
            let vertical_rule = Mesh::new_line(
                self.ctx,
                &[
                    Vec2 {
                        x: i as f32 * CELL_SIZE,
                        y: 0.0,
                    },
                    Vec2 {
                        x: i as f32 * CELL_SIZE,
                        y: BOARD_SIZE,
                    },
                ],
                2.0,
                Color::WHITE,
            )
            .expect("Failed to create vertical rule");

            canvas.draw(&vertical_rule, Vec2::ZERO);
        }

        for i in 1..4 {
            let horizontal_rule = Mesh::new_line(
                self.ctx,
                &[
                    Vec2 {
                        x: 0.0,
                        y: i as f32 * CELL_SIZE,
                    },
                    Vec2 {
                        x: BOARD_SIZE,
                        y: i as f32 * CELL_SIZE,
                    },
                ],
                2.0,
                Color::WHITE,
            )
            .expect("Failed to create horizontal rule");

            canvas.draw(&horizontal_rule, Vec2::ZERO);
        }
    }

    fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        Some(Rect::new(0.0, 0.0, BOARD_SIZE, BOARD_SIZE))
    }
}

fn set_color_transparency(color: Color, a: f32) -> Color {
    Color::new(color.r, color.g, color.b, a)
}
