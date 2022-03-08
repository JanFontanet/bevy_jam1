use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Cooldown {
    timer: Timer,
}

impl Cooldown {
    pub fn new(seconds: f32) -> Self {
        let mut timer = Timer::from_seconds(seconds, false);
        timer.tick(Duration::from_secs_f32(seconds));
        Self { timer: timer }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn start(&mut self) {
        self.timer.reset()
    }

    // pub fn remaining(&self) -> f32 {
    //     self.timer.percent_left()
    // }

    pub fn finished(&self) -> bool {
        self.timer.finished()
    }
}

pub(crate) fn tick_cooldowns(mut query: Query<&mut Cooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        // Extra check here avoids change-detection false positives
        if !cooldown.finished() {
            cooldown.tick(time.delta());
        }
    }
}
