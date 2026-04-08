use ::glam::Vec2;
use macroquad::audio::{PlaySoundParams, load_sound, play_sound, set_sound_volume};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad_particles::*;
use std::fs;

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

enum GameState {
    MainMenu,
    Paused,
    Playing,
    GameOver,
}

#[macroquad::main(configuration_window)]
async fn main() {
    // Set the random number'seed on the current time
    // Doing so produce different numbers every time the game is run.
    rand::srand(miniquad::date::now() as u64);

    let mut game_state: GameState = GameState::MainMenu;

    let font = load_ttf_font("./assets/fonts/Blazma-Regular.ttf")
        .await
        .unwrap();

    let music = load_sound("./assets/sounds/music.ogg").await.unwrap();
    let sound_explosion = load_sound("./assets/sounds/explosion.wav").await.unwrap();
    let sound_bullet = load_sound("./assets/sounds/bullet.wav").await.unwrap();

    let mut debug_mode: bool = false;
    let mut new_high_score: bool = false;

    let mut score: u32 = 0;
    let mut high_score: u32 = fs::read_to_string("high_score.dat")
        .map_or(Ok(0), |success| success.parse::<u32>())
        .unwrap_or(0);

    // Set timer instances
    let mut spawn_timer = Timer {
        duration: 0.3,
        ..Default::default()
    };
    spawn_timer.current_timer = spawn_timer.duration;

    let mut press_start_timer: Timer = Timer {
        duration: 0.4,
        ..Default::default()
    };
    press_start_timer.current_timer = press_start_timer.duration;
    let mut display_press_start: bool = true;

    let mut high_score_timer: Timer = Timer {
        duration: 0.4,
        ..Default::default()
    };
    high_score_timer.current_timer = high_score_timer.duration;
    let mut display_high_score: bool = true;

    let mut mobs: Vec<Shape> = vec![];

    let mut bullets: Vec<Shape> = vec![];
    let mut bullet_ready: bool = true;
    let mut last_bullet_fired: f64 = 0.0;
    const COOLDOWN_BULLET: f64 = 0.2;

    let mut explosions: Vec<(Emitter, Vec2)> = vec![];

    const CIRCLE_COLOR: Color = Color::from_hex(0xEB5E28);
    let mut player = Shape {
        size: 16.0,
        speed: 250.0,
        position: Vec2::new(screen_width(), screen_height()) * 0.5,
        color: CIRCLE_COLOR,
        collided: false,
    };

    let mut player_thruster: Emitter = Emitter::new(EmitterConfig {
        local_coords: false,
        one_shot: false,
        lifetime: 0.6,
        lifetime_randomness: 0.2,
        explosiveness: 0.5,
        amount: (player.size * 2.0) as u32,
        shape: ParticleShape::Circle {
            subdivisions: player.size as u32,
        },
        emitting: true,
        initial_direction: vec2(0.0, 1.0),
        initial_direction_spread: 0.25 * std::f32::consts::PI,
        initial_velocity: player.speed,
        colors_curve: ColorCurve {
            start: player.color,
            mid: player.color.with_alpha(0.5),
            end: player.color.with_alpha(0.0),
        },

        ..Default::default()
    });

    let mut velocity: Vec2;

    play_sound(
        &music,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    loop {
        clear_background(Color::from_hex(0xFFFCF2));

        #[allow(clippy::single_match)]
        match game_state {
            GameState::MainMenu => {
                set_sound_volume(&music, 0.1);

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }

                let title = "RUSTYRUGA";
                let title_dimensions = measure_text(title, Some(&font), 48, 1.0);
                draw_text_ex(
                    title,
                    (screen_width() * 0.5) - (title_dimensions.width * 0.5),
                    (screen_height() * 0.5) - (title_dimensions.height * 0.5),
                    TextParams {
                        font: Some(&font),
                        font_size: 48,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );

                let credit = "Joann Faseler - 2026";
                let credit_dimensions = measure_text(credit, Some(&font), 16, 1.0);
                draw_text_ex(
                    credit,
                    (screen_width() * 0.5) - (credit_dimensions.width * 0.5),
                    screen_height() - (credit_dimensions.height * 3.0),
                    TextParams {
                        font: Some(&font),
                        font_size: 16,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );

                if display_press_start {
                    let sub_title = "Press Start";
                    let sub_title_dimensions = measure_text(sub_title, Some(&font), 32, 1.0);
                    draw_text_ex(
                        sub_title,
                        (screen_width() * 0.5) - (sub_title_dimensions.width * 0.5),
                        (screen_height() * 0.5) - (sub_title_dimensions.height * 0.5)
                            + (title_dimensions.height * 2.0),
                        TextParams {
                            font: Some(&font),
                            font_size: 32,
                            font_scale: 1.0,
                            color: CIRCLE_COLOR,
                            ..Default::default()
                        },
                    );
                }
                if press_start_timer.current_timer > 0.0 {
                    press_start_timer.current_timer -= get_frame_time();
                } else {
                    display_press_start = !display_press_start;
                    press_start_timer.current_timer = press_start_timer.duration;
                }
            }
            GameState::Paused => {
                set_sound_volume(&music, 0.1);

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }

                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }

                // Draw player
                draw_circle(
                    player.position.x,
                    player.position.y,
                    player.size,
                    player.color,
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

                // Semi-transparent filter applied on instances in GameOver and Paused mode
                draw_rectangle_ex(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    DrawRectangleParams {
                        color: Color::from_rgba(255, 255, 255, 180),
                        ..Default::default()
                    },
                );

                let pause = "PAUSE";
                let pause_dimensions = measure_text(pause, Some(&font), 48, 1.0);
                draw_text_ex(
                    pause,
                    (screen_width() * 0.5) - (pause_dimensions.width * 0.5),
                    (screen_height() * 0.5) - (pause_dimensions.height * 0.5),
                    TextParams {
                        font: Some(&font),
                        font_size: 48,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );
                let font_size = 24;
                let center_continue = "Press SPACE to CONTINUE";
                let center_continue_dimensions =
                    measure_text(center_continue, Some(&font), font_size, 1.0);
                draw_text_ex(
                    center_continue,
                    (screen_width() * 0.5) - (center_continue_dimensions.width * 0.5),
                    (screen_height() * 0.5) + (pause_dimensions.height * 2.0),
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );

                let center_quit = "Press ESCAPE to QUIT";
                let center_quit_dimensions = measure_text(center_quit, Some(&font), font_size, 1.0);
                draw_text_ex(
                    center_quit,
                    (screen_width() * 0.5) - (center_quit_dimensions.width * 0.5),
                    (screen_height() * 0.5)
                        + (pause_dimensions.height * 2.0)
                        + (center_continue_dimensions.height * 3.0),
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );
            }
            GameState::Playing => {
                set_sound_volume(&music, 0.7);

                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Paused;
                }

                // Switch debug mode
                if is_key_pressed(KeyCode::D) {
                    debug_mode = !debug_mode;
                }

                let delta_time = get_frame_time();
                velocity = Vec2::ZERO;

                // Generate mob
                if spawn_timer.current_timer > 0.0 {
                    spawn_timer.current_timer -= delta_time;
                }

                if spawn_timer.current_timer <= 0.0 {
                    let size = gen_range(16.0, 64.0);
                    let speed = gen_range(100.0, 200.0);
                    let position =
                        Vec2::new(gen_range(size * 0.5, screen_width() - size * 0.5), -size);
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

                    spawn_timer.current_timer = spawn_timer.duration;
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

                player.position.x =
                    clamp(player.position.x, player.size, screen_width() - player.size);
                player.position.y = clamp(
                    player.position.y,
                    player.size,
                    screen_height() - player.size,
                );

                // Spawn bullets
                if is_key_down(KeyCode::Space) && bullet_ready {
                    bullets.push(Shape {
                        size: 8.0,
                        speed: player.speed * 2.0,
                        position: player.position,
                        color: player.color,
                        collided: false,
                    });

                    bullet_ready = false;
                    last_bullet_fired = get_time();

                    play_sound(
                        &sound_bullet,
                        PlaySoundParams {
                            looped: false,
                            volume: rand::gen_range(0.5, 1.0),
                        },
                    );
                }

                // Cooldown bullets
                if !bullet_ready && (get_time() - last_bullet_fired >= COOLDOWN_BULLET) {
                    bullet_ready = true;
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
                    // Update high_score.dat file if new high score
                    if score == high_score {
                        fs::write("high_score.dat", high_score.to_string()).ok();
                        new_high_score = true;
                    }

                    game_state = GameState::GameOver;
                }

                // Check collision between bullets and mobs
                for bullet in &mut bullets {
                    for mob in &mut mobs {
                        if bullet.circle().overlaps_rect(&mob.rect()) {
                            // Flag both instances if they collided
                            bullet.collided = true;
                            mob.collided = true;

                            // Update the score
                            score += mob.size.round() as u32;
                            high_score = high_score.max(score);

                            // Create a particle explosion
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    local_coords: false,
                                    one_shot: true,
                                    lifetime: 0.7,
                                    lifetime_randomness: 0.3,
                                    explosiveness: 0.5,
                                    amount: (mob.size * 1.2) as u32,
                                    initial_direction_spread: std::f32::consts::PI * 2.0,
                                    initial_velocity: mob.speed,
                                    initial_velocity_randomness: 0.8,
                                    size: mob.size * 0.2,
                                    size_randomness: 0.8,
                                    size_curve: Some(Curve {
                                        points: vec![(0.0, 1.0), (1.0, 0.0)],
                                        ..Default::default()
                                    }),
                                    colors_curve: ColorCurve {
                                        start: mob.color,
                                        mid: mob.color.with_alpha(0.5),
                                        end: mob.color.with_alpha(0.0),
                                    },
                                    ..Default::default()
                                }),
                                Vec2::new(mob.position.x, mob.position.y),
                            ));

                            // Play an explosion sound
                            play_sound(
                                &sound_explosion,
                                PlaySoundParams {
                                    looped: false,
                                    volume: rand::gen_range(0.5, 1.0),
                                },
                            );
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

                // Remove particle explosion which have been emitted
                // Once emitted, the <emitting> field will automatically switch to <false>
                // That is because the EmitterConfig <one_shot> field is set to <true>
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                // Draw player
                draw_circle(
                    player.position.x,
                    player.position.y,
                    player.size,
                    player.color,
                );

                // Draw thrusters
                player_thruster.draw(vec2(
                    player.position.x,
                    player.position.y + (player.size * 0.5),
                ));

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

                // Draw particle explosions
                for (explosion, coordinates) in explosions.iter_mut() {
                    explosion.draw(vec2(coordinates.x, coordinates.y));
                }

                // Draw score && high score
                let font_size = 24;
                let score_label = format!("SCORE: {}", score);
                let score_dimensions = measure_text(&score_label, Some(&font), font_size, 1.0);
                let offset: f32 = 10.0;
                draw_text_ex(
                    &score_label,
                    offset,
                    score_dimensions.height + offset,
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: player.color,
                        ..Default::default()
                    },
                );

                let high_score_label = format!("HIGHSCORE: {}", high_score);
                let high_score_dimensions =
                    measure_text(&high_score_label, Some(&font), font_size, 1.0);
                draw_text_ex(
                    &high_score_label,
                    screen_width() - high_score_dimensions.width - offset,
                    high_score_dimensions.height + offset,
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: player.color,
                        ..Default::default()
                    },
                );

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
            }
            GameState::GameOver => {
                set_sound_volume(&music, 0.1);

                // Draw player
                draw_circle(
                    player.position.x,
                    player.position.y,
                    player.size,
                    player.color,
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

                // Semi-transparent filter applied on instances in GameOver and Paused mode
                draw_rectangle_ex(
                    0.0,
                    0.0,
                    screen_width(),
                    screen_height(),
                    DrawRectangleParams {
                        color: Color::from_rgba(255, 255, 255, 180),
                        ..Default::default()
                    },
                );

                let gm_text = "GAME OVER";
                let font_size: u16 = 48;
                let gm_dimensions = measure_text(gm_text, Some(&font), font_size, 1.0);
                draw_text_ex(
                    gm_text,
                    (screen_width() * 0.5) - (gm_dimensions.width * 0.5),
                    (screen_height() * 0.5) - (gm_dimensions.height * 0.5),
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: player.color,
                        ..Default::default()
                    },
                );

                if new_high_score {
                    if display_high_score {
                        let hs_text = "NEW HIGH SCORE!";
                        let hs_dimensions = measure_text(hs_text, Some(&font), 32, 1.0);
                        let padding: f32 = 10.0;
                        draw_text_ex(
                            hs_text,
                            (screen_width() * 0.5) - (hs_dimensions.width * 0.5),
                            (screen_height() * 0.5)
                                - (hs_dimensions.height * 0.5)
                                - (gm_dimensions.height * 2.0)
                                - padding,
                            TextParams {
                                font: Some(&font),
                                font_size: 32,
                                font_scale: 1.0,
                                color: player.color,
                                ..Default::default()
                            },
                        );
                    }

                    if high_score_timer.current_timer >= 0.0 {
                        high_score_timer.current_timer -= get_frame_time();
                    } else {
                        display_high_score = !display_high_score;
                        high_score_timer.current_timer = high_score_timer.duration;
                    }
                }
                let font_size = 24;
                let center_continue = "Press SPACE to RESTART";
                let center_continue_dimensions =
                    measure_text(center_continue, Some(&font), font_size, 1.0);
                draw_text_ex(
                    center_continue,
                    (screen_width() * 0.5) - (center_continue_dimensions.width * 0.5),
                    (screen_height() * 0.5) + (gm_dimensions.height * 2.0),
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );

                let center_quit = "Press ESCAPE to QUIT";
                let center_quit_dimensions = measure_text(center_quit, Some(&font), font_size, 1.0);
                draw_text_ex(
                    center_quit,
                    (screen_width() * 0.5) - (center_quit_dimensions.width * 0.5),
                    (screen_height() * 0.5)
                        + (gm_dimensions.height * 2.0)
                        + (center_continue_dimensions.height * 3.0),
                    TextParams {
                        font: Some(&font),
                        font_size,
                        font_scale: 1.0,
                        color: CIRCLE_COLOR,
                        ..Default::default()
                    },
                );

                if is_key_pressed(KeyCode::Space) {
                    // Reset the player's position
                    player.position = Vec2 {
                        x: screen_width(),
                        y: screen_height(),
                    } * 0.5;

                    // Remove all instances of mobs, bullets and explosions
                    mobs.clear();
                    bullets.clear();
                    explosions.clear();

                    // Reset all the timers
                    spawn_timer.current_timer = spawn_timer.duration;
                    press_start_timer.current_timer = press_start_timer.duration;
                    high_score_timer.current_timer = high_score_timer.duration;

                    // Reset the bullet cooldown
                    bullet_ready = true;

                    // Reset the score
                    score = 0;
                    new_high_score = false;

                    game_state = GameState::Playing;
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }

        next_frame().await;
    }
}

fn configuration_window() -> Conf {
    Conf {
        window_title: "Macroquad Tutorial".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
