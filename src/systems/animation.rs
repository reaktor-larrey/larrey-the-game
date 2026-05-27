use bevy::prelude::*;

use crate::components::AnimationConfig;

pub fn run_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            // Last frame
            if atlas.index == config.last_sprite_index {
                atlas.index = config.first_sprite_index;
            } else {
                // Not last frame; advance to next frame
                atlas.index += 1;
                // Restart timer; timeout when next frame due
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}
