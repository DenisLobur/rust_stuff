use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: u32,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();

    // game setup goes here
    game.window_settings(Window {
        title: "Car Shoot".into(),
        ..Default::default()
    });

    let game_state = GameState {
        marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
        cars_left: 25,
        spawn_timer: Timer::from_seconds(0.0, TimerMode::Once),
    };

    let player = game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    let cars_left = game.add_text("cars_left", "Cars left: 25");
    cars_left.value = format!("Cars left: {}", game_state.cars_left);
    cars_left.translation = Vec2::new(540.0, -320.0);

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.1);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
}
