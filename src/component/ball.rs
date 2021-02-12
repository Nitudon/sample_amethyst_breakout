use amethyst::{
    core::Transform,
    core::ecs::{DenseVecStorage, Component},
    prelude::*,
    renderer::SpriteRender,
};

use crate::component::*;
use crate::util::sprite::*;
use amethyst::core::math::Vector2;

const BALL_HEIGHT: f32 = 16.0;
const BALL_WIDTH: f32 = 16.0;
const BALL_START_Y: f32 = 240.0;
const BALL_BASE_SPEED_X: f32 = 1.5;
const BALL_BASE_SPEED_Y: f32 = 1.0;

#[derive(Clone)]
pub struct Ball {
    pub size: Vector2<f32>,
    pub speed: Vector2<f32>,
}

impl Ball {
    pub fn new(size: Vector2<f32>, speed: Vector2<f32>) -> Ball {
        Ball {
            size,
            speed
        }
    }
    
    pub fn reverse_speed_x(&mut self) {
        self.speed = Vector2::new(- self.speed.x, self.speed.y);
    }

    pub fn reverse_speed_y(&mut self) {
        self.speed = Vector2::new(self.speed.x, - self.speed.y);
    }
    
    pub fn set_speed_x_by_base_speed_coefficient(&mut self, coefficient: f32) {
        self.speed = Vector2::new(BALL_BASE_SPEED_X * coefficient, self.speed.y);
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_ball(world: &mut World) {
    let speed = Vector2::new(0.0, - BALL_BASE_SPEED_Y);
    let size = Vector2::new(BALL_WIDTH, BALL_HEIGHT);
    let mut ball = Ball::new(size, speed);
    let mut sprite = create_ball_sprite(world);
    let mut transform = Transform::default();
    transform.set_translation_xyz(SCREEN_WIDTH / 2.0, BALL_START_Y, 0.0);

    world
        .create_entity()
        .with(ball)
        .with(sprite)
        .with(transform)
        .build();
}

fn create_ball_sprite(world: &mut World) -> SpriteRender {
    create_sprite("texture/ball.png", "texture/ball_spritesheet.ron", 0, world)
}
