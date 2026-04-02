use ::glam::Vec2;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

struct Shape {
    size: f32,
    speed: f32,
    position: Vec2,
    color: Color,
    collided: bool,
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

    fn circle(&self) -> Circle {
        Circle {
            x: self.position.x,
            y: self.position.y,
            r: self.size,
        }
    }

    #[allow(dead_code)]
    fn collides_with_rect(&self, other: &Self) -> bool {
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

    let font = load_ttf_font("./assets/fonts/Blazma-Regular.ttf")
        .await
        .unwrap();

    let mut game_over: bool = false;
    let mut debug_mode: bool = false;

    let mut timer = Timer {
        duration: 0.3,
        ..Default::default()
    };

    timer.current_timer = timer.duration;

    let mut mobs: Vec<Shape> = vec![];
    let mut bullets: Vec<Shape> = vec![];

    let mut player = Shape {
        size: 16.0,
        speed: 250.0,
        position: Vec2::new(screen_width(), screen_height()) * 0.5,
        color: Color::from_hex(0xEB5E28),
        collided: false,
    };

    let mut velocity: Vec2;

    const CIRCLE_COLOR: Color = Color::from_hex(0xF9A03F);

    loop {
        clear_background(Color::from_hex(0xFFFCF2));

        // Break out of the loop therefore quitting the app.
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Switch debug mode
        if is_key_pressed(KeyCode::D) {
            debug_mode = !debug_mode;
        }

        if !game_over {
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
                    collided: false,
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

            if is_key_pressed(KeyCode::Space) {
                println!("SHOOT!");
                bullets.push(Shape {
                    size: 8.0,
                    speed: player.speed * 2.0,
                    position: player.position,
                    color: player.color,
                    collided: false,
                });
            }

            // Update mobs position
            for mob in &mut mobs {
                mob.position.y += mob.speed * delta_time;
            }

            // Update bullets position
            for bullet in &mut bullets {
                bullet.position -= Vec2::new(0.0, 1.0) * bullet.speed * delta_time;
            }

            // Check collision with player
            if mobs
                .iter()
                .any(|mob| player.circle().overlaps_rect(&mob.rect()))
            {
                game_over = true;
            }

            // Check collision between bullets and mobs
            for bullet in &mut bullets {
                for mob in &mut mobs {
                    if bullet.circle().overlaps_rect(&mob.rect()) {
                        bullet.collided = true;
                        mob.collided = true;
                    }
                }
            }

            // Remove bullets not visible on screen
            bullets.retain(|bullet| bullet.position.y > -bullet.size);

            // Remove mobs not visible on screen
            mobs.retain(|mob| mob.position.y < screen_height() + mob.size);

            // Remove bullets and mobs which have collided
            mobs.retain(|mob| !mob.collided);
            bullets.retain(|bullet| !bullet.collided);
        }

        // Draw player
        draw_circle(
            player.position.x,
            player.position.y,
            player.size,
            CIRCLE_COLOR,
        );

        // Draw mobs
        for mob in &mobs {
            draw_rectangle(
                mob.position.x - (mob.size * 0.5),
                mob.position.y - (mob.size * 0.5),
                mob.size,
                mob.size,
                mob.color,
            );
        }

        // Draw bullets
        for bullet in &bullets {
            draw_circle(
                bullet.position.x,
                bullet.position.y,
                bullet.size,
                bullet.color,
            );
        }

        if game_over {
            let text = "GAME OVER";
            let font_size: u16 = 48;
            let text_dimensions = measure_text(text, Some(&font), font_size, 1.0);
            draw_text_ex(
                text,
                (screen_width() * 0.5) - (text_dimensions.width * 0.5),
                (screen_height() * 0.5) - (text_dimensions.height * 0.5),
                TextParams {
                    font: Some(&font),
                    font_size,
                    font_scale: 1.0,
                    color: Color::from_hex(0xEB5E28),
                    ..Default::default()
                },
            );

            if is_key_pressed(KeyCode::Space) {
                // Reset the player's position
                player.position = Vec2 {
                    x: screen_width(),
                    y: screen_height(),
                } * 0.5;

                // Remove all instances of mobs and bullets
                mobs.clear();
                bullets.clear();

                game_over = false;
            }
        }

        if debug_mode {
            // Debug player hitbox
            draw_circle(
                player.circle().x,
                player.circle().y,
                player.circle().r,
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
            for bullet in &bullets {
                draw_circle(
                    bullet.circle().x,
                    bullet.circle().y,
                    bullet.size,
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
