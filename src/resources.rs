use bevy::prelude::*;
use rand::prelude::*;

use crate::constants::{CLICK_DELAY, KNOWN_TYPES};

#[derive(Resource)]
pub struct SpawnTime {
    // prevent spawning in quick succession
    pub timer: Timer,
}

impl Default for SpawnTime {
    fn default() -> Self {
        Self {
            // default to 0 seconds as first click doesn't need buffer
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

impl SpawnTime {
    pub fn start_new_timer(&mut self) {
        self.timer = Timer::from_seconds(CLICK_DELAY, TimerMode::Once);
    }
}

#[derive(Resource)]
pub struct NextGenerator {
    pub current_fruit: Fruit,
    pub next_fruit: Fruit,
}

impl Default for NextGenerator {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let (cur, next) = (rng.gen_range(0..5), rng.gen_range(0..5));
        let (cur_size, image_file_name) = KNOWN_TYPES[cur];
        let (next_size, next_image_file_name) = KNOWN_TYPES[next];
        Self {
            current_fruit: Fruit {
                size: cur_size,
                image_file_name: String::from(image_file_name),
            },
            next_fruit: Fruit {
                size: next_size,
                image_file_name: String::from(next_image_file_name),
            },
        }
    }
}

impl NextGenerator {
    pub fn next(&mut self) {
        let mut rng = rand::thread_rng();
        let next = rng.gen_range(0..5);
        self.current_fruit = self.next_fruit.clone();
        let (size, image_file_name) = KNOWN_TYPES[next];
        self.next_fruit = Fruit {
            size,
            image_file_name: String::from(image_file_name),
        };
    }
}

#[derive(Component, Debug)]
pub struct Fruit {
    pub size: f32,
    pub image_file_name: String,
}

impl Fruit {
    pub fn clone(&self) -> Fruit {
        Fruit {
            size: self.size,
            image_file_name: self.image_file_name.clone(),
        }
    }

    pub fn merge(&self) -> Option<Fruit> {
        if self.size == KNOWN_TYPES[10].0 {
            return None;
        }
        let (size, image_file_name) =
            KNOWN_TYPES[KNOWN_TYPES.iter().position(|s| s.0 == self.size).unwrap() + 1];
        Some(Fruit {
            size,
            image_file_name: String::from(image_file_name),
        })
    }
}
