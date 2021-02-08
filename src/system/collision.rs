use amethyst::{
    core::Transform,
    core::math::Vector2,
    core::ecs::{Join, System, WriteStorage, WriteExpect, ReadStorage, Entities},
};

use crate::component::{
    ball::Ball,
    bar::Bar,
    block::Block,
    wall::Wall
};
use crate::resource::score::Score;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        WriteExpect<'a, Score>,
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Bar>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Wall>,
        Entities<'a>,
    );

    fn run(&mut self, (mut score, mut balls, transforms, bars, blocks, walls, entities): Self::SystemData) {
        for (ball_transform, mut ball) in (&transforms, &mut balls).join() {
            let ball_position = Vector2::new(ball_transform.translation().x, ball_transform.translation().y);

            // 壁の当たり判定
            for (wall_transform, wall) in (&transforms, &walls).join() {
                let wall_position = Vector2::new(wall_transform.translation().x, wall_transform.translation().y);
                let hit_wall = get_ball_stage_block_reflection(ball, &ball_position, &wall_position, &wall.size);
                match hit_wall {
                    BallReflectDirection::X => ball.reverse_speed_x(),
                    BallReflectDirection::Y => ball.reverse_speed_y(),
                    _ => {}
                }
            }

            // ブロックの当たり判定
            for (block_transform, block, entity) in (&transforms, &blocks, &*entities).join() {
                let block_position = Vector2::new(block_transform.translation().x, block_transform.translation().y);
                let hit_block = get_ball_stage_block_reflection(ball, &ball_position, &block_position, &block.size);
                match hit_block {
                    BallReflectDirection::X => ball.reverse_speed_x(),
                    BallReflectDirection::Y => ball.reverse_speed_y(),
                    _ => continue
                }

                score.add_score(block.score);
                entities.delete(entity);
            }

            // バーの当たり判定
            for (bar_transform, bar) in (&transforms, &bars).join() {
                if is_range(ball_position.y - ball.size.y / 2.0, bar_transform.translation().y, bar.size.y) {
                    ball.reverse_speed_y();
                    
                }
                let x_distance = ball_position.x - bar_transform.translation().x;
                let x_reflect_value = x_distance / bar.size.x / 2.0;
                ball.set_speed_x_by_base_speed_coefficient(x_reflect_value);
            }
        }
    }
}

enum BallReflectDirection {
    None,
    X,
    Y
}

fn get_ball_stage_block_reflection(ball: &Ball, ball_position: &Vector2<f32>, target_position: &Vector2<f32>, target_size: &Vector2<f32>) -> BallReflectDirection {
    let ball_size = &ball.size;

    if is_range(ball_position.y, target_position.y, target_size.y) &&
        (is_range(ball_position.x - ball_size.x / 2.0, target_position.x, target_size.x) || is_range(ball_position.x + ball_size.x / 2.0, target_position.x, target_size.x)) {
        return BallReflectDirection::X;
    }
    
    if is_range(ball_position.x, target_position.x, target_size.x) && 
        (is_range(ball_position.y - ball_size.y / 2.0, target_position.y, target_size.y) || is_range(ball_position.y + ball_size.y / 2.0, target_position.y, target_size.y)) {
        return BallReflectDirection::Y;
    }

    BallReflectDirection::None
}

fn is_range(value: f32, center: f32, length: f32) -> bool {
    center - length / 2.0 < value && value < center + length / 2.0
} 
