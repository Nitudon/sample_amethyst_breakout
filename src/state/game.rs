use amethyst::{
    SimpleState, 
    StateData, 
    GameData, 
    core::ecs::WorldExt,
    SimpleTrans, 
    Trans,
};

use crate::resource::rule::*;
use crate::state::result::ResultState;
use crate::component::ball::Ball;
use crate::component::bar::{Bar, MoveDirection};
use crate::component::block::Block;
use amethyst::shred::World;
use amethyst::core::Transform;
use amethyst::ui::UiFinder;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let mut rule = world.fetch_mut::<Rule>(); {
            rule.set_is_game(true);
        }
        
        let mut balls = world.write_component::<Ball>();{
            for ball in balls.as_mut_slice() {
                ball.initialize_speed();
            } 
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if world.read_component::<Block>().is_empty() {
            world.fetch_mut::<Rule>().set_is_game(false);
        }
        
        if world.fetch::<Rule>().is_game == false {
            let mut bar_storage = world.write_component::<Bar>();{
                for bar in bar_storage.as_mut_slice() {
                    bar.set_speed(MoveDirection::None);
                } 
            }

            let mut ball_storage = world.write_component::<Ball>();{
                for ball in ball_storage.as_mut_slice() {
                    ball.stop();
                }
            }
            
            return Trans::Switch(Box::new(ResultState::default()))
        }

        Trans::None
    }
}