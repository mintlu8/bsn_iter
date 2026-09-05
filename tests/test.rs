use bevy::app::App;
use bevy::asset::AssetPlugin;
use bevy::ecs::{component::Component, hierarchy::Children};
use bevy::scene::{Scene, SceneList, ScenePlugin, WorldSceneExt, bsn, bsn_list};
use bsn_iter::{bsni, bsni_list};

#[derive(Component, Clone, Copy, Default)]
pub struct Int(i32);

#[derive(Component, Clone, Copy, Default)]
pub struct Float(f32);

pub fn single_component() -> impl Scene {
    let numbers = [1, 2, 3, 4, 5];
    bsni! {
        Children [
            $(Int($numbers))*
        ]
    }
}

pub fn multi_times() -> impl Scene {
    let numbers = [1, 2, 3, 4, 5];
    bsni! {
        Children [
            $(Int($numbers) Float({$numbers as f32}))*
        ]
    }
}

pub fn multi_lists() -> impl Scene {
    let numbers = [1, 2, 3, 4, 5];
    let floats = [1.0f32, 2., 3., 4., 5.];
    bsni! {
        Children [
            $(Int($numbers) Float($floats))*
        ]
    }
}

pub fn multi_layers() -> impl Scene {
    let numbers = [1, 2, 3, 4, 5];
    let floats = [1.0f32, 2., 3., 4., 5.];
    bsni! {
        Children [
            $(
                Int($numbers)
                Children [
                    $(Float($floats))*
                ]
            )*
        ]
    }
}

pub fn multi_layers_duplicate() -> impl Scene {
    let numbers = [1, 2, 3, 4, 5];
    let floats = [1.0f32, 2., 3., 4., 5.];
    bsni! {
        Children [
            $(
                Int($numbers)
                Children [
                    $(
                        Int($numbers)
                        Float($floats)
                    )*
                ]
            )*
        ]
    }
}

pub fn scene_list() -> impl Scene {
    let numbers = [1, 2, 3, 4, 5];
    bsni! {
        Children [
            $(Int($numbers), Float({$numbers as f32}))*
        ]
    }
}

pub fn many_roots() -> impl SceneList {
    let numbers = [1, 2, 3, 4, 5];
    bsni_list! {
        $(Int($numbers), Float({$numbers as f32}))*
    }
}

pub fn many_roots_layered() -> impl SceneList {
    let numbers = [1, 2, 3, 4, 5];
    let numbers2 = [1, 2, 3, 4, 5];
    bsni_list! {
        $(
            Int($numbers)
            Children[
                $(Float({$numbers2 as f32}))*
            ],

            Float({$numbers as f32})
        )*
    }
}

#[track_caller]
pub fn validate(scene: impl Scene, entity_count: u32, children_on_root: usize) {
    let mut app = App::new();
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(ScenePlugin);
    let world = app.world_mut();
    let prev = world.entity_count();
    let e = world.spawn_scene(scene).unwrap().id();
    assert_eq!(world.entity_count() - prev, entity_count);
    assert_eq!(
        world.get::<Children>(e).map(|x| x.len()).unwrap_or(0),
        children_on_root
    )
}

#[track_caller]
pub fn validate_list(scene: impl SceneList, entity_count: u32) {
    let mut app = App::new();
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(ScenePlugin);
    let world = app.world_mut();
    let prev = world.entity_count();
    let _ = world.spawn_scene_list(scene);
    assert_eq!(world.entity_count() - prev, entity_count);
}

#[test]
pub fn test() {
    validate(single_component(), 6, 5);
    validate(multi_lists(), 6, 5);
    validate(multi_layers(), 31, 5);
    validate(multi_layers_duplicate(), 31, 5);
    validate(scene_list(), 11, 10);
    validate_list(many_roots(), 10);
    validate_list(many_roots_layered(), 35);
}
