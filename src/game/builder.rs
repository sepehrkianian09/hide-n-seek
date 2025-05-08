use std::{io::stdout, time::Duration};

use rand::RngCore;

use crate::{ui::UI, unit::{Collectible, Enemy, PlayerBuilder, Wall}};

use super::Game;


pub struct GameBuilder {
    height: u16,
    width: u16,
    n_random_walls: u16,
    update_interval: Duration,
    player_builder: PlayerBuilder,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    rng: Box<dyn RngCore>,
}

impl GameBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            height: 48,
            width: 80,
            player_builder: PlayerBuilder::new(),
            n_random_walls: 0,
            update_interval: Duration::from_millis(50),
            enemies: vec![
                Enemy::with_speed(0.6),
                Enemy::with_speed(0.5),
                Enemy::with_speed(0.4),
            ],
            walls: vec![],
            rng: Box::new(rand::thread_rng()),
        }
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn player_starting_health(mut self, health: u8) -> Self {
        self.player_builder = self.player_builder.health(health);
        self
    }

    pub fn player_starting_speed(mut self, speed: f64) -> Self {
        self.player_builder = self.player_builder.speed(speed);
        self
    }

    pub fn n_random_walls(mut self, n_random_walls: u16) -> Self {
        self.n_random_walls = n_random_walls;
        self
    }

    pub fn update_interval(mut self, update_interval: Duration) -> Self {
        self.update_interval = update_interval;
        self
    }

    pub fn enemies(mut self, enemies: Vec<Enemy>) -> Self {
        self.enemies = enemies;
        self
    }

    pub fn walls(mut self, walls: Vec<Wall>) -> Self {
        self.walls = walls;
        self
    }

    pub fn rng(mut self, rng: Box<dyn RngCore>,) -> Self {
        self.rng = rng;
        self
    }

    pub fn build(self) -> Game {
        Game {
            height: self.height,
            width: self.width,
            n_random_walls: self.n_random_walls,
            update_interval_millis: self.update_interval,
            enemies: self.enemies,
            walls: self.walls,
            collectible: Collectible::default(),
            player: self.player_builder.build(),
            ui: UI::new(),
            rng: self.rng,
            stdout: stdout(),
            score: 0,
        }
    }
}