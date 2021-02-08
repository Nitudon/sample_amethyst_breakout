use amethyst::{
    Result,
    core::bundle::{SystemBundle},
    ecs::DispatcherBuilder,
    prelude::World
};
use crate::system::{
    translation::TranslationSystem,
    collision::CollisionSystem,
    player::PlayerSystem
};

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {

        builder.add(TranslationSystem, "translation_system", &[]);
        builder.add(CollisionSystem, "collision_system", &[]);
        builder.add(PlayerSystem, "player_system", &[]);
        Ok(())
    }
}