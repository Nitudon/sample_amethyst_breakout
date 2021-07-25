use amethyst::{
    core::Transform,
    core::ecs::{Join, System, WriteExpect, WriteStorage, ReadStorage},
};

use crate::resource::rule::Rule;
use crate::component::{
    ball::Ball,
    bar::Bar,
};
use amethyst::core::math::clamp;

pub struct TranslationSystem;

impl<'a> System<'a> for TranslationSystem {
    type SystemData = (
        // ゲーム内の情報の参照
        WriteExpect<'a, Rule>,
        // 各要素の座標の参照
        WriteStorage<'a, Transform>,
        // ボールの参照
        ReadStorage<'a, Ball>,
        // バーの参照
        ReadStorage<'a, Bar>,
    );

    fn run(&mut self, (mut rule, mut transforms, balls, bars): Self::SystemData) {
        for (ball_transform, ball) in (&mut transforms, &balls).join() {
            // ボールの移動
            ball_transform.set_translation_x(ball_transform.translation().x + ball.speed.x);
            ball_transform.set_translation_y(ball_transform.translation().y + ball.speed.y);

            if ball_transform.translation().y < 0. {
                rule.set_is_game(false);
            }
        }

        for (bar_transform, bar) in (&mut transforms, &bars).join() {
            // バーの移動
            let position_x = clamp(bar_transform.translation().x + bar.speed, bar.size.x * 0.5, 480.0 - bar.size.x * 0.5);
            bar_transform.set_translation_x(position_x);
        }
    }
}