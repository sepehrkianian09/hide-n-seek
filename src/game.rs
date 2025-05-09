mod builder;
mod tests;

use std::{
    cell::RefCell,
    fmt::Debug,
    io::{stdout, Stdout, Write},
    time::Duration,
};

use builder::GameBuilder;
use derivative::Derivative;
use num::traits::NumAssign;
use rand::{rngs::ThreadRng, Rng, RngCore};

use serde::{Deserialize, Serialize};

use crate::{
    hud::Hud,
    input,
    point::Point2d,
    traits::*,
    ui::{draw::*, UI},
    unit::{Collectible, Enemy, Player, PlayerBuilder, Wall},
};

fn rng_new() -> RefCell<Box<dyn RngCore>> {
    RefCell::new(Box::new(rand::thread_rng()))
}

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Game {
    height: u16,
    width: u16,
    #[serde(skip, default = "stdout")]
    stdout: Stdout,
    enemies: RefCell<Vec<Enemy>>,
    n_random_walls: u16,
    walls: Vec<Wall>,
    collectible: RefCell<Collectible>,
    player: RefCell<Player>,
    #[serde(skip, default = "crate::ui::UI::new")]
    ui: UI,
    #[serde(skip, default = "rng_new")]
    #[derivative(Debug = "ignore")]
    rng: RefCell<Box<dyn RngCore>>,
    update_interval_millis: Duration,
    hud: Hud,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
            && self.width == other.width
            && self.enemies == other.enemies
            && self.n_random_walls == other.n_random_walls
            && self.walls == other.walls
            && self.collectible == other.collectible
            && self.player == other.player
            && self.update_interval_millis == other.update_interval_millis
    }
}

impl Game {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn score(&self) -> u32 {
        self.collectible.borrow().score()
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
            wall.set_rand_position(
                &mut self.rng.borrow_mut(),
                1..self.width - 1,
                1..self.height - 1,
            );
            self.walls.push(wall);
        }

        // randomize enemy positions
        self.enemies.borrow_mut().iter_mut().for_each(|enemy| {
            enemy.set_rand_position(
                &mut self.rng.borrow_mut(),
                1.0..(self.width - 1).into(),
                1.0..(self.height - 1).into(),
            );
        });

        // randomize collectible position
        self.collectible.borrow_mut().randomize_position(self);
    }

    pub fn do_walls_collide(&self, position: Point2d<u16>) -> bool {
        self.walls.iter().any(|wall| wall.position() == position)
    }

    pub fn player_position(&self) -> Point2d<f64> {
        self.player.borrow().position()
    }

    pub fn damage_player(&self, damage: u8) {
        self.player.borrow_mut().take_damage(damage);
    }

    pub fn randomize_position_u16(&self, a_position: &mut dyn Position<u16>) {
        a_position.set_rand_position(
            &mut self.rng.borrow_mut(),
            1..self.width - 1,
            1..self.height - 1,
        );
    }

    fn update(&mut self, since_last_time: Duration) {
        self.player.borrow_mut().update(self, &since_last_time);

        self.collectible
            .borrow_mut()
            .update(&self, &since_last_time);

        self.enemies.borrow_mut()
            .iter_mut()
            .for_each(|enemy: &mut Enemy| enemy.update(&self, &since_last_time));

        self.hud.set(self.score(), self.player.borrow().health());
    }

    fn draw(&mut self) {
        self.ui.clear();
        let mut buffer: Vec<u8> = Vec::new();
        self.walls.iter().for_each(|wall| wall.draw(&mut buffer));
        self.player.borrow().draw(&mut buffer);
        self.enemies.borrow()
            .iter()
            .for_each(|enemy| enemy.draw(&mut buffer));
        self.collectible.borrow().draw(&mut buffer);
        self.hud.draw(&mut buffer);
        self.stdout
            .write_all(&buffer)
            .expect("failed to write to stdout");
        self.stdout.flush().expect("Failed to flush stdout");
    }

    pub fn run(&mut self) {
        self.init();
        let mut quit = false;
        while self.player.borrow().is_alive() && !quit {
            // poll for key events for the duration of the update interval
            let now = std::time::Instant::now();
            while let Some(time_remaining) = self.update_interval_millis.checked_sub(now.elapsed())
            {
                if let Some(key) = input::poll_key_event(time_remaining) {
                    input::handle_key_event(key, &mut self.player.borrow_mut(), &mut quit);
                }
            }

            self.update(self.update_interval_millis.clone());
            self.draw();
        }
        self.ui.restore();
        print!("\nGame over!");
        println!("  Score: {}", self.score());
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::builder().build()
    }
}
