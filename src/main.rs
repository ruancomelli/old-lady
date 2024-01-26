// A tic-tac-toe game implementation

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::ContextBuilder;

mod board;
mod game_state;
mod players;

use board::WINDOW_SIZE;
use game_state::GameState;

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
