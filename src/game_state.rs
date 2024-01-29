use crate::board::{Board, BoardDrawer, Cell, BOARD_SIZE, CELL_SIZE};
use crate::players::Player;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, Drawable, PxScale, TextFragment};
use ggez::{Context, GameResult};

pub struct GameState {
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
        if self.winner.is_none() {
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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let message_fragment = self.message();
        let message_text = graphics::Text::new(message_fragment);
        let message_pos = Vec2 {
            x: 10.0,
            y: BOARD_SIZE + 10.0,
        };

        canvas.draw(&message_text, message_pos);

        let board_drawer = BoardDrawer {
            ctx: &ctx.gfx,
            board: &self.board,
            mouse_on_cell: self.mouse_on_cell,
            winner_cells: self
                .board
                .closed_lines()
                .iter()
                .flatten()
                .copied()
                .collect(),
            line_width: 10.0,
            offset: Vec2::ZERO,
        };

        board_drawer.draw(&mut canvas, graphics::DrawParam::default());

        canvas.finish(ctx)?;
        Ok(())
    }
}
