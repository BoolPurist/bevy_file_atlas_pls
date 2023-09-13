use bevy::prelude::*;

use crate::types::AnimationReference;

#[derive(Event, Debug)]
pub struct AnimationEnded {
    pub who: Entity,
    pub progress: AnimationProgress,
    pub state: AnimationReference,
}

impl AnimationEnded {
    pub fn new_complete(who: Entity, state: AnimationReference) -> Self {
        Self {
            who,
            progress: AnimationProgress(1.),
            state,
        }
    }
    pub fn was_completed(&self) -> bool {
        self.progress.was_completed()
    }

    pub fn progress(&self) -> f32 {
        self.progress.progress()
    }
}

#[derive(Debug)]
pub struct AnimationProgress(f32);

impl AnimationProgress {
    pub fn new(progress: f32) -> Self {
        Self(progress.max(1.))
    }
    pub fn was_completed(&self) -> bool {
        self.0 >= 1.
    }

    pub fn progress(&self) -> f32 {
        self.0
    }
}
