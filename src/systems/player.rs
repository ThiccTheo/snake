use {
    crate::{
        components::{food::Food, player::*},
        prelude::*,
        resources::player::*,
    },
    bevy::{prelude::*, sprite::collide_aabb},
};

pub fn spawn_player(mut cmds: Commands) {
    cmds.spawn((
        SnakeHead,
        SnakeSegment { next: None },
        SnakeTail,
        PositionHistory { pos: Vec2::ZERO },
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(16.)),
                ..default()
            },
            ..default()
        },
        MoveDirection::Left,
    ));
    cmds.insert_resource(SnakeLength(1));
}

pub fn update_player_direction(
    keys: Res<Input<KeyCode>>,
    mut dir: Query<&mut MoveDirection, With<SnakeHead>>,
    snake_len: Res<SnakeLength>,
) {
    let mut dir = dir.single_mut();

    if keys.just_pressed(KeyCode::W) && (*dir != MoveDirection::Down || snake_len.0 == 1) {
        *dir = MoveDirection::Up;
    } else if keys.just_pressed(KeyCode::S) && (*dir != MoveDirection::Up || snake_len.0 == 1) {
        *dir = MoveDirection::Down;
    } else if keys.just_pressed(KeyCode::A) && (*dir != MoveDirection::Right || snake_len.0 == 1) {
        *dir = MoveDirection::Left;
    } else if keys.just_pressed(KeyCode::D) && (*dir != MoveDirection::Left || snake_len.0 == 1) {
        *dir = MoveDirection::Right;
    }
}

pub fn move_player(
    mut player: Query<(&mut Transform, &MoveDirection, &mut PositionHistory), With<SnakeHead>>,
) {
    let (mut transform, dir, mut old_pos) = player.single_mut();
    old_pos.pos = transform.translation.truncate();

    match *dir {
        MoveDirection::Up => transform.translation.y += 16.,
        MoveDirection::Down => transform.translation.y -= 16.,
        MoveDirection::Left => transform.translation.x -= 16.,
        MoveDirection::Right => transform.translation.x += 16.,
    }
}

pub fn eat_food(
    mut cmds: Commands,
    head: Query<&Transform, With<SnakeHead>>,
    food: Query<(Entity, &Transform), With<Food>>,
    mut tail: Query<(Entity, &mut SnakeSegment), With<SnakeTail>>,
    mut snake_len: ResMut<SnakeLength>,
) {
    let head_transform = head.single();
    let Ok((food_id, food_transform)) = food.get_single() else { return };
    let (tail_id, mut tail_segment) = tail.single_mut();

    if collide_aabb::collide(
        head_transform.translation,
        Vec2::splat(16.),
        food_transform.translation,
        Vec2::splat(16.),
    )
    .is_some()
    {
        cmds.entity(food_id).despawn_recursive();
        cmds.entity(tail_id).remove::<SnakeTail>();
        let next_segment_id = cmds
            .spawn((
                SnakeTail,
                SnakeSegment { next: None },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::splat(16.)),
                        ..default()
                    },
                    transform: *head_transform,
                    ..default()
                },
                PositionHistory {
                    pos: head_transform.translation.truncate(),
                },
            ))
            .id();
        tail_segment.next = Some(next_segment_id);
        snake_len.0 += 1;
    }
}

pub fn move_snake(
    segments: Query<(Entity, &SnakeSegment)>,
    mut transforms: Query<&mut Transform>,
    mut pos_histories: Query<&mut PositionHistory>,
) {
    for (prev_id, segment) in segments.into_iter().collect::<Vec<_>>().into_iter().rev() {
        if let Some(next_id) = segment.next {
            if let Ok([prev_transform, mut next_transform]) =
                transforms.get_many_mut([prev_id, next_id])
            {
                pos_histories.get_mut(next_id).unwrap().pos = next_transform.translation.truncate();
                *next_transform = *prev_transform;
            }
        }
    }
}

pub fn is_player_colliding(
    this: Query<&Transform, With<SnakeHead>>,
    others: Query<(&Transform, &Sprite), (Without<SnakeHead>, Without<Food>)>,
) -> bool {
    let player_transform = this.single();

    others.iter().any(|(t, s)| {
        collide_aabb::collide(
            player_transform.translation,
            Vec2::splat(16.),
            t.translation,
            s.custom_size.unwrap(),
        )
        .is_some()
    })
}

pub fn switch_state(
    mut next_state: ResMut<NextState<GameState>>,
    mut segments: Query<(&mut Transform, &PositionHistory), With<SnakeSegment>>,
) {
    for (mut transform, PositionHistory { pos }) in segments.iter_mut() {
        transform.translation = pos.extend(1.);
    }
    next_state.set(GameState::GameOver);
}
