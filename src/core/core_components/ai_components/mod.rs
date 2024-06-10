mod component_ai_attack_spawn_enemies;
mod component_ai_chase;
mod component_ai_flee;
mod component_ai_idle;
mod component_ai_wandering;
mod tiw_ai_trait;

pub(crate) mod prelude {
    pub(crate) use super::component_ai_attack_spawn_enemies::*;
    pub(crate) use super::component_ai_chase::*;
    pub(crate) use super::component_ai_flee::*;
    pub(crate) use super::component_ai_idle::*;
    pub(crate) use super::component_ai_wandering::*;
    pub(crate) use super::tiw_ai_trait::*;
}
