use amethyst::{
    SimpleState,
    StateData,
    GameData,
    SimpleTrans,
    Trans,
    StateEvent,
    core::ecs::{WorldExt, Entity},
    input::*,
    renderer::rendy::wsi::winit::MouseButton,
    ui::UiCreator,
};
use crate::component::{
    ball::*,
    block::*,
    bar::*,
    camera::*,
};
use crate::state::game::GameState;
use crate::resource::score::Score;
use crate::state::start::StartState;

pub const BLOCK_COUNT_X : i32 = 4;
pub const BLOCK_COUNT_Y : i32 = 5;
pub const BLOCK_START_X : f32 = 120.0;
pub const BLOCK_START_Y : f32 = 180.0;
pub const BLOCK_MARGIN_X : f32 = 144.0;
pub const BLOCK_MARGIN_Y : f32 = 84.0;

#[derive(Default)]
pub struct ResultState {
    pub result_ui : Option<Entity>
}

impl SimpleState for ResultState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { mut world, .. } = data;

        let block_count = world.fetch::<Score>().block_count;

        world.exec(|mut creator: UiCreator<'_>| {
            if block_count > 0 {
                self.result_ui = Some(creator.create("ui/result/game_over.ron", ()));
                
            } else {
                self.result_ui = Some(creator.create("ui/result/clear.ron", ()));
            }
        });
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { mut world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_mouse_button_down(&event, MouseButton::Left) {
                if self.result_ui.is_some() {
                    world.entities().delete(self.result_ui.unwrap());
                    self.result_ui = None;
                }
                return Trans::Switch(Box::new(StartState::default()))
            }
        }
        Trans::None
    }
}