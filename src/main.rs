// A tic-tac-toe game implementation

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, PxScale, TextFragment};
use ggez::{Context, ContextBuilder, GameResult};

mod players;
use players::Player;

const BOARD_SIZE: f32 = 300.0;
const CELL_SIZE: f32 = BOARD_SIZE / 3.0;
const MESSAGE_PANEL_SIZE: (f32, f32) = (BOARD_SIZE, 100.0);
const WINDOW_SIZE: (f32, f32) = (BOARD_SIZE, BOARD_SIZE + MESSAGE_PANEL_SIZE.1);

#[derive(PartialEq, Clone, Copy)]
pub struct Cell {
    player: Option<Player>, // None means empty
    row: usize,
    col: usize,
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
    cells: [[Cell; 3]; 3],
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

    pub fn cells_with_position(&self) -> Vec<((usize, usize), Cell)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .map(|(col, cell)| ((row, col), *cell))
                    .collect::<Vec<((usize, usize), Cell)>>()
            })
            .collect()
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

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) =
        ContextBuilder::new("tic-tac-toe", "Ruan Comelli (ruancomelli@gmail.com)")
            .window_setup(WindowSetup::default().title("Tic-Tac-Toe"))
            .window_mode(
                WindowMode::default()
                    .dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1)
                    .resizable(false),
            )
            .build()
            .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let game = GameState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, game);
}

struct GameState {
    board: Board,
    player: Player,
    mouse_on_cell: Option<Cell>,
    winner: Option<Player>,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        GameState {
            board: Board::new(),
            player: Player::X,
            mouse_on_cell: None,
            winner: None,
        }
    }

    pub fn message(&self) -> TextFragment {
        let message = match self.winner {
            None => format!("Player {}'s turn", self.player),
            Some(player) => format!("Player {} won!", player),
        };
        let color = match self.winner {
            None => self.player.color(),
            Some(_) => Color::YELLOW,
        };

        TextFragment::new(message)
            .scale(PxScale::from(30.0))
            .color(color)
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        if let None = self.winner {
            let row = (y / CELL_SIZE) as i32;
            let col = (x / CELL_SIZE) as i32;

            if (0..3).contains(&row) && (0..3).contains(&col) {
                self.mouse_on_cell = Some(self.board.at(row as usize, col as usize));
            } else {
                self.mouse_on_cell = None;
            }
        }

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> GameResult {
        if let None = self.winner {
            if let Some(cell) = self.mouse_on_cell {
                if cell.is_empty() {
                    let row = cell.row;
                    let col = cell.col;

                    self.board.set_player(row, col, self.player);

                    if self.board.closed_lines().is_empty() {
                        // No winner yet
                        self.player = self.player.next();
                    } else {
                        self.winner = Some(self.player);
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        let message_fragment = self.message();
        let message_text = graphics::Text::new(message_fragment);
        let message_pos = Vec2 {
            x: 10.0,
            y: BOARD_SIZE + 10.0,
        };

        canvas.draw(&message_text, message_pos);

        for cell in self.board.cells.iter().flatten() {
            let row = cell.row;
            let col = cell.col;

            match cell.player {
                Some(player) => {
                    let text_fragment = graphics::TextFragment::new(format!("{player}"))
                        .scale(PxScale::from(50.0))
                        .color(if let Some(winner) = self.winner {
                            if winner == player {
                                Color::YELLOW
                            } else {
                                player.color()
                            }
                        } else {
                            player.color()
                        });

                    let text = graphics::Text::new(text_fragment);
                    let pos = cell.top_left() + Vec2::splat(CELL_SIZE * 0.35);

                    canvas.draw(&text, pos);
                }
                None => {}
            }

            if self.mouse_on_cell.is_some_and(|c| c == *cell) {
                let cell_color = cell.color();
                let (r, g, b) = cell_color.to_rgb();
                let transparent_cell_color = graphics::Color::from_rgba(r, g, b, 255 / 4);

                let cell_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE),
                    transparent_cell_color,
                )?;

                canvas.draw(&cell_mesh, cell.top_left());

                let text_fragment =
                    graphics::TextFragment::new(format!("{player}", player = self.player))
                        .scale(PxScale::from(50.0))
                        .color(cell.color());

                let text = graphics::Text::new(text_fragment);
                let pos = Vec2 {
                    x: col as f32 * CELL_SIZE + CELL_SIZE * 0.35,
                    y: row as f32 * CELL_SIZE + CELL_SIZE * 0.35,
                };

                canvas.draw(&text, pos);
            }
        }

        for i in 1..3 {
            let vertical_rule = graphics::Mesh::new_line(
                ctx,
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
                graphics::Color::WHITE,
            )?;
            canvas.draw(&vertical_rule, Vec2 { x: 0.0, y: 0.0 });
        }

        for i in 1..4 {
            let horizontal_rule = graphics::Mesh::new_line(
                ctx,
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
                graphics::Color::WHITE,
            )?;

            canvas.draw(&horizontal_rule, Vec2 { x: 0.0, y: 0.0 });
        }

        for closed_line in self.board.closed_lines().iter() {
            for (origin, dest) in closed_line.iter().zip(closed_line.iter().skip(1)) {
                let line = graphics::Mesh::new_line(
                    ctx,
                    &[origin.center(), dest.center()],
                    10.0,
                    graphics::Color::YELLOW,
                )?;

                canvas.draw(&line, Vec2 { x: 0.0, y: 5.0 });
            }
        }

        canvas.finish(ctx)?;
        Ok(())
        // Draw code here...
    }
}
