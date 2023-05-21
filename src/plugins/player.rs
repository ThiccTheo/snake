use {
    crate::{prelude::*, systems::player::*},
    bevy::prelude::*,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (update_player_direction, move_snake, move_player, eat_food)
                    .chain()
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(
                switch_state
                    .run_if(is_player_colliding)
                    .after(move_player)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}
