mod actors;
mod constants_general;
mod core_components;
mod debugging;
mod events;
mod resource_animation_info;
mod resource_game_settings;
mod resource_general_game_state;
mod screen_menu;
mod screen_playing;
mod screen_state;

pub(crate) mod prelude {
    pub(crate) use super::actors::prelude::*;
    pub(crate) use super::constants_general::*;
    pub(crate) use super::core_components::prelude::*;
    pub(crate) use super::debugging::prelude::*;
    pub(crate) use super::events::prelude::*;
    pub(crate) use super::resource_animation_info::*;
    pub(crate) use super::resource_game_settings::*;
    pub(crate) use super::resource_general_game_state::*;
    pub(crate) use super::screen_menu::*;
    pub(crate) use super::screen_playing::*;
    pub(crate) use super::screen_state::*;
}
