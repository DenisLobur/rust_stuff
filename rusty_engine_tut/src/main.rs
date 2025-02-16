use rand::prelude::*;
use rusty_engine::prelude::{bevy::utils::tracing::event, *};

#[derive(Resource)]
struct GameState {
    highscore: u32,
    score: u32,
    ferris_index: i32,
    // enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            highscore: 0,
            score: 0,
            ferris_index: 0,
            // enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        title: "Rusty Engine Tutorial".into(),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.1);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = UP;
    player.scale = 1.0;
    player.layer = 0.0;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let highscore = game.add_text("high_score", "High Score: 0");
    highscore.translation = Vec2::new(-520.0, 320.0);

    let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    car1.translation = Vec2::new(300.0, 0.0);
    car1.collision = true;

    // let temp = game.add_sprite("temp", SpritePreset::RacingCarRed);
    // temp.translation = Vec2::new(3.0, 0.0);
    // temp.layer = 1.0;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    //quit is Q is pressed
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }

    //keep text near the edges of the screen
    let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 100.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 100.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    engine.show_colliders = true;
    // handle collisions
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Current Score: {}", game_state.score);
            if game_state.score > game_state.highscore {
                game_state.highscore = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.highscore);
            }
            engine.audio_manager.play_sfx(SfxPreset::Minimize2, 0.3);
        }
    }

    // handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    //handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("ferris_{}", game_state.ferris_index);
            game_state.ferris_index += 1;
            let ferris = engine.add_sprite(label.clone(), "sprite\\racing\\barrel_blue.png");
            ferris.translation = mouse_location;
            ferris.collision = true;
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris_{}", game_state.ferris_index);
        game_state.ferris_index += 1;
        let ferris = engine.add_sprite(label.clone(), "sprite\\racing\\barrel_blue.png");
        ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
        ferris.translation.y = thread_rng().gen_range(-325.0..325.0);
        ferris.collision = true;
    }

    //reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }
}
