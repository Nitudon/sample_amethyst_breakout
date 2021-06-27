use amethyst::{
    SimpleState, 
    StateData, 
    GameData,
    SimpleTrans,
    Trans,
    StateEvent,
    input::*,
    renderer::rendy::wsi::winit::MouseButton,
};
use crate::component::{
    ball::*,
    block::*,
    bar::*,
    camera::*
};
use crate::state::game::GameState;
use crate::resource::rule::*;

#[derive(Default)]
pub struct StartState;

impl SimpleState for StartState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        create_camera(world);
        create_ball(world);
        create_block_list(world);
        create_bar(world);
        create_rule(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_mouse_button_down(&event, MouseButton::Left) {
                return Trans::Switch(Box::new(GameState::default()))
            }
        }
        Trans::None
    }
}