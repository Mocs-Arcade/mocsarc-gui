
use super::*;

/// Contains Data about the current game selection in the GUI. 
pub struct Selector {
    // The ID of the Currently Selected game
    pub current: u32,
    // the total number of games available
    pub total_games: u32,
    // the y value of the currently selected game for the camera to lerp to
    pub y_value: f32,
}

impl Default for Selector {
    fn default() -> Self {
        Self { current: 0, total_games: 0, y_value: 0.0 }
    }
}

// Configuration Constants //
// The rate at which the cover image will scale when selected
const SCALE_SMOOTHING : f32 = 0.35;
// the rate at which the camera will move to follow the selected
const TRANSLATE_SMOOTHING : f32 = 0.15;

pub fn select (
    mut res: ResMut<Selector>,
    mut query_games: Query<(&mut GameListing, &ID, &mut Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    // Joystick up
    if keys.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        // keep from going out of bounds
        if res.current > 4 { res.current -= 5; }
    }
    // Joystick Down
    if keys.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        // keep from going out of bounds
        if res.current + 5 < res.total_games - 1 { res.current += 5; }
    }
    // Joystick Left
    if keys.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        // keep from going out of bounds
        if res.current != 0 { res.current -= 1; }
    }
    // Joystick Right
    if keys.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        // keep from going out of bounds
        if res.current + 1 < res.total_games - 1 { res.current += 1; }
    }
    // A down - select a game
    if keys.any_just_pressed([KeyCode::J, KeyCode::E]) {
        
    }

    // Lerp the scale o the selected scale. 
    for (_gl, id, mut tr) in &mut query_games {
        // if this is the current selected game, scale up.
        if id.0 == res.current {
            res.y_value = tr.translation.y;
            let scale = tr.scale.x.lerp(1.15, SCALE_SMOOTHING);
            tr.translation = Vec3::new(tr.translation.x, tr.translation.y, 100.0);
            tr.scale = Vec3::new(scale,scale,scale);
        // else scale down to normal
        } else {
            let scale = tr.scale.x.lerp(1.0, SCALE_SMOOTHING);
            tr.translation = Vec3::new(tr.translation.x, tr.translation.y, 0.0);
            tr.scale = Vec3::new(scale,scale,scale);
        }
    }
}

/// Lerp the Camera to the Y-value of the current selected game.
pub fn lerp_camera (
    res: ResMut<Selector>,
    mut query_cam: Query<(&mut Transform, With<Camera>)>,
) {
    for (mut tr, _o) in &mut query_cam {
        tr.translation.y = tr.translation.y.lerp(res.y_value - 50.0,TRANSLATE_SMOOTHING);
    }
}