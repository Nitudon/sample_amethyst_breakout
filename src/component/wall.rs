use amethyst::{
    core::Transform,
    core::ecs::{DenseVecStorage, Component},
    core::math::Vector2,
    prelude::*,
    renderer::SpriteRender,
};
use crate::util::sprite::*;

#[derive(Clone)]
pub struct Wall {
    pub size: Vector2<f32>,
}

impl Wall {
    pub fn new(size: Vector2<f32>) -> Wall {
        Wall {
            size
        }
    }
}

impl Component for Wall {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_wall(x: f32, y: f32, width: f32, height: f32, world: &mut World) {
    let size = Vector2::new(width, height);
    let mut stage = Wall::new(size);
    let mut sprite = create_wall_sprite(world);
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 1.0);

    world
        .create_entity()
        .with(stage)
        .with(sprite)
        .with(transform)
        .build();
}

fn create_wall_sprite(world: &mut World) -> SpriteRender {
    create_sprite("texture/stage_spritesheet.png", "texture/stage_spritesheet.ron", 0, world)
}