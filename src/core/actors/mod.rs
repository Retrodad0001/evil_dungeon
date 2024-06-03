mod camera;
mod enemies;
mod player;
mod tiles;

pub(crate) mod prelude {
    pub(crate) use super::camera::prelude::*;
    pub(crate) use super::enemies::prelude::*;
    pub(crate) use super::player::prelude::*;
    pub(crate) use super::tiles::prelude::*;
}
