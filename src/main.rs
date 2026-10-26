use macroquad::prelude::*;

mod enemy;
mod item;
mod player;
mod projectile;

use enemy::Enemy;
use item::{Item, ITEM_DEFINITIONS};
use player::Player;
use projectile::Projectile;

#[macroquad::main("Hot Patch")]
async fn main() {
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    let mut enemies = vec![
        Enemy::new(100.0, 100.0),
        Enemy::new(screen_width() - 140.0, 100.0),
        Enemy::new(screen_width() / 2.0 - 14.0, 100.0),
    ];

    let mut items = vec![
        Item::new(100.0, screen_height() - 100.0, &ITEM_DEFINITIONS[0]),
        Item::new(screen_width() - 100.0, screen_height() - 100.0, &ITEM_DEFINITIONS[1]),
        Item::new(screen_width() / 2.0 + 70.0, screen_height() / 2.0, &ITEM_DEFINITIONS[2]),
        Item::new(screen_width() / 2.0 - 100.0, screen_height() / 2.0, &ITEM_DEFINITIONS[3]),
        Item::new(screen_width() / 2.0, screen_height() / 2.0 + 100.0, &ITEM_DEFINITIONS[4]),
    ];

    let mut projectiles: Vec<Projectile> = Vec::new();
    let mut shot_cooldowns = [0.0_f32; 4];

    let mut paused = false;
    let mut show_map = false;
    let mut notification = String::from("WASD move | Arrow keys fire");
    let mut notification_timer = 3.0_f32;

    // Up, down, left, right. Each direction has its own firing cooldown.
    let shooting = [
        (KeyCode::Up, vec2(0.0, -1.0)),
        (KeyCode::Down, vec2(0.0, 1.0)),
        (KeyCode::Left, vec2(-1.0, 0.0)),
        (KeyCode::Right, vec2(1.0, 0.0)),
    ];

    loop {
        let dt = get_frame_time().min(0.05);

        if is_key_pressed(KeyCode::Escape) {
            paused = !paused;
        }

        if is_key_pressed(KeyCode::Tab) {
            show_map = !show_map;
        }

        if is_key_pressed(KeyCode::E) {
            notification = String::from("Bombs are not implemented yet.");
            notification_timer = 2.0;
        }

        if is_key_pressed(KeyCode::Space) {
            notification = String::from("No active item equipped yet.");
            notification_timer = 2.0;
        }

        if is_key_pressed(KeyCode::Q) {
            notification = String::from("No consumable available yet.");
            notification_timer = 2.0;
        }

        if is_key_pressed(KeyCode::LeftControl) {
            notification = String::from("Item dropping is not implemented yet.");
            notification_timer = 2.0;
        }

        if notification_timer > 0.0 {
            notification_timer -= dt;
        }

        if !paused {
            player.update();

            // Directional shooting is independent of movement.
            for (index, (key, direction)) in shooting.iter().enumerate() {
                shot_cooldowns[index] = (shot_cooldowns[index] - dt).max(0.0);

                if is_key_down(*key) && shot_cooldowns[index] <= 0.0 {
                    projectiles.push(Projectile::new(
                        player.center(),
                        *direction,
                        player.projectile_speed,
                        player.damage,
                    ));

                    shot_cooldowns[index] = 1.0 / player.fire_rate.max(0.1);
                }
            }

            for projectile in &mut projectiles {
                projectile.update();
            }

            for enemy in &mut enemies {
                enemy.update(player.center());
            }

            // Apply damage once per projectile hit.
            for projectile in &mut projectiles {
                if projectile.radius <= 0.0 {
                    continue;
                }

                for enemy in &mut enemies {
                    if enemy.health > 0.0
                        && projectile.collides_with_rect(enemy.pos, enemy.size)
                    {
                        enemy.health -= projectile.damage;
                        projectile.radius = 0.0;
                        break;
                    }
                }
            }

            enemies.retain(|enemy| enemy.health > 0.0);

            projectiles.retain(|projectile| {
                projectile.radius > 0.0 && !projectile.is_offscreen()
            });

            // Collect each item once.
            for item in &mut items {
                if !item.collected
                    && item.collides_with_player(player.pos, player.size)
                {
                    item.apply(&mut player);
                    item.collected = true;

                    notification = format!(
                        "{} — {}",
                        item.definition.name,
                        item.definition.description
                    );
                    notification_timer = 3.0;
                }
            }
        }

        // Draw the room.
        clear_background(Color::new(0.05, 0.06, 0.08, 1.0));

        let grid_color = Color::new(0.13, 0.15, 0.19, 1.0);
        let grid_spacing = 40.0;

        let mut x = 0.0;
        while x < screen_width() {
            draw_line(x, 0.0, x, screen_height(), 1.0, grid_color);
            x += grid_spacing;
        }

        let mut y = 0.0;
        while y < screen_height() {
            draw_line(0.0, y, screen_width(), y, 1.0, grid_color);
            y += grid_spacing;
        }

        for item in &items {
            item.draw();
        }

        for projectile in &projectiles {
            projectile.draw();
        }

        for enemy in &enemies {
            enemy.draw();
        }

        player.draw();

        // HUD.
        draw_text(
            &format!("HP: {:.0}/{:.0}", player.health, player.max_health),
            20.0,
            28.0,
            22.0,
            WHITE,
        );

        draw_text(
            &format!(
                "DMG {:.1}   FIRE {:.1}/s   SPEED {:.0}",
                player.damage,
                player.fire_rate,
                player.speed
            ),
            20.0,
            54.0,
            18.0,
            LIGHTGRAY,
        );

        draw_text(
            &format!("Enemies: {}", enemies.len()),
            20.0,
            78.0,
            18.0,
            LIGHTGRAY,
        );

        if notification_timer > 0.0 {
            let text_width = measure_text(&notification, None, 20, 1.0).width;
            draw_text(
                &notification,
                (screen_width() - text_width) / 2.0,
                screen_height() - 24.0,
                20.0,
                GOLD,
            );
        }

        if show_map {
            let map_width = 180.0;
            let map_height = 130.0;
            let map_x = screen_width() - map_width - 20.0;
            let map_y = 20.0;

            draw_rectangle(
                map_x,
                map_y,
                map_width,
                map_height,
                Color::new(0.02, 0.02, 0.03, 0.9),
            );
            draw_rectangle_lines(
                map_x,
                map_y,
                map_width,
                map_height,
                2.0,
                LIGHTGRAY,
            );

            // Current prototype has one room, so the map shows that room.
            draw_rectangle(
                map_x + 65.0,
                map_y + 45.0,
                50.0,
                40.0,
                DARKGRAY,
            );
            draw_circle(
                map_x + 90.0,
                map_y + 65.0,
                4.0,
                WHITE,
            );
            draw_text("MAP", map_x + 8.0, map_y + 18.0, 14.0, LIGHTGRAY);
        }

        if paused {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.72),
            );

            let title = "PAUSED";
            let title_width = measure_text(title, None, 40, 1.0).width;
            draw_text(
                title,
                (screen_width() - title_width) / 2.0,
                screen_height() / 2.0,
                40.0,
                WHITE,
            );

            let subtitle = "Press Esc to resume";
            let subtitle_width = measure_text(subtitle, None, 20, 1.0).width;
            draw_text(
                subtitle,
                (screen_width() - subtitle_width) / 2.0,
                screen_height() / 2.0 + 35.0,
                20.0,
                LIGHTGRAY,
            );
        }

        if enemies.is_empty() {
            let message = "ROOM CLEAR";
            let message_width = measure_text(message, None, 32, 1.0).width;
            draw_text(
                message,
                (screen_width() - message_width) / 2.0,
                110.0,
                32.0,
                GOLD,
            );
        }

        next_frame().await;
    }
}
