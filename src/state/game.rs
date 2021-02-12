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

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;
    }

    /*
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

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
        Trans::None
    }
    */
}