use rand::rngs::ThreadRng;

use crate::{
    traits::*,
    unit::Collectible,
    unit::Enemy,
    unit::Wall,
    unit::{Player, PlayerBuilder},
};

pub struct Game {
    height: u16,
    width: u16,
    enemies: Vec<Enemy>,
    n_random_walls: u16,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
    rng: ThreadRng,
}

pub struct GameBuilder {
    height: u16,
    width: u16,
    n_random_walls: u16,
    player_builder: PlayerBuilder,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
}

impl GameBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            height: 48,
            width: 80,
            player_builder: PlayerBuilder::new(),
            n_random_walls: 0,
            enemies: vec![
                Enemy::with_speed(0.6),
                Enemy::with_speed(0.5),
                Enemy::with_speed(0.4),
            ],
            walls: vec![],
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

    pub fn enemies(mut self, enemies: Vec<Enemy>) -> Self {
        self.enemies = enemies;
        self
    }

    pub fn walls(mut self, walls: Vec<Wall>) -> Self {
        self.walls = walls;
        self
    }

    pub fn build(self) -> Game {
        Game {
            height: self.height,
            width: self.width,
            n_random_walls: self.n_random_walls,
            enemies: self.enemies,
            walls: self.walls,
            collectible: Collectible::default(),
            player: self.player_builder.build(),
            rng: rand::thread_rng(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> GameBuilder {
        GameBuilder::new()
    }

    fn init(&mut self) {
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

    pub fn run(&mut self) {
        self.init();
        // print positions
        println!(
            "Player: x = {}, y = {}",
            self.player.position().x,
            self.player.position().y
        );
        println!(
            "Collectible: x = {}, y = {}",
            self.collectible.position().x,
            self.collectible.position().y
        );
        for enemy in &self.enemies {
            println!(
                "Enemy: x = {}, y = {}",
                enemy.position().x,
                enemy.position().y
            );
        }
        for wall in &self.walls {
            println!("Wall: x = {}, y = {}", wall.position().x, wall.position().y);
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::builder().build()
    }
}

