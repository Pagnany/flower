use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub enum OverviewClickable {
    Flower(usize),
    BtnWater,
    BtnFertilize,
    BtnHarvest,
}

pub fn input_mouse_button(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    clickable_flowers: Query<(&crate::flower::Flower, &Sprite, &Transform)>,
    assets: Res<Assets<Image>>,
) {
    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                //println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                if ev.button == MouseButton::Left {
                    if let Some(position) = q_windows.single().cursor_position() {
                        // World coordinates start at the center of the screen
                        // Cursor coordinates start at the top left of the screen
                        let cur_pos = Vec2::new(
                            position.x - (crate::SCREEN_WIDTH / 2.0),
                            -1.0 * (position.y - (crate::SCREEN_HEIGHT / 2.0)),
                        );

                        for (flower, sprite, transform) in clickable_flowers.iter() {
                            let handle = sprite.image.clone();
                            let image_size = assets.get(&handle).unwrap().size();
                            let px_width = image_size.x as f32;
                            let px_height = image_size.y as f32;

                            if cur_pos.x > transform.translation.x - px_width / 2.0
                                && cur_pos.x < transform.translation.x + px_width / 2.0
                                && cur_pos.y > transform.translation.y - px_height / 2.0
                                && cur_pos.y < transform.translation.y + px_height / 2.0
                            {
                                println!("Clicked on flower {:?}", flower);
                            }
                        }
                    }
                }
            }
        }
    }
}
