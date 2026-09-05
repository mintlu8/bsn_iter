use bevy::prelude::*;
use bsn_iter::bsni_list;

fn main() {
    let dx = -20..20;
    let dy = [-100.0f32, 0., 100.];
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    let _ = app.world_mut().spawn_scene_list(bsni_list!(
        $(
            Sprite {
                custom_size: {Some(Vec2 {x: 40., y: 40.})},
                color: Color::Hsla(Hsla { hue: {($dx as f32 * 5.).rem_euclid(360.)}, saturation: 1., lightness: 0.5, alpha: 1.0})
            }
            Transform {
                translation: Vec3 {x: {$dx as f32 * 50.}}
            }
            Children [
                $(
                    Sprite {
                        custom_size: {Some(Vec2 {x: 20., y: 20.})},
                        color: Color::Hsla(Hsla {
                            hue: {($dx as f32 * 5. + 180. + $dy / 4.).rem_euclid(360.)},
                            saturation: 1., lightness: 0.5, alpha: 1.0
                        })
                    }
                    Transform {
                        translation: Vec3 {y: $dy}
                    }
                )*
            ]
        )*
    ));
    app.world_mut().spawn(Camera2d);
    app.run();
}
