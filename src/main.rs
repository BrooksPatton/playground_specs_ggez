use bbggez;
use playground_specs_ggez::Game;

fn main() {
    let mut game = Game::new();

    bbggez::run::run_dim(
        &mut game,
        1024.0,
        768.0,
        "SPECS and GGEZ Playground",
        "Brooks Patton",
    );
}
