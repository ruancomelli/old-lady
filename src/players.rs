use ggez::context::Has;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, GraphicsContext, Mesh, Rect};
use ggez::mint::Point2;
use std::fmt::Display;
#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::X => Color::RED,
            Self::O => Color::BLUE,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

pub struct PlayerDrawer<'a> {
    pub ctx: &'a GraphicsContext,
    pub player: Player,
    pub bounding_box: Rect,
    pub color: Color,
    pub line_width: f32,
}

impl<'a> PlayerDrawer<'a> {
    pub fn left(&self) -> f32 {
        self.bounding_box.left()
    }

    pub fn right(&self) -> f32 {
        self.bounding_box.right()
    }

    pub fn width(&self) -> f32 {
        self.bounding_box.w
    }

    pub fn top(&self) -> f32 {
        self.bounding_box.top()
    }

    pub fn bottom(&self) -> f32 {
        self.bounding_box.bottom()
    }

    pub fn height(&self) -> f32 {
        self.bounding_box.h
    }

    pub fn center(&self) -> Point2<f32> {
        self.bounding_box.center()
    }

    pub fn top_left(&self) -> Point2<f32> {
        Point2 {
            x: self.left(),
            y: self.top(),
        }
    }

    pub fn top_right(&self) -> Point2<f32> {
        Point2 {
            x: self.right(),
            y: self.top(),
        }
    }

    pub fn bottom_left(&self) -> Point2<f32> {
        Point2 {
            x: self.left(),
            y: self.bottom(),
        }
    }

    pub fn bottom_right(&self) -> Point2<f32> {
        Point2 {
            x: self.right(),
            y: self.bottom(),
        }
    }
}

impl<'a> Drawable for PlayerDrawer<'a> {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        let param = param.into();

        let line_width = self.line_width;

        match self.player {
            Player::X => {
                let line1 = Mesh::new_line(
                    self.ctx,
                    &[self.top_left(), self.bottom_right()],
                    line_width,
                    self.color,
                )
                .expect("Failed to create line '\\' for X");
                let line2 = Mesh::new_line(
                    self.ctx,
                    &[self.top_right(), self.bottom_left()],
                    line_width,
                    self.color,
                )
                .expect("Failed to create line '/' for X");

                canvas.draw(&line1, param);
                canvas.draw(&line2, param);
            }
            Player::O => {
                let circle_border = Mesh::new_ellipse(
                    self.ctx,
                    DrawMode::stroke(line_width),
                    self.center(),
                    self.width() / 2.0,
                    self.height() / 2.0,
                    1.0,
                    self.color,
                )
                .expect("Failed to create circle for O");
                canvas.draw(&circle_border, param);
            }
        }
    }

    fn dimensions(&self, _gfx: &impl Has<GraphicsContext>) -> Option<Rect> {
        Some(self.bounding_box)
    }
}
