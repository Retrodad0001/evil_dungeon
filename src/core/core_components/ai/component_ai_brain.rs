use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{tiw_tilemap::prelude::TiwTileMap, ComponentActorKind, ComponentAiAction};

#[derive(Component)]
pub(crate) struct ComponentAIBrain {
    pub(crate) current_action: ComponentAiAction,
    pub(crate) next_target_position: Option<Vec2>,
    pub(crate) ai_check_timer: Timer,
    pub(crate) chase_attack_range: f32,
    pub(crate) attack_melee_range: f32,
    pub(crate) max_wander_time: f32,
    wandering_timer: f32,
}

impl ComponentAIBrain {
    pub(crate) fn new(
        start_ai_action: ComponentAiAction,
        ai_check_timer: f32,
        chase_attack_range: f32,
        attack_melee_range: f32,
        max_wander_time: f32,
    ) -> Self {
        Self {
            current_action: start_ai_action,
            next_target_position: None,
            ai_check_timer: Timer::from_seconds(ai_check_timer, TimerMode::Repeating),
            chase_attack_range,
            attack_melee_range,
            wandering_timer: 0.0,
            max_wander_time,
        }
    }

    pub(crate) fn determine_new_state(
        &mut self,
        actor_kind: ComponentActorKind,
        enemy_pos: Vec2,
        player_pos: Vec2,
        tile_map: &TiwTileMap,
        max_wander_time: &f32,
    ) {
        match actor_kind {
            ComponentActorKind::PlayerKnight | ComponentActorKind::Wall => {
                //* ignore, these actors and the player (knight) have no AI
            }
            ComponentActorKind::BigZombie => {
                self.big_zombie_process_ai(enemy_pos, player_pos, tile_map, max_wander_time);
            }
        }
    }

    #[inline(always)]
    fn big_zombie_process_ai(
        &mut self,
        enemy_pos: Vec2,
        player_pos: Vec2,
        tile_map: &TiwTileMap,
        max_wander_time: &f32,
    ) {
        //* velocity is calculated in different system based of current AIState

        match self.current_action {
            ComponentAiAction::Idle => {
                self.current_action = ComponentAiAction::Wandering;
                self.next_target_position = None;
            }
            ComponentAiAction::Wandering => {
                let distance_to_player: f32 = get_distance_to_actor(enemy_pos, player_pos);
                let is_chasing_range: bool = distance_to_player < self.chase_attack_range;

                if is_chasing_range {
                    let already_chasing: bool = self.current_action == ComponentAiAction::Chasing;

                    if already_chasing {
                        return;
                    }

                    self.current_action = ComponentAiAction::Chasing;
                    self.next_target_position = Some(player_pos);
                } else {
                    self.wandering_timer += 1.0;
                    info!("wandering timer: {}", self.wandering_timer);

                    if self.wandering_timer < *max_wander_time {
                        return;
                    }

                    //TODO add random wander time, the code below panics, maybe use bevy specific random
                    //*  let mut rng: ThreadRng = rand::thread_rng();
                    //*  let next_random_wander_time: f32 =
                    //*  rng.gen_range(2..self.max_wander_time as i32) as f32;
                    //*  self.max_wander_time = next_random_wander_time;

                    self.wandering_timer = 0.0;

                    self.current_action = ComponentAiAction::Wandering;
                    let random_pos = tile_map.get_random_non_blocking_tile_world_position();
                    let random_pos_vec3: Vec2 = Vec2::new(random_pos.x, random_pos.y);
                    self.next_target_position = Some(random_pos_vec3);
                }
            }
            ComponentAiAction::Chasing => {
                if get_distance_to_actor(enemy_pos, player_pos) < self.attack_melee_range {
                    self.current_action = ComponentAiAction::AttackMelee;
                } else {
                    //* move towards player so state is still chasing,
                    self.current_action = ComponentAiAction::Chasing;
                    self.next_target_position = Some(player_pos);
                }
            }
            ComponentAiAction::AttackMelee => {
                //* attack actor */ by event

                if get_distance_to_actor(enemy_pos, player_pos) > self.attack_melee_range {
                    self.current_action = ComponentAiAction::Chasing;
                } else {
                    self.current_action = ComponentAiAction::AttackMelee;
                }
            }
        }
    }
}

#[inline(always)]
fn get_distance_to_actor(actor_from: Vec2, actor_to: Vec2) -> f32 {
    let result: f32 = Vec2::distance(actor_from, actor_to);
    result
}
