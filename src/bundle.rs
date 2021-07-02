use amethyst::{
    Result,
    core::bundle::SystemBundle,
    core::ecs::WorldExt,
    ecs::DispatcherBuilder,
    prelude::World
};
use crate::component::{
    block::Block,
    ball::Ball,
    bar::Bar
};
// この後Systemを実装した後に使う
use crate::system::{
    translation::TranslationSystem,
    player::PlayerSystem,
    collision::CollisionSystem
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        _world.register::<Block>();
        _world.register::<Ball>();
        _world.register::<Bar>();
        // 移動を処理するTranslationSystem
        builder.add(TranslationSystem, "translation_system", &[]);
        // ユーザー入力と対応するバー操作を処理するPlayerSystem
        builder.add(PlayerSystem, "player_system", &[]);
        // 衝突を処理するCollisionSystem
        builder.add(CollisionSystem, "collision_system", &[]);
        Ok(())
    }
}