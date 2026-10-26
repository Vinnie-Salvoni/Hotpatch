use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Chaser,
    Runner,
}

/// Special traits that can be attached to individual enemies.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyModifier {
    ExplodesOnDeath,
    FiresTearsOnDeath,
}

/// Baseline stats for an enemy type.
struct EnemyBaseStats {
    health: f32,
    speed: f32,
    damage: f32,
    size: f32,
}

pub struct Enemy {
    pub pos: Vec2,
    pub size: Vec2,

    // Individual rolled stats.
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub damage: f32,

    pub kind: EnemyType,
    pub modifiers: Vec<EnemyModifier>,
}

impl Enemy {
    /// Creates a Chaser with independently randomised stats.
    pub fn new(x: f32, y: f32) -> Self {
        Self::from_type(x, y, EnemyType::Chaser)
    }

    /// Creates a Runner with independently randomised stats.
    pub fn runner(x: f32, y: f32) -> Self {
        Self::from_type(x, y, EnemyType::Runner)
    }

    /// Creates an enemy from its type's baseline stats.
    pub fn from_type(x: f32, y: f32, kind: EnemyType) -> Self {
        let base = match kind {
            EnemyType::Chaser => EnemyBaseStats {
                health: 8.0,
                speed: 90.0,
                damage: 1.0,
                size: 28.0,
            },
            EnemyType::Runner => EnemyBaseStats {
                health: 10.0,
                speed: 150.0,
                damage: 1.0,
                size: 22.0,
            },
        };

        // Each stat receives its own multiplier.
        // These values are rolled once when the enemy spawns.
        let health_multiplier = random_multiplier();
        let speed_multiplier = random_multiplier();
        let damage_multiplier = random_multiplier();

        let max_health = (base.health * health_multiplier).max(1.0);
        let speed = (base.speed * speed_multiplier).max(1.0);
        let damage = (base.damage * damage_multiplier).max(0.1);

        Self {
            pos: vec2(x, y),
            size: vec2(base.size, base.size),
            health: max_health,
            max_health,
            speed,
            damage,
            kind,
            modifiers: Vec::new(),
        }
    }

    /// Adds a special trait to this individual enemy.
    pub fn add_modifier(&mut self, modifier: EnemyModifier) {
        if !self.modifiers.contains(&modifier) {
            self.modifiers.push(modifier);
        }
    }

    /// Checks whether this enemy has a particular special trait.
    pub fn has_modifier(&self, modifier: EnemyModifier) -> bool {
        self.modifiers.contains(&modifier)
    }

    pub fn update(&mut self, target: Vec2) {
        let dt = get_frame_time().min(0.05);

        let enemy_center = self.pos + self.size / 2.0;
        let direction = target - enemy_center;

        if direction.length_squared() > 1.0 {
            self.pos += direction.normalize() * self.speed * dt;
        }
    }

    pub fn draw(&self) {
        let color = match self.kind {
            EnemyType::Chaser => Color::new(0.85, 0.12, 0.22, 1.0),
            EnemyType::Runner => Color::new(1.0, 0.55, 0.10, 1.0),
        };

        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, color);

        // Show a small purple marker when an enemy has a special trait.
        if !self.modifiers.is_empty() {
            draw_circle(self.pos.x + self.size.x, self.pos.y, 4.0, PURPLE);
        }

        let health_ratio = (self.health / self.max_health).clamp(0.0, 1.0);

        draw_rectangle(self.pos.x, self.pos.y - 7.0, self.size.x, 4.0, DARKGRAY);

        draw_rectangle(
            self.pos.x,
            self.pos.y - 7.0,
            self.size.x * health_ratio,
            4.0,
            GREEN,
        );
    }
}

/// Returns a random stat multiplier between 0.8x and 1.4x.
fn random_multiplier() -> f32 {
    macroquad::rand::gen_range(0.8_f32, 1.4_f32)
}
