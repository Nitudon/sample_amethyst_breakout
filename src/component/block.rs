use amethyst::{
    core::Transform,
    core::ecs::{DenseVecStorage, Component},
    core::math::Vector2,
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::sprite::create_sprite;

const BLOCK_HEIGHT: f32 = 32.0;
const BLOCK_WIDTH: f32 = 96.0;
const BLOCK_COUNT_X : i32 = 4;
const BLOCK_COUNT_Y : i32 = 4;
const BLOCK_START_X : f32 = 18.0;
const BLOCK_START_Y : f32 = 350.0;
const BLOCK_MARGIN_X : f32 = 20.0;
const BLOCK_MARGIN_Y : f32 = 20.0;

pub enum BlockType
{
    Green,
    Orange,
    Red,
}

#[derive(Clone)]
pub struct Block {
    pub size: Vector2<f32>,
}

impl Block {
    fn new(size: Vector2<f32>) -> Block {
        Block {
            size,
        }
    }
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_block_list(world: &mut World) {
    for x in 0..BLOCK_COUNT_X {
        for y in 0..BLOCK_COUNT_Y {
            let position_x = BLOCK_WIDTH * 0.5 + BLOCK_START_X + (x as f32) * (BLOCK_WIDTH + BLOCK_MARGIN_X);
            let position_y = BLOCK_START_Y + (y as f32) * (BLOCK_HEIGHT + BLOCK_MARGIN_Y);
            let block_type = &match y {
                0 => BlockType::Green,
                1 => BlockType::Orange,
                2 => BlockType::Red,
                _ => BlockType::Green,
            };
            create_block((position_x, position_y), block_type, world);
        }
    }
}

fn create_block(position: (f32, f32), block_type: &BlockType, world: &mut World) {
    let size = Vector2::new(BLOCK_WIDTH, BLOCK_HEIGHT);
    let block = Block::new(size);
    let sprite = create_block_sprite(block_type, world);
    let mut transform = Transform::default();
    transform.set_translation_xyz(position.0, position.1, 0.0);
    
    world
        .create_entity()
        .with(block)
        .with(sprite)
        .with(transform)
        .build();
}

fn create_block_sprite(block_type: &BlockType, world: &mut World) -> SpriteRender {
    let index = match block_type {
        BlockType::Green => 0,
        BlockType::Orange => 1,
        BlockType::Red => 2,
    };
    create_sprite("texture/block.png", "texture/block_spritesheet.ron", index, world)
}