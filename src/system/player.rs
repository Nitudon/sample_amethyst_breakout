use amethyst::{
    core::ecs::{System, WriteStorage, ReadExpect},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::component::bar::*;
use crate::resource::rule::Rule;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        // バーの参照
        WriteStorage<'a, Bar>,
        // キーボードの入力の参照
        ReadExpect<'a, InputHandler<StringBindings>>,
        // ゲーム中かどうかの参照
        ReadExpect<'a, Rule>,
    );

    fn run(&mut self, (mut bars, input, rule): Self::SystemData) {
        if rule.is_game == false {
            return
        }

        for bar in bars.as_mut_slice() {
            if input.key_is_down(VirtualKeyCode::Right) {
                bar.set_speed(MoveDirection::Right);
            }
            else if input.key_is_down(VirtualKeyCode::Left) {
                bar.set_speed(MoveDirection::Left)
            }
            else {
                bar.set_speed(MoveDirection::None);
            }
        }
    }
}