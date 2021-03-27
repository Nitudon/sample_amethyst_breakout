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

pub const BLOCK_COUNT_X : i32 = 4;
pub const BLOCK_COUNT_Y : i32 = 5;
pub const BLOCK_START_X : f32 = 120.0;
pub const BLOCK_START_Y : f32 = 180.0;
pub const BLOCK_MARGIN_X : f32 = 144.0;
pub const BLOCK_MARGIN_Y : f32 = 84.0;

#[derive(Default)]
pub struct StartState {
    pub title_ui : Option<Entity>
}

impl SimpleState for StartState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        let score = Score::new();
        world.insert(score);
        
        create_camera(world);
        create_ball(world);
        create_block_list(world);
        create_bar(world);

        world.exec(|mut creator: UiCreator<'_>| {
            self.title_ui = Some(creator.create("ui/title/screen.ron", ()));
        });
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_mouse_button_down(&event, MouseButton::Left) {
                if self.title_ui.is_some() {
                    match world.entities().delete(self.title_ui.unwrap()) {
                        Ok(()) => self.title_ui = None,
                        Err(err) => println!("delete wrong generation {:?}", err)
                    }
                }
                return Trans::Switch(Box::new(GameState::default()))
            }
        }
        Trans::None
    }
}