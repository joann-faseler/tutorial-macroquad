use ::glam::Vec2;
use macroquad::{prelude::*, rand::gen_range};

struct Shape {
    size: f32,
    speed: f32,
    position: Vec2,
    color: Color,
}

impl Shape {
    fn rect(&self) -> Rect {
        Rect {
            x: self.position.x - (self.size * 0.5),
            y: self.position.y - (self.size * 0.5),
            w: self.size,
            h: self.size,
        }
    }

    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }
}

#[derive(Default)]
struct Timer {
    duration: f32,
    current_timer: f32,
}

#[macroquad::main(configuration_window)]
async fn main() {
    // Set the random number'seed on the current time
    // Doing so produce different numbers every time the game is run.
    rand::srand(miniquad::date::now() as u64);

    let mut game_over: bool = false;
    let mut debug_mode: bool = false;

    let mut timer = Timer {
        duration: 0.3,
        ..Default::default()
    };

    timer.current_timer = timer.duration;

    let mut mobs: Vec<Shape> = vec![];

    let mut player = Shape {
        size: 16.0,
        speed: 250.0,
        position: Vec2::new(screen_width(), screen_height()) * 0.5,
        color: Color::from_hex(0xEB5E28),
    };

    let mut velocity: Vec2;

    const CIRCLE_COLOR: Color = Color::from_hex(0xF9A03F);

    loop {
        clear_background(Color::from_hex(0xFFFCF2));
        if (!game_over) {
            let delta_time = get_frame_time();
            velocity = Vec2::ZERO;

            // Generate mob
            if timer.current_timer > 0.0 {
                timer.current_timer -= delta_time;
            }

            if timer.current_timer <= 0.0 {
                let size = gen_range(16.0, 64.0);
                let speed = gen_range(100.0, 200.0);
                let position = Vec2::new(gen_range(size * 0.5, screen_width() - size * 0.5), -size);
                let color: Color = match gen_range(0, 3) {
                    0 => Color::from_hex(0x252422),
                    1 => Color::from_hex(0x403D39),
                    2 => Color::from_hex(0xCCC5B9),
                    _ => GREEN,
                };

                mobs.push(Shape {
                    size,
                    speed,
                    position,
                    color,
                });

                timer.current_timer = timer.duration;
            }

            // Update position
            if is_key_down(KeyCode::Left) {
                velocity.x -= 1.0;
            }
            if is_key_down(KeyCode::Right) {
                velocity.x += 1.0;
            }
            if is_key_down(KeyCode::Up) {
                velocity.y -= 1.0;
            }
            if is_key_down(KeyCode::Down) {
                velocity.y += 1.0;
            }

            // Break out of the loop therefore quitting the app.
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            // Switch debug mode
            if is_key_pressed(KeyCode::D) {
                debug_mode = !debug_mode;
            }

            // Update player position
            velocity = velocity.normalize_or_zero();
            velocity = velocity * player.speed * delta_time;

            player.position += velocity;

            player.position.x = clamp(player.position.x, player.size, screen_width() - player.size);
            player.position.y = clamp(
                player.position.y,
                player.size,
                screen_height() - player.size,
            );

            // Update mobs position
            for mob in &mut mobs {
                mob.position.y += mob.speed * delta_time;
            }

            // Remove mobs not visible on screen
            mobs.retain(|mob| mob.position.y < screen_height() + mob.size);
        }

        // Render entities
        draw_circle(
            player.position.x,
            player.position.y,
            player.size,
            CIRCLE_COLOR,
        );

        for mob in &mobs {
            draw_rectangle(
                mob.position.x - (mob.size * 0.5),
                mob.position.y - (mob.size * 0.5),
                mob.size,
                mob.size,
                mob.color,
            );
        }

        if debug_mode {
            // Debug player hitbox
            draw_rectangle(
                player.rect().x,
                player.rect().y,
                player.rect().w,
                player.rect().h,
                Color::from_rgba(0, 255, 0, 122),
            );

            // Debug mob hitbox
            for mob in &mobs {
                draw_rectangle(
                    mob.rect().x,
                    mob.rect().y,
                    mob.rect().w,
                    mob.rect().h,
                    Color::from_rgba(255, 0, 0, 122),
                );
            }
        }

        next_frame().await;
    }
}

fn configuration_window() -> Conf {
    Conf {
        window_title: "Macroquad Tutorial".to_string(),
        ..Default::default()
    }
}
