mod tests;
mod builder;

use std::{
    fmt::Debug, io::{stdout, Stdout, Write}, time::Duration
};

use builder::GameBuilder;
use derivative::Derivative;
use rand::{rngs::ThreadRng, Rng, RngCore};

use serde::{Serialize, Deserialize};

use crate::{
    hud::Hud, input, point::Point2d, traits::*, ui::{draw::*, UI}, unit::{Collectible, Enemy, Player, PlayerBuilder, Wall}
};

fn rng_new() -> Box<dyn RngCore> {
    Box::new(rand::thread_rng())
}

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Game {
    height: u16,
    width: u16,
    #[serde(skip, default="stdout")]
    stdout: Stdout,
    score: u32,
    enemies: Vec<Enemy>,
    n_random_walls: u16,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
    #[serde(skip, default="crate::ui::UI::new")]
    ui: UI,
    #[serde(skip, default="rng_new")]
    #[derivative(Debug = "ignore")]
    rng: Box<dyn RngCore>,
    update_interval_millis: Duration,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height && self.width == other.width && self.score == other.score && self.enemies == other.enemies && self.n_random_walls == other.n_random_walls && self.walls == other.walls && self.collectible == other.collectible && self.player == other.player && self.update_interval_millis == other.update_interval_millis
    }
}

impl Game {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn init(&mut self) {
        self.ui.prepare();

        // surround the game area with walls
        for x in 0..self.width {
            self.walls.push(Wall::new(x, 0));
            self.walls.push(Wall::new(x, self.height - 1));
        }
        for y in 0..self.height {
            self.walls.push(Wall::new(0, y));
            self.walls.push(Wall::new(self.width - 1, y));
        }

        // add random walls
        for _ in 0..self.n_random_walls {
            let mut wall = Wall::default();
            wall.set_rand_position(&mut self.rng, 1..self.width - 1, 1..self.height - 1);
            self.walls.push(wall);
        }

        // randomize enemy positions
        self.enemies.iter_mut().for_each(|enemy| {
            enemy.set_rand_position(
                &mut self.rng,
                1.0..(self.width - 1).into(),
                1.0..(self.height - 1).into(),
            );
        });

        // randomize collectible position
        while self
            .walls
            .iter()
            .any(|wall| wall.position() == self.collectible.position())
        {
            self.collectible.set_rand_position(
                &mut self.rng,
                1..self.width - 1,
                1..self.height - 1,
            );
        }
    }

    fn do_walls_collide(&self, position: Point2d<u16>) -> bool {
        self
            .walls
            .iter()
            .any(|wall| wall.position() == position)
    }

    fn update(&mut self, since_last_time: Duration) {
        // move player if not colliding with a wall
        let player_next_position = self.player.forward_position();
        if !self.do_walls_collide(player_next_position)
        {
            self.player.move_forward(&since_last_time);
        }

        // increase score if player collides with collectible
        if self.player.position().round().to_u16() == self.collectible.position() {
            self.score += 1;
            // move collectible to a new random position
            self.collectible.set_rand_position(
                &mut self.rng,
                1..self.width - 1,
                1..self.height - 1,
            );
            while self.do_walls_collide(self.collectible.position())
            {
                self.collectible.set_rand_position(
                    &mut self.rng,
                    1..self.width - 1,
                    1..self.height - 1,
                );
            }
        }

        // move enemies
        self.enemies
            .iter_mut()
            .for_each(|enemy: &mut Enemy| enemy.move_towards_player(&self.player, &since_last_time));

        // reduce player health for each enemy collision
        self.enemies.iter_mut().for_each(|enemy| {
            if enemy.position().round() == self.player.position().round() {
                self.player.take_damage(1);
            }
        });
    }

    fn draw(&mut self) {
        self.ui.clear();
        let mut buffer: Vec<u8> = Vec::new();
        self.walls.iter().for_each(|wall| wall.draw(&mut buffer));
        self.player.draw(&mut buffer);
        self.enemies
            .iter()
            .for_each(|enemy| enemy.draw(&mut buffer));
        self.collectible.draw(&mut buffer);
        Hud::new(self.score, &self.player, Point2d::new(self.width / 2 - 10, self.height + 2)).draw(&mut buffer);
        self.stdout
            .write_all(&buffer)
            .expect("failed to write to stdout");
        self.stdout.flush().expect("Failed to flush stdout");
    }

    pub fn run(&mut self) {
        self.init();
        let mut quit = false;
        while self.player.is_alive() && !quit {
            // poll for key events for the duration of the update interval
            let now = std::time::Instant::now();
            while let Some(time_remaining) = self.update_interval_millis.checked_sub(now.elapsed())
            {
                if let Some(key) = input::poll_key_event(time_remaining) {
                    input::handle_key_event(key, &mut self.player, &mut quit);
                }
            }

            self.update(self.update_interval_millis.clone());
            self.draw();
        }
        self.ui.restore();
        print!("\nGame over!");
        println!("  Score: {}", self.score);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::builder().build()
    }
}
