use crate::enemy::Enemy;
use crate::projectile::Projectile;

/// Resolves projectile hits, removes defeated enemies,
/// and removes projectiles that have hit something or left the screen.
pub fn resolve_projectile_hits(projectiles: &mut Vec<Projectile>, enemies: &mut Vec<Enemy>) {
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

    // Remove defeated enemies.
    enemies.retain(|enemy| enemy.health > 0.0);

    // Remove used projectiles and projectiles outside the room.
    projectiles.retain(|projectile| projectile.radius > 0.0 && !projectile.is_offscreen());
}
