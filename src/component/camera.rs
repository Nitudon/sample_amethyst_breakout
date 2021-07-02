use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
};
use crate::component::*;

// カメラの初期化
pub fn create_camera(world: &mut World) {
    // 必要なComponentの用意
    let mut camera_transform = Transform::default();
    let camera_component = Camera::standard_2d(480.0, 600.0);
    camera_transform.set_translation_xyz(240.0, 300.0, 1.0);

    // EntityにCamera用のComponentを紐づけて生成
    world
        .create_entity()
        .with(camera_component)
        .with(camera_transform)
        .build();
}