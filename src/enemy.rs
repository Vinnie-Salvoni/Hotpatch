
use macroquad::prelude::*;

pub struct Enemy {
    pub pos: Vec2,
    pub size: Vec2,
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            size: vec2(28.0, 28.0),
            health: 3.0,
            max_health: 3.0,
            speed: 90.0,
        }
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
        draw_rectangle(
            self.pos.x,
            self.pos.y,
            self.size.x,
            self.size.y,
            Color::new(0.85, 0.12, 0.22, 1.0),
        );

        let bar_width = self.size.x;
        let health_ratio = (self.health / self.max_health).clamp(0.0, 1.0);

        draw_rectangle(
            self.pos.x,
            self.pos.y - 7.0,
            bar_width,
            4.0,
            DARKGRAY,
        );

        draw_rectangle(
            self.pos.x,
            self.pos.y - 7.0,
            bar_width * health_ratio,
            4.0,
            GREEN,
        );
    }
}
