use {crate::prelude::*, crate::systems::tile::*, bevy::prelude::*};

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tiles.in_schedule(OnEnter(GameState::Playing)));
    }
}
