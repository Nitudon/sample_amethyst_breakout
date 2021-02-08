use amethyst::{
    core::Transform,
    core::math::Vector2,
    core::ecs::{Join, System, WriteStorage, ReadStorage},
};

use crate::component::{
    ball::Ball,
    bar::Bar,
};
use amethyst::core::math::clamp;

pub const PLAYER_POSITION_X_MIN: f32 = 40.0;
pub const PLAYER_POSITION_X_MAX: f32 = 920.0;

pub struct TranslationSystem;

impl<'a> System<'a> for TranslationSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Bar>,
    );

    fn run(&mut self, (mut transforms, balls, bars): Self::SystemData) {
        for (ball_transform, ball) in (&mut transforms, &balls).join() {
            // ボールの移動
            ball_transform.set_translation_x(ball_transform.translation().x + ball.speed.x);
            ball_transform.set_translation_y(ball_transform.translation().y + ball.speed.y);
        }

        for (bar_transform, bar) in (&mut transforms, &bars).join() {
            // バーの移動
            let position_x = clamp(bar_transform.translation().x + bar.speed, PLAYER_POSITION_X_MIN, PLAYER_POSITION_X_MAX);
            bar_transform.set_translation_x(position_x);
        }
    }
}