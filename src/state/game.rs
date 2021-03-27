use amethyst::{
    SimpleState, 
    StateData, 
    GameData, 
    core::ecs::{WorldExt, Entity},
    SimpleTrans, 
    Trans,
    ui::{UiText, UiCreator, UiFinder},
};

use crate::resource::score::*;
use crate::state::result::ResultState;
use crate::component::ball::Ball;
use crate::component::bar::{Bar, MoveDirection};
use crate::component::block::get_start_block_count;

#[derive(Default)]
pub struct GameState {
    pub score_text : Option<Entity>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/game/screen.ron", ());
        });

        let mut score = world.fetch_mut::<Score>(); {
            score.set_is_game(true);
            score.add_block_count(get_start_block_count()); 
        }
        
        let mut balls = world.write_component::<Ball>();{
            for ball in balls.as_mut_slice() {
                ball.initialize_speed();
            } 
        }
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        world.exec(|finder: UiFinder<'_>| {
            if let Some(entity) = finder.find("score_text") {
                self.score_text = Some(entity)
            }
        });
        
        let mut ui_text_storage = world.write_storage::<UiText>(); {
            if let Some(score_text) = self.score_text.and_then(|entity| ui_text_storage.get_mut(entity)) {
                score_text.text = world.fetch::<Score>().score.to_string();
            }
        }
        if world.fetch::<Score>().is_game == false {
            let mut bar_storage = world.write_component::<Bar>();{
                for bar in bar_storage.as_mut_slice() {
                    bar.set_speed(MoveDirection::None);
                } 
            }
            return Trans::Switch(Box::new(ResultState::default()))
        }

        Trans::None
    }
}