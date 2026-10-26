
use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub friction: f32,

    pub damage: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub health: f32,
    pub max_health: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        let size = vec2(28.0, 28.0);

        Self {
            pos: vec2(x - size.x / 2.0, y - size.y / 2.0),
            vel: Vec2::ZERO,
            size,
            speed: 300.0,
            friction: 1800.0,
            damage: 1.0,
            fire_rate: 4.0,
            projectile_speed: 500.0,
            health: 6.0,
            max_health: 6.0,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time().min(0.05);
        let mut input = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            input.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            input.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            input.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            input.x += 1.0;
        }

        if input.length_squared() > 0.0 {
            input = input.normalize();
        }

        let target_velocity = input * self.speed;
        let acceleration = if input.length_squared() > 0.0 {
            2400.0
        } else {
            self.friction
        };

        let difference = target_velocity - self.vel;
        let max_change = acceleration * dt;

        if difference.length_squared() > max_change * max_change {
            self.vel += difference.normalize() * max_change;
        } else {
            self.vel = target_velocity;
        }

        self.pos += self.vel * dt;

        self.pos.x = self.pos.x.clamp(
            0.0,
            (screen_width() - self.size.x).max(0.0),
        );
        self.pos.y = self.pos.y.clamp(
            0.0,
            (screen_height() - self.size.y).max(0.0),
        );
    }

    pub fn center(&self) -> Vec2 {
        self.pos + self.size / 2.0
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            WHITE,
        );

        // Small visual marker showing which way the player faces is
        // deliberately omitted for now; shooting is directional.
    }
}
