use macroquad::prelude::*;

pub struct Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub damage: f32,
    pub radius: f32,
}

impl Projectile {
    pub fn new(pos: Vec2, direction: Vec2, speed: f32, damage: f32) -> Self {
        Self {
            pos,
            vel: direction.normalize_or_zero() * speed,
            damage,
            radius: 5.0,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time().min(0.05);
        self.pos += self.vel * dt;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, RED);
        draw_circle_lines(
            self.pos.x,
            self.pos.y,
            self.radius,
            1.0,
            Color::new(1.0, 0.65, 0.65, 1.0),
        );
    }

    pub fn is_offscreen(&self) -> bool {
        self.pos.x < -20.0
            || self.pos.y < -20.0
            || self.pos.x > screen_width() + 20.0
            || self.pos.y > screen_height() + 20.0
    }

    pub fn collides_with_rect(&self, rect_pos: Vec2, rect_size: Vec2) -> bool {
        let nearest_x = self.pos.x.clamp(rect_pos.x, rect_pos.x + rect_size.x);
        let nearest_y = self.pos.y.clamp(rect_pos.y, rect_pos.y + rect_size.y);

        let dx = self.pos.x - nearest_x;
        let dy = self.pos.y - nearest_y;

        dx * dx + dy * dy <= self.radius * self.radius
    }
}
