// Daydream Engine — Input: Swipe Gesture Detection
// Right = Yes, Left = No, Down = Dig Deeper
// Uses a shared Resource for inter-system communication (Bevy 0.18 compatible)

use bevy::prelude::*;
use crate::components::*;

/// Tracks an active drag gesture
#[derive(Resource, Default)]
pub struct DragState {
    pub active: bool,
    pub start_pos: Vec2,
    pub current_pos: Vec2,
}

/// Pending swipe action — checked each frame by the render system
#[derive(Resource, Default)]
pub struct PendingSwipe {
    pub direction: Option<SwipeChoice>,
}

/// Minimum drag distance to register as a swipe
const SWIPE_THRESHOLD: f32 = 80.0;

/// System: detect mouse drag start
pub fn drag_start(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut drag: ResMut<DragState>,
    slide: Res<CurrentSlide>,
) {
    if !slide.ready_for_input || slide.depth_showing {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.iter().next() {
            if let Some(pos) = window.cursor_position() {
                drag.active = true;
                drag.start_pos = pos;
                drag.current_pos = pos;
            }
        }
    }
}

/// System: track drag movement
pub fn drag_move(
    windows: Query<&Window>,
    mut drag: ResMut<DragState>,
) {
    if !drag.active { return; }
    if let Some(window) = windows.iter().next() {
        if let Some(pos) = window.cursor_position() {
            drag.current_pos = pos;
        }
    }
}

/// System: detect swipe on release
pub fn drag_end(
    mouse: Res<ButtonInput<MouseButton>>,
    mut drag: ResMut<DragState>,
    mut pending: ResMut<PendingSwipe>,
    slide: Res<CurrentSlide>,
) {
    if !drag.active { return; }

    if mouse.just_released(MouseButton::Left) {
        let delta = drag.current_pos - drag.start_pos;
        let magnitude = delta.length();

        if magnitude > SWIPE_THRESHOLD && slide.ready_for_input {
            let abs_x = delta.x.abs();
            let abs_y = delta.y.abs();

            pending.direction = if abs_x > abs_y {
                Some(if delta.x > 0.0 { SwipeChoice::Yes } else { SwipeChoice::No })
            } else if delta.y > 0.0 {
                Some(SwipeChoice::Deeper)
            } else {
                None // Up swipe ignored
            };
        }
        drag.active = false;
    }
}

/// System: keyboard arrow keys
pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut pending: ResMut<PendingSwipe>,
    slide: Res<CurrentSlide>,
) {
    if !slide.ready_for_input || slide.depth_showing { return; }

    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        pending.direction = Some(SwipeChoice::Yes);
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        pending.direction = Some(SwipeChoice::No);
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS)
        || keys.just_pressed(KeyCode::Space) {
        pending.direction = Some(SwipeChoice::Deeper);
    }
}
