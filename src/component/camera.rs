use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::Camera,
};
use crate::component::*;

pub fn create_camera(world: &mut World) {
    let mut camera_transform = Transform::default();
    let camera_component = Camera::standard_2d(AREA_WIDTH, AREA_HEIGHT);
    camera_transform.set_translation_xyz(AREA_WIDTH * 0.5, AREA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(camera_component)
        .with(camera_transform)
        .build();
}
