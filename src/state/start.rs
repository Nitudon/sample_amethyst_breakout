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

        // カメラの初期化
        create_camera(world);

        // ブロックの配列の初期化
        create_block_list(world);

        // ボールの初期化
        create_ball(world);

        // バーの初期化
        create_bar(world);

        // ルールの初期化
        create_rule(world);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            // マウスの左クリックのイベントを取得
            if is_mouse_button_down(&event, MouseButton::Left) {
                // GameStateへの遷移、遷移先をreturnする
                return Trans::Switch(Box::new(GameState::default()))
            }
        }
        Trans::None
    }
}