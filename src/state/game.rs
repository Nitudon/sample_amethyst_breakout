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
use crate::component::ball::*;
use crate::component::block::*;
use crate::resource::score::*;
use crate::component::wall::*;
use crate::component::bar::*;

pub const BLOCK_COUNT_X : i32 = 4;
pub const BLOCK_COUNT_Y : i32 = 5;
pub const BLOCK_START_X : f32 = 0.0;
pub const BLOCK_START_Y : f32 = 0.0;
pub const BLOCK_MARGIN_X : f32 = 144.0;
pub const BLOCK_MARGIN_Y : f32 = 84.0;
pub const BLOCK_SCORE_UNIT : i32 = 100;

pub const STAGE_LEFT_X : f32 = -240.0;
pub const STAGE_CENTER_X : f32 = 0.0;
pub const STAGE_RIGHT_X : f32 = 240.0;
pub const STAGE_SIDE_Y : f32 = 180.0;
pub const STAGE_CENTER_Y : f32 = 540.0;
pub const STAGE_SIDE_WIDTH : f32 = 80.0;
pub const STAGE_SIDE_HEIGHT : f32 = 480.0;
pub const STAGE_CENTER_WIDTH : f32 = 320.0;
pub const STAGE_CENTER_HEIGHT : f32 = 80.0;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        let mut score = Score::new();
        world.insert(score);
        
        create_ball(world);
        for x in 0..BLOCK_COUNT_X {
            for y in 0..BLOCK_COUNT_Y {
                let position_x = BLOCK_START_X + (x as f32) * BLOCK_MARGIN_X;
                let position_y = BLOCK_START_Y + (y as f32) * BLOCK_MARGIN_Y;
                let score = BLOCK_SCORE_UNIT * (y + 1);
                create_block(position_x, position_y, score, world);
            }
        }
        create_bar(world);
        create_wall(STAGE_LEFT_X, STAGE_SIDE_Y, STAGE_SIDE_WIDTH, STAGE_SIDE_HEIGHT, world);
        create_wall(STAGE_RIGHT_X, STAGE_SIDE_Y, STAGE_SIDE_WIDTH, STAGE_SIDE_HEIGHT, world);
        create_wall(STAGE_CENTER_X, STAGE_CENTER_Y, STAGE_CENTER_WIDTH, STAGE_CENTER_HEIGHT, world);
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;
/*
        if world.fetch::<Score>().is_dead {
            return Trans::Switch(Box::new(GameEnd::default()))
        }

        if self.score_text.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("score_text") {
                    self.score_text = Some(entity);
                }
            });
        }

        let mut ui_text = world.write_storage::<UiText>();
        {
            let score = world.try_fetch::<Score>()?;
            if let Some(score_text) = self.score_text.and_then(|entity| ui_text.get_mut(entity)) {
                score_text.text = score.score.to_string();
            }
            if let Some(time_text) = self.time_text.and_then(|entity| ui_text.get_mut(entity)) {
                time_text.text = score.time.to_string();
            }
        }
*/
        Trans::None
    }
}