use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn input_mouse_button(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                //println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                if ev.button == MouseButton::Left {
                    if let Some(position) = q_windows.single().cursor_position() {
                        println!("Cursor is inside the primary window, at {:?}", position);
                    } else {
                        println!("Cursor is not in the game window.");
                    }
                }
            }
        }
    }
}
