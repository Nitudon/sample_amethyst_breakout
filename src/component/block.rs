use amethyst::{
    core::Transform,
    core::ecs::{DenseVecStorage, Component},
    core::math::Vector2,
    prelude::*,
    renderer::SpriteRender,
};

use crate::util::sprite::*;

const BLOCK_HEIGHT: f32 = 60.0;
const BLOCK_WIDTH: f32 = 120.0;

enum BlockType
{
    Green,
    Orange,
    Red,
}

#[derive(Clone)]
pub struct Block {
    pub size: Vector2<f32>,
    pub score: i32,
}

impl Block {
    fn new(size: Vector2<f32>, score : i32) -> Block {
        Block {
            size,
            score,
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_block(x: f32, y: f32, block_type: &BlockType, world: &mut World) {
    let size = Vector2::new(BLOCK_WIDTH, BLOCK_HEIGHT);
    let score = match block_type {
        Green => 100,
        Orange => 200, 
        Red => 300
    };
    let mut block = Block::new(size, score);
    let mut sprite = create_block_sprite(block_type, world);
    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 1.0);

    world
        .create_entity()
        .with(block)
        .with(sprite)
        .with(transform)
        .build();
}

fn create_block_sprite(block_type: &BlockType, world: &mut World) -> SpriteRender {
    create_sprite("texture/block_spritesheet.png", "texture/block_spritesheet.ron", block_type as usize, world)
}
