
use macroquad::prelude::*;

use crate::player::Player;

#[derive(Clone, Copy)]
pub enum ItemEffect {
    AddDamage(f32),
    MultiplyFireRate(f32),
    AddSpeed(f32),
    AddHealth(f32),
    MultiplyProjectileSpeed(f32),
}

pub struct ItemDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub effect: ItemEffect,
}

pub static ITEM_DEFINITIONS: [ItemDefinition; 5] = [
    ItemDefinition {
        id: "overclocked",
        name: "Overclocked",
        description: "Fire 50% faster.",
        effect: ItemEffect::MultiplyFireRate(1.5),
    },
    ItemDefinition {
        id: "heavy_payload",
        name: "Heavy Payload",
        description: "Deal 1 extra damage.",
        effect: ItemEffect::AddDamage(1.0),
    },
    ItemDefinition {
        id: "hot_wires",
        name: "Hot Wires",
        description: "Move 15% faster.",
        effect: ItemEffect::AddSpeed(45.0),
    },
    ItemDefinition {
        id: "velocity_patch",
        name: "Velocity Patch",
        description: "Projectiles travel 30% faster.",
        effect: ItemEffect::MultiplyProjectileSpeed(1.3),
    },
    ItemDefinition {
        id: "health_cache",
        name: "Health Cache",
        description: "Restore one heart container.",
        effect: ItemEffect::AddHealth(2.0),
    },
];

pub struct Item {
    pub pos: Vec2,
    pub size: Vec2,
    pub definition: &'static ItemDefinition,
    pub collected: bool,
}

impl Item {
    pub fn new(x: f32, y: f32, definition: &'static ItemDefinition) -> Self {
        Self {
            pos: vec2(x, y),
            size: vec2(22.0, 22.0),
            definition,
            collected: false,
        }
    }

    pub fn apply(&self, player: &mut Player) {
        match self.definition.effect {
            ItemEffect::AddDamage(amount) => {
                player.damage += amount;
            }
            ItemEffect::MultiplyFireRate(multiplier) => {
                player.fire_rate *= multiplier;
            }
            ItemEffect::AddSpeed(amount) => {
                player.speed += amount;
            }
            ItemEffect::AddHealth(amount) => {
                player.max_health += amount;
                player.health = (player.health + amount).min(player.max_health);
            }
            ItemEffect::MultiplyProjectileSpeed(multiplier) => {
                player.projectile_speed *= multiplier;
            }
        }
    }

    pub fn collides_with_player(&self, player_pos: Vec2, player_size: Vec2) -> bool {
        self.pos.x < player_pos.x + player_size.x
            && self.pos.x + self.size.x > player_pos.x
            && self.pos.y < player_pos.y + player_size.y
            && self.pos.y + self.size.y > player_pos.y
    }

    pub fn draw(&self) {
        if self.collected {
            return;
        }

        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            GOLD,
        );

        draw_rectangle_lines(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            2.0,
            Color::new(1.0, 0.75, 0.1, 1.0),
        );

        draw_text(
            "?",
            self.pos.x + 6.0,
            self.pos.y + 16.0,
            16.0,
            BLACK,
        );
    }
}
