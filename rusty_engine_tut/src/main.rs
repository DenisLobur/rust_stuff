use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    highscore: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            highscore: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = UP;
    player.scale = 1.0;
    player.layer = 0.0;

    // let temp = game.add_sprite("temp", SpritePreset::RacingCarRed);
    // temp.translation = Vec2::new(3.0, 0.0);
    // temp.layer = 1.0;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game_state.current_score += 1;   
    // println!("Current Score: {}", game_state.current_score);
}
