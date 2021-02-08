use amethyst::ecs::prelude::*;
use amethyst::core::math::Id;
use amethyst::ui::{UiLabel, UiText};

#[derive(Default)]
pub struct Score {
    pub score : i32,
    pub time : f32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score : 0,
            time : 0.,
        }
    }
    
    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn subtract_score(&mut self, score: i32) {
        self.score -= score;
    }

    pub fn add_time(&mut self, time: f32) {
        self.time += time;
    }

    pub fn subtract_time(&mut self, time: f32) {
        self.time -= time;
    }
}

pub fn create_score(world: &mut World){
    let mut score = Score::new();
    world.insert(score);
}