use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct SnakeSegment {
    pub next: Option<Entity>,
}

#[derive(Component)]
pub struct PositionHistory {
    pub pos: Vec2,
}
