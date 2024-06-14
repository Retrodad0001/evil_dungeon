use bevy::prelude::*;

use crate::ScreenState;

pub(crate) fn go_next_screen(mut state: ResMut<NextState<ScreenState>>) {
    state.set(ScreenState::Playing);
    println!("go_next_screen");
}
