#![cfg(test)] // note the ! which applies the attribute to the whole module and not just the subsequent item

use std::error::Error;

use rand::{rngs::StdRng, SeedableRng};

use crate::common::JsonIo;

use super::*;

#[test]
fn test_game_after_updates() -> Result<(), Box<dyn Error>> {
    let rng = StdRng::seed_from_u64(42);

    let mut game = Game::builder()
        .n_random_walls(30)
        .height(40)
        .player_starting_health(10)
        .player_starting_speed(0.5)
        .enemies(
            (1..10)
                .map(|i| Enemy::with_speed(i as f64 * 0.05))
                .collect(),
        )
        .update_interval(std::time::Duration::from_millis(280))
        .rng(Box::new(rng))
        .build();
    
    let json_io = JsonIo::new("src/game/initial_game.json");
    json_io.write_json(&game)?;
    assert_eq!(game, json_io.read_json()?);

    game.init();
    for _ in 0..5 {
        game.update(std::time::Duration::from_millis(280));
    }

    let json_io = JsonIo::new("src/game/after_update_game.json");
    json_io.write_json(&game)?;
    let read_game = json_io.read_json()?;
    assert_eq!(game, read_game);

    Ok(())
}