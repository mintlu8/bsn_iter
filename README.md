# bsn_iter

`macro_rules!` style iteration for BSN.

## Syntax

Simply replace `bsn!` with `bsni!`, `bsn_list!` with `bsni_list!`.

```rust
use bevy::prelude::bsn;

let x_offsets = vec![0.0f32, 1., 2., 3.];
let image_list = vec!["a.png", "b.png", "c.png", "d.png"];

bsni!(
    MyComponent,
    MyComponent2,
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
```

Repeating multiple entities are allowed, but we must manually import `bsn_list!` and `SceneList`.

```rust
use bevy::prelude::{bsn, bsn_list, SceneList};

bsni!(
    Children [
        $(
            Portrait($sprite),
            Text($name)
        )*
    ]
)
```

## Output

The macro coverts lists into `Vec<impl Scene>` or `Vec<Box<dyn SceneList>>`,
this only works for the root portion of
`bsn_list`, `Children` or other relationships.

## Imports

This crate does not depend on bevy, so you must manually import `bsn!` or `bsn_list!` if needed,
if any repeated portion contains multiple entities (contains `,`), you must manually import `SceneList`
as we need to construct a `Box<dyn SceneList>`.
