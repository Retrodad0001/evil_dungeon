use bevy::prelude::*;

use crate::{tiw_tilemap::prelude::TiwTileMap, ComponentActorKind, ComponentAiAction};

#[derive(Component)]
pub(crate) struct ComponentAIBrain {
    pub(crate) current_action: ComponentAiAction,
    pub(crate) next_target_position: Option<Vec3>,
    pub(crate) ai_check_timer: Timer,
    pub(crate) chase_attack_range: f32,
    pub(crate) attack_melee_range: f32,
    wandering_timer: f32,
}

impl ComponentAIBrain {
    pub(crate) fn new(
        start_ai_action: ComponentAiAction,
        ai_check_timer: f32,
        chase_attack_range: f32,
        attack_melee_range: f32,
    ) -> Self {
        Self {
            current_action: start_ai_action,
            next_target_position: None,
            ai_check_timer: Timer::from_seconds(ai_check_timer, TimerMode::Repeating),
            chase_attack_range,
            attack_melee_range,
            wandering_timer: 0.0,
        }
    }

    pub(crate) fn determine_new_state(
        &mut self,
        actor_kind: ComponentActorKind,
        enemy_pos: Vec3,
        player_pos: Vec3,
        tile_map: &TiwTileMap,
    ) {
        match actor_kind {
            ComponentActorKind::PlayerKnight | ComponentActorKind::Wall => {
                //* ignore, these actors and the player (knight) have no AI
            }
            ComponentActorKind::BigZombie => {
                self.big_zombie_process_ai(enemy_pos, player_pos, tile_map);
            }
            ComponentActorKind::Door => todo!(),
            ComponentActorKind::Axe => todo!(),
            ComponentActorKind::Wizard => todo!(),
            ComponentActorKind::WizardSpawn => todo!(),
            ComponentActorKind::SomeBoss => todo!(),
        }
    }

    #[inline(always)]
    fn big_zombie_process_ai(&mut self, enemy_pos: Vec3, player_pos: Vec3, tile_map: &TiwTileMap) {
        //* velocity is calculated in different system based of current AIState

        match self.current_action {
            ComponentAiAction::Idle => {
                self.current_action = ComponentAiAction::Wandering;
                self.next_target_position = None;
            }
            ComponentAiAction::Wandering => {
                let distance_to_player = get_distance_to_actor(enemy_pos, player_pos);
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

                    if self.wandering_timer < 10.0 {
                        return;
                    }

                    self.wandering_timer = 0.0;
                    self.current_action = ComponentAiAction::Wandering;
                    let random_pos = tile_map.get_random_non_blocking_tile_world_position();
                    let random_pos_vec3 = Vec3::new(random_pos.x, random_pos.y, 0.0);
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
            ComponentAiAction::AttackingWithSpawningEnemies | ComponentAiAction::Fleeing => {
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
