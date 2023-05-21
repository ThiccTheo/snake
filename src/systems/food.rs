use {
    crate::components::{food::*, player::SnakeSegment},
    bevy::prelude::*,
    rand::Rng,
};

pub fn spawn_food(mut cmds: Commands, snake_positions: Query<&Transform, With<SnakeSegment>>) {
    let generate_rand_pos = || {
        Vec3::new(
            rand::thread_rng().gen_range(-20..=20) as f32 * 16.,
            rand::thread_rng().gen_range(-20..=20) as f32 * 16.,
            1.,
        )
    };

    let mut spawn_pos = generate_rand_pos();

    while snake_positions
        .iter()
        .any(|snake_pos| snake_pos.translation == spawn_pos)
    {
        spawn_pos = generate_rand_pos();
    }
    cmds.spawn((
        Food,
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(16.)),
                ..default()
            },
            transform: Transform::from_translation(spawn_pos),
            ..default()
        },
    ));
}

pub fn does_food_exist(food: Query<(), With<Food>>) -> bool {
    !food.is_empty()
}
