use macroquad::prelude::*;

#[macroquad::main(configuration_window)]
async fn main() {
    let mut x_pos: f32 = screen_width() * 0.5;
    let mut y_pos: f32 = screen_height() * 0.5;

    const MOVEMENT_SPEED: f32 = 200.0;
    const CIRCLE_RADIUS: f32 = 16.0;

    loop {
        clear_background(DARKBLUE);

        let delta_time = get_frame_time();

        if is_key_down(KeyCode::Left) {
            x_pos -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            x_pos += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            y_pos -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            y_pos += MOVEMENT_SPEED * delta_time;
        }

        x_pos = clamp(x_pos, CIRCLE_RADIUS, screen_width() - CIRCLE_RADIUS);
        y_pos = clamp(y_pos, CIRCLE_RADIUS, screen_height() - CIRCLE_RADIUS);

        draw_circle(x_pos, y_pos, CIRCLE_RADIUS, Color::from_hex(0xF9A03F));

        next_frame().await;
    }
}

fn configuration_window() -> Conf {
    Conf {
        window_title: "Macroquad Tutorial".to_string(),
        ..Default::default()
    }
}
