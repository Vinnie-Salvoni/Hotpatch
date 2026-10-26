use std::mem;

#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    ProjectileFired,

    ProjectileHit { damage: f32 },

    EnemyDied,

    ItemCollected { item_id: &'static str },
}

#[derive(Default)]
pub struct EventQueue {
    pending: Vec<GameEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&mut self, event: GameEvent) {
        self.pending.push(event);
    }

    pub fn drain(&mut self) -> Vec<GameEvent> {
        mem::take(&mut self.pending)
    }
}
