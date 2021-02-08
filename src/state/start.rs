use amethyst::{
    SimpleState, 
    StateData, 
    GameData, 
    core::ecs::WorldExt, 
    ui::UiCreator, 
    SimpleTrans, 
    Trans
};
use crate::component::ball::*;
use amethyst::core::ecs::Entity;

pub const BLOCK_COUNT_X : i32 = 4;
pub const BLOCK_COUNT_Y : i32 = 5;
pub const BLOCK_START_X : f32 = 120.0;
pub const BLOCK_START_Y : f32 = 180.0;
pub const BLOCK_MARGIN_X : f32 = 144.0;
pub const BLOCK_MARGIN_Y : f32 = 84.0;

#[derive(Default)]
pub struct GameState {
    pub score_text : Option<Entity>
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}