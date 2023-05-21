use bevy::prelude::*;

#[macro_export]
macro_rules! rgb_u8 {
    ($r:expr, $g:expr, $b:expr) => {
        Color::Rgba {
            red: $r as f32 / 255.,
            green: $g as f32 / 255.,
            blue: $b as f32 / 255.,
            alpha: 255.,
        }
    };

    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::Rgba {
            red: $r as f32 / 255.,
            green: $g as f32 / 255.,
            blue: $b as f32 / 255.,
            alpha: $a as f32 / 255.,
        }
    };
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}
