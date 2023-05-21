use {crate::prelude::*, crate::systems::food::*, bevy::prelude::*};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_food
                .in_set(OnUpdate(GameState::Playing))
                .run_if(not(does_food_exist)),
        );
    }
}
