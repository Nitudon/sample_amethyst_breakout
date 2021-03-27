use amethyst::{
    core::Transform,
    core::math::Vector2,
    core::ecs::{Join, System, WriteStorage, WriteExpect, ReadStorage, ReadExpect},
    input::{InputHandler, StringBindings, VirtualKeyCode},
};

use crate::component::bar::*;
use crate::resource::score::Score;

pub struct PlayerSystem;

pub const PLAYER_POSITION_X_MIN: f32 = 40.0;
pub const PLAYER_POSITION_X_MAX: f32 = 920.0;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Bar>,
        ReadExpect<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, Score>,
    );

    fn run(&mut self, (mut bars, input, score): Self::SystemData) {
        if score.is_game == false {
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
