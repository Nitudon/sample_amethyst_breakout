use amethyst::{
    SimpleState,
    StateData,
    GameData,
    core::ecs::{WorldExt, Entity},
    SimpleTrans,
    Trans,
};

use crate::state::result::ResultState;
use crate::component::{
    block::Block,
    ball::Ball,
    bar::{Bar, MoveDirection},
};
use crate::resource::rule::*;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // ゲーム中フラグをtrueに
        let mut rule = world.fetch_mut::<Rule>(); {
            rule.set_is_game(true);
        }

        // ボールに初期速度を付与
        let mut balls = world.write_component::<Ball>();{
            for ball in balls.as_mut_slice() {
                ball.initialize_speed();
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        // ブロックがなくなった（クリアした）らゲーム中フラグをfalseに
        if world.read_component::<Block>().is_empty() {
            world.fetch_mut::<Rule>().set_is_game(false);
        }

        // ゲーム中フラグがfalseだったら遷移処理へ
        if world.fetch::<Rule>().is_game == false {
            // バーの動きを止める
            let mut bar_storage = world.write_component::<Bar>();{
                for bar in bar_storage.as_mut_slice() {
                    bar.set_speed(MoveDirection::None);
                }
            }
            // ボールの動きを止める
            let mut ball_storage = world.write_component::<Ball>();{
                for ball in ball_storage.as_mut_slice() {
                    ball.stop();
                }
            }
            // ResultStateへの遷移   
            return Trans::Switch(Box::new(ResultState::default()))
        }

        Trans::None
    }
}