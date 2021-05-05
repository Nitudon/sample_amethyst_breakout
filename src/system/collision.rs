use amethyst::{
    core::Transform,
    core::math::Vector2,
    core::ecs::{Join, System, WriteStorage, WriteExpect, ReadStorage, Entities},
};

use crate::component::{ball::Ball, bar::Bar, block::Block, AREA_WIDTH, AREA_HEIGHT};
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
        // ゲームが終了していたら何もしない
        if score.is_game == false {
            return;
        }

        for (ball_transform, ball) in (&transforms, &mut balls).join() {
            // 壁の当たり判定、単純に画面の縁を壁に見立ててはみ出そうとしたら速度を反転
            if ball_transform.translation().x - ball.size.x * 0.5 <= 0. || ball_transform.translation().x + ball.size.x * 0.5 >= AREA_WIDTH{
                ball.reverse_speed_x();
            }
            if ball_transform.translation().y + ball.size.y * 0.5 >= AREA_HEIGHT{
                ball.reverse_speed_y();
            }
            
            // バーの当たり判定
            for (bar_transform, bar) in (&transforms, &bars).join() {
                // バーの上部の辺の当たり判定だけチェックする
                if is_range_edge(ball_transform.translation().y - ball.size.y * 0.5, bar_transform.translation().y, bar.size.y)
                    && is_range_edge(ball_transform.translation().x, bar_transform.translation().x, bar.size.x){
                    // Y軸の速度反転はしつつ、X軸方向の速度は当たった場所によって調整する
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
                // ボールとブロックから衝突を検証
                let hit_block = get_ball_reflection(ball, &ball_position, &block_position, &block.size);
                // 衝突状態でパターンマッチング
                match hit_block {
                    BallReflectDirection::X => ball.reverse_speed_x(),
                    BallReflectDirection::Y => ball.reverse_speed_y(),
                    // 衝突してない場合は抜ける
                    _ => continue
                }

                // 衝突したのでブロックを消してオブジェクトを削除
                match entities.delete(entity) {
                    Ok(()) => {
                        // スコアの加点
                        score.add_score(block.score);
                        // ブロックの数を減らす
                        score.subtract_block_count(1);
                        // ブロックの数がゼロになったらクリア
                        if score.block_count == 0 {
                            score.set_is_game(false);
                        }
                    },
                    Err(err) => println!("delete wrong generation {:?}", err)
                }
            }
        }
    }
}

// ボールの跳ね返りの処理、X軸での反射かY軸の反射
enum BallReflectDirection {
    // 反射しない
    None,
    // X軸での反射
    X,
    // Y軸での反射
    Y
}

// ボールの位置からボールの半径の矩形で当たり判定を処理する。対象も矩形
fn get_ball_reflection(ball: &Ball, ball_position: &Vector2<f32>, target_position: &Vector2<f32>, target_size: &Vector2<f32>) -> BallReflectDirection {
    // ボールの四辺
    let ball_size = &ball.size;
    let ball_position_left = ball_position.x - ball_size.x * 0.5;
    let ball_position_right = ball_position.x + ball_size.x * 0.5;
    let ball_position_down = ball_position.y - ball_size.x * 0.5;
    let ball_position_up = ball_position.y + ball_size.y * 0.5;

    // 横向きの衝突判定
    if is_range_edge(ball_position.y, target_position.y, target_size.y) &&
        (is_range_edge(ball_position_left, target_position.x, target_size.x) || is_range_edge(ball_position_right, target_position.x, target_size.x)) {
        return BallReflectDirection::X;
    }
    
    // 縦向きの衝突判定
    if is_range_edge(ball_position.x, target_position.x, target_size.x) && 
        (is_range_edge(ball_position_down, target_position.y, target_size.y) || is_range_edge(ball_position_up, target_position.y, target_size.y)) {
        return BallReflectDirection::Y;
    }

    BallReflectDirection::None
}

// シンプルに中心と長さから成る直線の内外判定
fn is_range_edge(value: f32, center: f32, length: f32) -> bool {
    center - length * 0.5 < value && value < center + length * 0.5
} 
