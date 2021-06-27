use amethyst::{
    core::Transform,
    core::math::Vector2,
    core::ecs::{Join, System, WriteStorage, ReadStorage, ReadExpect, Entities},
};

use crate::component::{
    ball::*, 
    bar::Bar, 
    block::Block, 
};
use crate::resource::rule::Rule;
use amethyst::core::math::DimSub;
use amethyst::renderer::rendy::wsi::winit::VirtualKeyCode::E;
use amethyst::core::num::abs;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadExpect<'a, Rule>,
        WriteStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Bar>,
        ReadStorage<'a, Block>,
        Entities<'a>,
    );

    fn run(&mut self, (rule, mut balls, transforms, bars, blocks, entities): Self::SystemData) {
        // ゲームが終了していたら何もしない
        if rule.is_game == false {
            return;
        }

        for (ball_transform, ball) in (&transforms, &mut balls).join() {
            let ball_position = Vector2::new(ball_transform.translation().x, ball_transform.translation().y);
            // 壁の当たり判定、単純に画面の縁を壁に見立ててはみ出そうとしたら速度を反転
            if ball_transform.translation().x - ball.radius * 0.5 <= 0. || ball_transform.translation().x + ball.radius * 0.5 >= 480.0 {
                ball.reverse_speed_x();
            }
            if ball_transform.translation().y + ball.radius * 0.5 >= 600.0 {
                ball.reverse_speed_y();
            }
            
            // バーの当たり判定
            for (bar_transform, bar) in (&transforms, &bars).join() {
                // バーの上部の辺の当たり判定だけチェックする
                let x_half = bar.size.x * 0.5;
                let y_half = bar.size.y * 0.5;
                let left = Vector2::new(bar_transform.translation().x - x_half, bar_transform.translation().y + y_half);
                let right = Vector2::new(bar_transform.translation().x + x_half, bar_transform.translation().y + y_half);
                if is_hit_circle_with_edge( &ball_position, ball.radius, &left, &right){
                    // Y軸の速度反転はしつつ、X軸方向の速度は当たった場所によって調整する
                    ball.reverse_speed_y();
                    let x_distance = ball_transform.translation().x - bar_transform.translation().x;
                    let x_reflect_value = x_distance / bar.size.x * 0.5;
                    ball.set_speed_x(BALL_BASE_SPEED_X * x_reflect_value);
                }
            }
            
            // ブロックの当たり判定
            for (block_transform, block, entity) in (&transforms, &blocks, &*entities).join() {
                let ball_position = Vector2::new(ball_transform.translation().x, ball_transform.translation().y);
                let block_position = Vector2::new(block_transform.translation().x, block_transform.translation().y);
                // ボールとブロックから衝突を検証
                let hit_block = get_ball_reflection_with_block(&ball_position, ball.radius,  &block_position, &block.size);
                // 衝突状態でパターンマッチング
                match hit_block {
                    BallReflectDirection::X => ball.reverse_speed_x(),
                    BallReflectDirection::Y => ball.reverse_speed_y(),
                    // 衝突してない場合は抜ける
                    _ => continue
                }

                // 衝突したのでブロックを消してオブジェクトを削除
                match entities.delete(entity) {
                    Ok(()) => {},
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

// ボールの位置と半径からブロックの四辺を見て当たり判定を処理する。
fn get_ball_reflection_with_block(ball_position: &Vector2<f32>, ball_radius: f32, target_position: &Vector2<f32>, target_size: &Vector2<f32>) -> BallReflectDirection {
    // 長方形の四辺    
    let target_left_up = Vector2::new(target_position.x - target_size.x * 0.5, target_position.y + target_size.y * 0.5);
    let target_left_down = Vector2::new(target_position.x - target_size.x * 0.5, target_position.y - target_size.y * 0.5);
    let target_right_up = Vector2::new(target_position.x + target_size.x * 0.5, target_position.y + target_size.y * 0.5);
    let target_right_down = Vector2::new(target_position.x + target_size.x * 0.5, target_position.y - target_size.y * 0.5);

    // 横向きの衝突判定
    if is_hit_circle_with_edge(ball_position, ball_radius, &target_left_down, &target_left_up) || is_hit_circle_with_edge(ball_position, ball_radius, &target_right_down, &target_right_up){
        return BallReflectDirection::X;
    }

    // 縦向きの衝突判定
    if is_hit_circle_with_edge(ball_position, ball_radius, &target_left_up, &target_right_up) || is_hit_circle_with_edge(ball_position, ball_radius, &target_left_down, &target_right_down){
        return BallReflectDirection::Y;
    }

    BallReflectDirection::None
}

// 線分と円の当たり判定
fn is_hit_circle_with_edge(center: &Vector2<f32>, radius: f32, edge_start: &Vector2<f32>, edge_end: &Vector2<f32>) -> bool {
    // 縁の中心と線分の端点についてのベクトル
    let edge = Vector2::new(edge_end.x - edge_start.x, edge_end.y - edge_start.y);
    let start_center = Vector2::new(center.x - edge_start.x, center.y - edge_start.y);
    let end_center = Vector2::new(center.x - edge_end.x, center.y - edge_end.y);

    // 円について始点と終点の内積とその積を見て場合わけ
    let dot_start = edge.dot(&start_center);
    let dot_end = edge.dot(&end_center);
    // 内積の積が負、つまり線分の内側に円がいる
    if dot_start * dot_end <= 0. {
        // 二次元ベクトルでの外積のスカラー値から、円の中心から線分に下ろした垂線の長さを見る
        let distance = abs( edge.x * start_center.y - edge.y * start_center.x) / edge.magnitude();
        // 半径より短いなら円の内側、つまり当たっている
        return distance <= radius;
    } else {
        // 線分の外側に円があるなら端点と円の中心の距離が半径より短ければ当たっている
        return start_center.magnitude() <= radius || end_center.magnitude() <= radius;
    }
}
