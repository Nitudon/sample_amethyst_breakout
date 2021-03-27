use amethyst::{
    core::Transform,
    core::math::Vector2,
    core::ecs::{Join, System, WriteStorage, WriteExpect, ReadStorage, Entities, Entity},
};

use crate::component::{ball::Ball, bar::Bar, block::Block, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::resource::score::Score;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Bar>,
        ReadStorage<'a, Block>,
        Entities<'a>,
    );

    fn run(&mut self, (mut score, mut balls, transforms, bars, blocks, entities): Self::SystemData) {
        for (ball_transform, mut ball) in (&transforms, &mut balls).join() {
            // 壁の当たり判定
            if ball_transform.translation().x - ball.size.x * 0.5 <= 0. || ball_transform.translation().x + ball.size.x * 0.5 >= SCREEN_WIDTH{
                ball.reverse_speed_x();
            }
            if ball_transform.translation().y + ball.size.y * 0.5 >= SCREEN_HEIGHT{
                ball.reverse_speed_y();
            }
            
            // バーの当たり判定
            for (bar_transform, bar) in (&transforms, &bars).join() {
                if is_range(ball_transform.translation().y - ball.size.y * 0.5, bar_transform.translation().y, bar.size.y)
                    && is_range(ball_transform.translation().x, bar_transform.translation().x, bar.size.x){
                    ball.reverse_speed_y();
                    let x_distance = ball_transform.translation().x - bar_transform.translation().x;
                    let x_reflect_value = x_distance / bar.size.x * 0.5;
                    ball.set_speed_x_by_base_speed_coefficient(x_reflect_value);
                }
            }
            
            // ブロックの当たり判定
            for (block_transform, block, entity) in (&transforms, &blocks, &*entities).join() {
                let ball_position = Vector2::new(ball_transform.translation().x, ball_transform.translation().y);
                let block_position = Vector2::new(block_transform.translation().x, block_transform.translation().y);
                let hit_block = get_ball_reflection(ball, &ball_position, &block_position, &block.size);
                match hit_block {
                    BallReflectDirection::X => ball.reverse_speed_x(),
                    BallReflectDirection::Y => ball.reverse_speed_y(),
                    _ => continue
                }

                score.add_score(block.score);
                score.subtract_block_count(1);
                entities.delete(entity);
                if score.block_count == 0 {
                    // クリア
                    score.set_is_game(false);
                }
            }
        }
    }
}

enum BallReflectDirection {
    None,
    X,
    Y
}

fn get_ball_reflection(ball: &Ball, ball_position: &Vector2<f32>, target_position: &Vector2<f32>, target_size: &Vector2<f32>) -> BallReflectDirection {
    let ball_size = &ball.size;

    if is_range(ball_position.y, target_position.y, target_size.y) &&
        (is_range(ball_position.x - ball_size.x * 0.5, target_position.x, target_size.x) || is_range(ball_position.x + ball_size.x / 2.0, target_position.x, target_size.x)) {
        return BallReflectDirection::X;
    }
    
    if is_range(ball_position.x, target_position.x, target_size.x) && 
        (is_range(ball_position.y - ball_size.y * 0.5, target_position.y, target_size.y) || is_range(ball_position.y + ball_size.y / 2.0, target_position.y, target_size.y)) {
        return BallReflectDirection::Y;
    }

    BallReflectDirection::None
}

fn is_range(value: f32, center: f32, length: f32) -> bool {
    center - length * 0.5 < value && value < center + length * 0.5
} 
