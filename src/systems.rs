use bevy::{ecs::query::Has, prelude::*};

#[cfg(feature = "assets")]
pub mod asset_handling;

use crate::{
    animation_comp::{get_animation_seq, new_reapting_time, AnimationComp},
    animation_time_factor::AnimationTimeScale,
    listen_animation_end::ListenAnimationEnd,
    prelude::AllAnimationResource,
    types::AnimationResult,
    utils, AnimationEnded,
};

pub fn animate(
    query: Query<(
        Entity,
        &mut AnimationComp,
        &mut TextureAtlasSprite,
        &AnimationTimeScale,
        Has<ListenAnimationEnd>,
    )>,
    time: Res<Time>,
    repos: Res<AllAnimationResource>,
    on_animation_finish: EventWriter<AnimationEnded>,
) {
    if time.is_paused() {
        return;
    }

    let result = try_apply_update(query, &time, &repos, on_animation_finish);
    utils::log_if_error(result, "Updating animation frame over time failed.");

    fn try_apply_update(
        mut query: Query<(
            Entity,
            &mut AnimationComp,
            &mut TextureAtlasSprite,
            &AnimationTimeScale,
            Has<ListenAnimationEnd>,
        )>,
        time: &Time,
        repos: &AllAnimationResource,
        mut on_animation_finish: EventWriter<AnimationEnded>,
    ) -> AnimationResult {
        let mut animations_finished: Vec<AnimationEnded> = Vec::new();
        for (who, mut animator, mut current_sprite, time_scale, listen_end) in query.iter_mut() {
            let scaled_time = time_scale.scale_duration(time.delta());

            if animator
                .duration_for_animation
                .tick(scaled_time)
                .just_finished()
            {
                let current_animation = animator.get_current_seq(repos)?;
                let last_frame_has_ended =
                    listen_end && current_animation.end() == current_sprite.index;
                if last_frame_has_ended {
                    info!("{}", stringify!(last_frame_has_ended));
                    animations_finished.push(AnimationEnded::new_complete(
                        who,
                        animator.current_state.clone(),
                    ));
                }
                let next = current_sprite.index + 1;
                animator.duration_for_animation =
                    crate::animation_comp::new_reapting_time(current_animation.time());

                current_sprite.index = if next > current_animation.end() {
                    current_animation.start()
                } else {
                    next
                };
            }
        }
        if !animations_finished.is_empty() {
            on_animation_finish.send_batch(animations_finished.into_iter());
        }
        Ok(())
    }
}

pub fn apply_pending_states(
    mut query: Query<(
        Entity,
        &mut AnimationComp,
        &mut TextureAtlasSprite,
        Has<ListenAnimationEnd>,
    )>,
    repos: Res<AllAnimationResource>,
    mut on_animation_switch: EventWriter<AnimationEnded>,
) {
    for (who, mut animmator, mut texture_sprite, listen_ani_end) in query.iter_mut() {
        let mut animations_finished: Vec<AnimationEnded> = Vec::new();
        utils::log_if_error(
            try_apply_state_change(
                who,
                &mut animmator,
                &mut texture_sprite,
                listen_ani_end,
                &repos,
                &mut animations_finished,
            ),
            "Applying state change for animation failed.",
        );
        if !animations_finished.is_empty() {
            on_animation_switch.send_batch(animations_finished)
        }
    }

    fn try_apply_state_change(
        who: Entity,
        animator: &mut AnimationComp,
        to_adjust: &mut TextureAtlasSprite,
        listen_animation_end: bool,
        respo: &AllAnimationResource,
        on_change: &mut Vec<AnimationEnded>,
    ) -> AnimationResult {
        if let Some(new) = animator.next_state.take() {
            if listen_animation_end {
                let progress = utils::get_progress_of_animation(&animator, respo, to_adjust)?;
                let state = animator.current_state.clone();
                on_change.push(AnimationEnded {
                    who,
                    state,
                    progress,
                });
            }

            let (new_time, start) = {
                let (key, new_animation) = get_animation_seq(respo, &animator.sequence, &new)?;
                animator.current_state = key;
                (new_animation.time_per_frame(), new_animation.start())
            };
            animator.duration_for_animation = new_reapting_time(new_time);
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
            animator.duration_for_animation.reset();
        }
        Ok(())
    }
}
