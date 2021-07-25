use amethyst::{
    SimpleState,
    StateData,
    GameData,
    SimpleTrans,
    Trans,
    StateEvent,
    core::ecs::WorldExt,
    input::*,
    renderer::rendy::wsi::winit::MouseButton,
};
use crate::state::start::StartState;

#[derive(Default)]
pub struct ResultState;

impl SimpleState for ResultState {
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.delete_all();
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            // 左クリックしたらStartStateへ遷移
            if is_mouse_button_down(&event, MouseButton::Left) {
                return Trans::Switch(Box::new(StartState::default()))
            }
        }
        Trans::None
    }
}