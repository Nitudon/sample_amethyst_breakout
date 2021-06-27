use amethyst::{
    core::Transform,
    core::ecs::{DenseVecStorage, Component},
    prelude::*,
    renderer::SpriteRender,
};

use amethyst::core::math::Vector2;
use crate::component::sprite::create_sprite;

pub const BALL_BASE_SPEED_X: f32 = 9.0;
const BALL_RADIUS: f32 = 8.0;
const BALL_START_X: f32 = 240.0;
const BALL_START_Y: f32 = 240.0;
const BALL_BASE_SPEED_Y: f32 = 3.0;

#[derive(Clone)]
pub struct Ball {
    pub radius: f32,
    pub speed: Vector2<f32>,
}

impl Ball {
    pub fn new(radius: f32) -> Ball {
        Ball {
            radius,
            speed: Vector2::new(0., 0.)
        }
    }
    
    pub fn initialize_speed(&mut self) {
        self.speed = Vector2::new(0.0, - BALL_BASE_SPEED_Y); 
    }
    
    pub fn reverse_speed_x(&mut self) {
        self.speed = Vector2::new(- self.speed.x, self.speed.y);
    }

    pub fn reverse_speed_y(&mut self) {
        self.speed = Vector2::new(self.speed.x, - self.speed.y);
    }
    
    pub fn set_speed_x(&mut self, speed: f32) {
        self.speed = Vector2::new(speed, self.speed.y);
    }
    
    pub fn stop(&mut self) {
        self.speed = Vector2::new(0., 0.);
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_ball(world: &mut World) {
    let ball = Ball::new(BALL_RADIUS);
    let sprite = create_ball_sprite(world);
    let mut transform = Transform::default();
    transform.set_translation_xyz(BALL_START_X, BALL_START_Y, 0.0);

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
