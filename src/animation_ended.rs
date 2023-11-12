use bevy::prelude::*;

use crate::{types::AnimationReference, PercentScaleFactor};

#[derive(Event, Debug)]
pub struct AnimationEnded {
    pub who: Entity,
    pub progress: PercentScaleFactor,
    pub state: AnimationReference,
}

impl AnimationEnded {
    pub fn new_complete(who: Entity, state: AnimationReference) -> Self {
        Self {
            who,
            progress: PercentScaleFactor::new_as_complete(),
            state,
        }
    }
    pub fn was_completed(&self) -> bool {
        self.progress.is_complete()
    }

    pub fn progress(&self) -> f32 {
        self.progress.to_f32()
    }
}
