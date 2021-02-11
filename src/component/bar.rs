use amethyst::{
    core::Transform,
    core::ecs::{DenseVecStorage, Component},
    core::math::Vector2,
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::*;
use crate::util::sprite::*;
use amethyst::core::math::Vector3;

const BAR_HEIGHT: f32 = 16.0;
const BAR_WIDTH: f32 = 96.0;
const BAR_SPEED: f32 = 3.0;
const BAR_START_Y: f32 = 60.0;

pub enum MoveDirection {
    None,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Bar {
    pub size: Vector2<f32>,
    pub speed: f32,
}

impl Bar {
    pub fn new(size: Vector2<f32>, speed: f32) -> Bar {
        Bar {
            size,
            speed
        }
    }
    
    pub fn set_speed(&mut self, dir: MoveDirection) {
        self.speed = match dir { 
            MoveDirection::None => 0.0,
            MoveDirection::Left => - BAR_SPEED,
            MoveDirection::Right => BAR_SPEED
        };
    }
}

impl Component for Bar {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_bar(world: &mut World) {
    let size = Vector2::new(BAR_WIDTH, BAR_HEIGHT);
    let mut bar = Bar::new(size, BAR_SPEED);
    let mut sprite = create_bar_sprite(world);
    let mut transform = Transform::default();
    transform.set_translation_xyz(SCREEN_WIDTH * 0.5, BAR_START_Y, 0.0);
    
    world
        .create_entity()
        .with(bar)
        .with(sprite)
        .with(transform)
        .build();
}

fn create_bar_sprite(world: &mut World) -> SpriteRender {
    create_sprite("texture/bar.png", "texture/bar_spritesheet.ron", 0, world)
}
