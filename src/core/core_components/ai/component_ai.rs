use bevy::prelude::*;

use crate::{AiState, ComponentActorKind};

#[derive(Component, Reflect, Resource, Default)]
#[reflect(Resource)]
pub(crate) struct ComponentAI {
    pub(crate) current_state: AiState,
    pub(crate) next_target_position: Option<Vec3>,
    pub(crate) timer: Timer,
    pub(crate) chase_attack_range: f32,
    pub(crate) attack_melee_range: f32,
}

impl ComponentAI {
    pub(crate) fn new(
        start_ai_state: AiState,
        ai_timer_in_sec: f32,
        chase_attack_range: f32,
        attack_melee_range: f32,
    ) -> Self {
        Self {
            current_state: start_ai_state,
            next_target_position: None,
            timer: Timer::from_seconds(ai_timer_in_sec, TimerMode::Repeating),
            chase_attack_range,
            attack_melee_range,
        }
    }

    pub(crate) fn determine_new_state(
        &mut self,
        actor_kind: ComponentActorKind,
        enemy_pos: Vec3,
        player_pos: Vec3,
    ) {
        match actor_kind {
            ComponentActorKind::PlayerKnight | ComponentActorKind::Wall => {
                //* ignore, these actors have no AI
            }
            ComponentActorKind::BigZombie => {
                self.big_zombie_process_ai(enemy_pos, player_pos);
            }
        }
    }

    #[inline(always)]
    fn big_zombie_process_ai(&mut self, enemy_pos: Vec3, player_pos: Vec3) {
        //* velocity is calculated in different system based of current AIState

        match self.current_state {
            AiState::Idle => {
                self.current_state = AiState::Wandering;
                self.next_target_position = None;
            }
            AiState::Wandering => {
                if get_distance_to_actor(enemy_pos, player_pos) < self.chase_attack_range {
                    self.current_state = AiState::Chasing;
                    self.next_target_position = Some(player_pos);
                } else {
                    self.current_state = AiState::Wandering;
                    self.next_target_position = Some(player_pos); //TODO randomize instead of player_pos
                }
            }
            AiState::Chasing => {
                if get_distance_to_actor(enemy_pos, player_pos) < self.attack_melee_range {
                    self.current_state = AiState::AttackMelee;
                } else {
                    //* move towards player so state is still chasing,
                    self.current_state = AiState::Chasing;
                    self.next_target_position = Some(player_pos);
                }
            }
            AiState::AttackMelee => {
                //* attack actor */ by event

                if get_distance_to_actor(enemy_pos, player_pos) > self.attack_melee_range {
                    self.current_state = AiState::Chasing;
                } else {
                    self.current_state = AiState::AttackMelee;
                }
            }
            AiState::AttackingWithSpawningEnemies | AiState::Fleeing => {
                //* ignore these states */
                self.next_target_position = None;
            }
        }
    }
}

#[inline(always)]
fn get_distance_to_actor(actor_from: Vec3, actor_to: Vec3) -> f32 {
    let result: f32 = Vec3::distance(actor_from, actor_to);
    result
}
