use amethyst::{
    SimpleState, 
    StateData, 
    GameData, 
    core::ecs::{Entity, WorldExt},
    core::math::Vector2,
    ui::{UiCreator, UiText, UiFinder}, 
    SimpleTrans, 
    Trans
};

use crate::resource::score::*;
use crate::state::start::StartState;
use crate::state::result::ResultState;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
        world.fetch_mut::<Score>().set_is_game(true);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if world.fetch::<Score>().is_game == false {
            return Trans::Switch(Box::new(ResultState::default()))
        }

        Trans::None
    }
}