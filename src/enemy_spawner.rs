use macroquad::prelude::*;

use crate::enemy::{Enemy, EnemyModifier, EnemyType};

/// Controls which enemies can appear and how often they get special traits.
///
/// Later, each stage can use a different configuration.
pub struct EnemySpawnConfig<'a> {
    pub allowed_types: &'a [EnemyType],
    pub special_modifier_chance: f32,
    pub min_distance_from_center: f32,
}

impl<'a> EnemySpawnConfig<'a> {
    pub fn new(
        allowed_types: &'a [EnemyType],
        special_modifier_chance: f32,
        min_distance_from_center: f32,
    ) -> Self {
        Self {
            allowed_types,
            special_modifier_chance: special_modifier_chance.clamp(0.0, 1.0),
            min_distance_from_center: min_distance_from_center.max(0.0),
        }
    }
}

/// Creates a group of enemies at random positions around the room.
pub fn spawn_room_enemies(
    count: usize,
    room_width: f32,
    room_height: f32,
    config: &EnemySpawnConfig<'_>,
) -> Vec<Enemy> {
    let fallback_types = [EnemyType::Chaser];
    let allowed_types = if config.allowed_types.is_empty() {
        &fallback_types[..]
    } else {
        config.allowed_types
    };

    let mut enemies = Vec::with_capacity(count);

    for _ in 0..count {
        let type_index = macroquad::rand::gen_range(0, allowed_types.len());
        let kind = allowed_types[type_index];

        // The constructor rolls independent stats for this enemy.
        let mut enemy = Enemy::from_type(0.0, 0.0, kind);

        let max_x = (room_width - enemy.size.x).max(0.0);
        let max_y = (room_height - enemy.size.y).max(0.0);
        let room_center = vec2(room_width / 2.0, room_height / 2.0);

        // Try to avoid placing enemies directly on top of the player.
        for _ in 0..12 {
            let x = macroquad::rand::gen_range(0.0_f32, max_x + 1.0);
            let y = macroquad::rand::gen_range(0.0_f32, max_y + 1.0);
            let enemy_center = vec2(x, y) + enemy.size / 2.0;

            enemy.pos = vec2(x, y);

            if enemy_center.distance(room_center) >= config.min_distance_from_center {
                break;
            }
        }

        // Each trait is rolled independently, so an enemy can have
        // neither trait, either trait, or both.
        if macroquad::rand::gen_range(0.0_f32, 1.0_f32) < config.special_modifier_chance {
            enemy.add_modifier(EnemyModifier::ExplodesOnDeath);
        }

        if macroquad::rand::gen_range(0.0_f32, 1.0_f32) < config.special_modifier_chance {
            enemy.add_modifier(EnemyModifier::FiresTearsOnDeath);
        }

        enemies.push(enemy);
    }

    enemies
}
