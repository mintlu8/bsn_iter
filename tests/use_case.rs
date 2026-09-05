use bevy::prelude::{SceneList, bsn, bsn_list};
use bevy::{
    ecs::{component::Component, hierarchy::Children},
    math::Vec3,
    scene::Scene,
    sprite::Sprite,
    transform::components::Transform,
};
use bsn_iter::bsni;

#[derive(Clone, Component, Default)]
pub struct MyComponent;

#[derive(Clone, Component, Default)]
pub struct MyComponent2;

/// Ensure the thing on `README.md` works.
fn readme_example() -> impl Scene {
    let x_offsets = vec![0.0f32, 1., 2., 3.];
    let image_list = vec!["a.png", "b.png", "c.png", "d.png"];

    bsni!(
        MyComponent
        MyComponent2
        Children [
            $(
                Transform {
                    translation: Vec3 { x: $x_offsets }
                }
                Sprite {
                    image: $image_list
                }
            )*
        ]
    )
}

#[derive(Clone, Component, Default)]
pub struct Portrait(String);

#[derive(Clone, Component, Default)]
pub struct Text(String);

fn readme_example2() -> impl Scene {
    let sprite = ["1.png", "2.png", "3.png"];
    let name = ["red", "green", "blue"];

    bsni!(
        Children [
            $(
                Portrait($sprite),
                Text($name)
            )*
        ]
    )
}
