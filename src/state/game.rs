use amethyst::{
    SimpleState, 
    StateData, 
    GameData, 
    core::ecs::WorldExt,
    SimpleTrans, 
    Trans
};

use crate::resource::score::*;
use crate::state::result::ResultState;
use crate::component::ball::Ball;
use crate::component::bar::{Bar, MoveDirection};
use crate::component::block::get_start_block_count;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        let mut score = world.fetch_mut::<Score>();
        score.set_is_game(true);
        score.add_block_count(get_start_block_count());
        let mut balls = world.write_component::<Ball>();
        for ball in balls.as_mut_slice() {
            ball.initialize_speed();
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if world.fetch::<Score>().is_game == false {
            let mut bar = world.write_component::<Bar>();
            for bar in bar.as_mut_slice() {
                bar.set_speed(MoveDirection::None);
            }
            return Trans::Switch(Box::new(ResultState::default()))
        }

        Trans::None
    }
}