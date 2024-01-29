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
    pub scale: f32,
}

impl<'a> PlayerDrawer<'a> {
    pub fn drawing_box(&self) -> Rect {
        rescale_rect_around_center(self.bounding_box, self.scale)
    }
}

impl<'a> Drawable for PlayerDrawer<'a> {
    fn draw(&self, canvas: &mut Canvas, param: impl Into<DrawParam>) {
        let param = param.into();

        let line_width = self.line_width;

        let drawing_box = self.drawing_box();

        match self.player {
            Player::X => {
                let line1 = Mesh::new_line(
                    self.ctx,
                    &[
                        Point2 {
                            x: drawing_box.left(),
                            y: drawing_box.top(),
                        },
                        Point2 {
                            x: drawing_box.right(),
                            y: drawing_box.bottom(),
                        },
                    ],
                    line_width,
                    self.color,
                )
                .expect("Failed to create line '\\' for X");
                let line2 = Mesh::new_line(
                    self.ctx,
                    &[
                        Point2 {
                            x: drawing_box.left(),
                            y: drawing_box.bottom(),
                        },
                        Point2 {
                            x: drawing_box.right(),
                            y: drawing_box.top(),
                        },
                    ],
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
                    drawing_box.center(),
                    drawing_box.w / 2.0,
                    drawing_box.h / 2.0,
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

fn rescale_rect_around_center(rect: Rect, scale: f32) -> Rect {
    let center = rect.center();
    let w = rect.w * scale;
    let h = rect.h * scale;
    Rect::new(center.x - w / 2.0, center.y - h / 2.0, w, h)
}
