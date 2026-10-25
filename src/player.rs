use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub friction: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            vel: Vec2::ZERO,
            size: vec2(32.0, 32.0),
            speed: 500.0,
            friction: 1800.0,
        }
    }

    pub fn update(&mut self) {
        // Limit unusually large frame times after a pause.
        let dt = get_frame_time().min(0.05);

        let mut input = Vec2::ZERO;

        // Capture directional input.
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            input.y -= 1.0;
        }

        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            input.y += 1.0;
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            input.x -= 1.0;
        }

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            input.x += 1.0;
        }

        // Normalize diagonal input so diagonal movement
        // isn't faster than horizontal or vertical movement.
        if input.length_squared() > 0.0 {
            input = input.normalize();
        }

        // Always use the player's CURRENT speed stat.
        // Items and upgrades can change this at runtime.
        let target_velocity = input * self.speed;

        // Accelerate while moving and decelerate when released.
        let acceleration = if input.length_squared() > 0.0 {
            2400.0
        } else {
            self.friction
        };

        let difference = target_velocity - self.vel;
        let max_change = acceleration * dt;

        // Smoothly approach the target velocity without overshooting.
        if difference.length_squared() > max_change * max_change {
            self.vel += difference.normalize() * max_change;
        } else {
            self.vel = target_velocity;
        }

        // Apply velocity to position.
        self.pos += self.vel * dt;
    }

    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size.x, self.size.y, WHITE);
    }
}
