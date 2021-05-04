use amethyst::{
    core::Transform,
    core::ecs::{Join, System, WriteExpect, WriteStorage, ReadStorage},
};

use crate::resource::score::Score;
use crate::component::{
    SCREEN_WIDTH,
    ball::Ball,
    bar::Bar,
};
use amethyst::core::math::clamp;

pub struct TranslationSystem;

impl<'a> System<'a> for TranslationSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Bar>,
    );

    fn run(&mut self, (mut score, mut transforms, balls, bars): Self::SystemData) {
        
        for (ball_transform, ball) in (&mut transforms, &balls).join() {
            // ボールの移動
            ball_transform.set_translation_x(ball_transform.translation().x + ball.speed.x);
            ball_transform.set_translation_y(ball_transform.translation().y + ball.speed.y);
            
            if ball_transform.translation().y < 0. {
                score.set_is_game(false);
            }
        }

        for (bar_transform, bar) in (&mut transforms, &bars).join() {
            // バーの移動
            let position_x = clamp(bar_transform.translation().x + bar.speed, bar.size.x * 0.5, SCREEN_WIDTH - bar.size.x * 0.5);
            bar_transform.set_translation_x(position_x);
        }
    }
}
