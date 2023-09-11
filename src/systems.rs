use bevy::prelude::*;

#[cfg(feature = "assets")]
pub mod asset_handling;

use crate::{
    animation_comp::{get_animation_seq, new_reapting_time, AnimationComp},
    animation_time_factor::AnimationTimeScale,
    prelude::AllAnimationResource,
    types::AnimationResult,
    utils,
};

pub fn animate(
    mut query: Query<(
        &mut AnimationComp,
        &mut TextureAtlasSprite,
        &AnimationTimeScale,
    )>,
    time: Res<Time>,
    repos: Res<AllAnimationResource>,
) {
    if time.is_paused() {
        return;
    }

    for (mut animmator, mut texture_sprite, time_scale) in query.iter_mut() {
        utils::log_if_error(
            try_apply_update(
                &mut animmator,
                &mut texture_sprite,
                &time_scale,
                &time,
                &repos,
            ),
            "Updating animation frame over time failed.",
        );
    }

    fn try_apply_update(
        animmtor: &mut AnimationComp,
        altlas_sprite: &mut TextureAtlasSprite,
        time_scale: &AnimationTimeScale,
        time: &Time,
        repos: &AllAnimationResource,
    ) -> AnimationResult<()> {
        let scaled_time = time_scale.scale_duration(time.delta());

        if animmtor
            .frame_seq_duration
            .tick(scaled_time)
            .just_finished()
        {
            let next = altlas_sprite.index + 1;
            let current_animation = animmtor.get_current_seq(repos)?;
            animmtor.frame_seq_duration =
                crate::animation_comp::new_reapting_time(current_animation.time());
            altlas_sprite.index = if next > current_animation.end() {
                current_animation.start()
            } else {
                next
            }
        };
        Ok(())
    }
}

pub fn apply_pending_states(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animmator, mut texture_sprite) in query.iter_mut() {
        utils::log_if_error(
            try_apply_state_change(&mut animmator, &mut texture_sprite, &repos),
            "Applying state change for animation failed.",
        );
    }

    fn try_apply_state_change(
        animator: &mut AnimationComp,
        to_adjust: &mut TextureAtlasSprite,
        respo: &AllAnimationResource,
    ) -> AnimationResult {
        if let Some(new) = animator.next_state.take() {
            let (new_time, start) = {
                let (key, new_animation) = get_animation_seq(respo, &animator.all_frames, &new)?;
                animator.current_state = key;
                (new_animation.time_per_frame(), new_animation.start())
            };
            animator.frame_seq_duration = new_reapting_time(new_time);
            to_adjust.index = start;
        }
        Ok(())
    }
}
pub fn do_pending_resets(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animator, mut texture_sprite) in query.iter_mut() {
        let result = apply_state_reset(&mut animator, &mut texture_sprite, &repos);
        utils::log_if_error(result, "Applying state reset for animation failed.");
    }

    fn apply_state_reset(
        animator: &mut AnimationComp,
        texture_sprite: &mut TextureAtlasSprite,
        repos: &AllAnimationResource,
    ) -> AnimationResult<()> {
        if animator.reset_state {
            animator.reset_state = false;
            let current_animation = animator.get_current_seq(&repos)?;
            texture_sprite.index = current_animation.start();
            animator.frame_seq_duration.reset();
        }
        Ok(())
    }
}
