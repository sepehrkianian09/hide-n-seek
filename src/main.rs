// replace application with the name of your package
use application::{game, unit::Enemy};

fn main() {
    let mut game = game::Game::builder()
        .player_starting_health(10)
        .player_starting_speed(0.5)
        .enemies(
            (1..15)
                .map(|i| Enemy::with_speed(i as f64 * 0.05))
                .collect(),
        )
        .build();
    game.run();
}
