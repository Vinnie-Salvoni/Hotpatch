use crate::enemy::Enemy;
use crate::event::{EventQueue, GameEvent};
use crate::projectile::Projectile;

/// Resolves projectile hits, publishes enemy death events,
/// and removes defeated enemies and used projectiles.
pub fn resolve_projectile_hits(
    projectiles: &mut Vec<Projectile>,
    enemies: &mut Vec<Enemy>,
    event_queue: &mut EventQueue,
) {
    // Each projectile can damage at most one enemy.
    for projectile in projectiles.iter_mut() {
        if projectile.radius <= 0.0 {
            continue;
        }

        for enemy in enemies.iter_mut() {
            if enemy.health > 0.0 && projectile.collides_with_rect(enemy.pos, enemy.size) {
                enemy.health -= projectile.damage;

                // A projectile disappears immediately after a hit.
                projectile.radius = 0.0;
                break;
            }
        }
    }

    // Publish one event for each enemy defeated this frame.
    let defeated_count = enemies.iter().filter(|enemy| enemy.health <= 0.0).count();

    for _ in 0..defeated_count {
        event_queue.publish(GameEvent::EnemyDied);
    }

    // Remove defeated enemies and used or offscreen projectiles.
    enemies.retain(|enemy| enemy.health > 0.0);
    projectiles.retain(|projectile| projectile.radius > 0.0 && !projectile.is_offscreen());
}
