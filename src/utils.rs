use bevy::prelude::*;

pub fn get_direction_between_transform_and_cursor(
    window: &Window,
    player_transform: &Transform,
) -> Option<Vec2> {
    if let Some(pos) = window.cursor_position() {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let cursor_position = pos - size / 2.0;

        let direction = cursor_position - player_transform.translation.truncate();
        Some(direction)
    } else {
        None
    }
}
