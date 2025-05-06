pub use crate::{
    unit::Collectible,
    unit::Enemy,
    unit::Wall,
    unit::{Player, PlayerBuilder},
};

pub struct Game {
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
}

pub struct GameBuilder {
    player: PlayerBuilder,
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
}

impl GameBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            player: PlayerBuilder::new(),
            enemies: vec![
                Enemy::with_speed(0.6),
                Enemy::with_speed(0.5),
                Enemy::with_speed(0.4),
            ],
            walls: vec![],
        }
    }

    pub fn player_starting_health(mut self, health: u8) -> Self {
        self.player = self.player.health(health);
        self
    }

    pub fn player_starting_speed(mut self, speed: f64) -> Self {
        self.player = self.player.speed(speed);
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
            enemies: self.enemies,
            walls: self.walls,
            collectible: Collectible::default(),
            player: self.player.build(),
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
        println!("Game initialized");
        println!("Player speed: {}", self.player.speed());
        println!("Player health: {}", self.player.health());
        println!("Player is alive: {}", self.player.is_alive());
        println!("Player takes {} damage... ", self.player.health());
        self.player.take_damage(self.player.health());
        println!("Player is alive: {}", self.player.is_alive());
    }

    pub fn run(&mut self) {
        self.init();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

