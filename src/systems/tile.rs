use {crate::components::tile::*, bevy::prelude::*};

pub fn spawn_tiles(mut cmds: Commands) {
    for y in -21..=21 {
        for x in -21..=21 {
            if (y == -21 || y == 21) || (x == -21 || x == 21) {
                cmds.spawn((
                    Tile,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::GRAY,
                            custom_size: Some(Vec2::splat(16.)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x as f32 * 16., y as f32 * 16., 1.),
                        ..default()
                    },
                ));
            }
        }
    }
}
