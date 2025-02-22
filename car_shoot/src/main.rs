use rand::prelude::IteratorRandom;
use rand::{thread_rng, Rng};
use rusty_engine::prelude::*;

const MARBLE_SPEED: f32 = 600.0;
const CAR_SPEED: f32 = 250.0;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: u32,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();
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
    let player = engine.sprites.get_mut("player").unwrap();
    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = location.x;
    }
    let player_x = player.translation.x;

    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(marble) = game_state.marble_labels.pop() {
            let marble = engine.add_sprite(marble, SpritePreset::RollingBallBlue);
            marble.translation.x = player_x;
            marble.translation.y = -275.0;
            marble.layer = 5.0;
            marble.collision = true;

            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
        };
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("marble") {
            sprite.translation.y += MARBLE_SPEED * engine.delta_f32;
        }
    }

    let mut labels_to_delete: Vec<String> = vec![];
    for (label, sprite) in engine.sprites.iter() {
        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            labels_to_delete.push(label.clone());
        }
    }

    labels_to_delete.iter().for_each(|label| {
        engine.sprites.get_mut(label);
        if label.starts_with("marble") {
            game_state.marble_labels.push(label.clone());
        }
    });

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        game_state.spawn_timer =
            Timer::from_seconds(thread_rng().gen_range(0.1..1.25), TimerMode::Once);

        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;
            let cars_left = engine.texts.get_mut("cars_left").unwrap();
            cars_left.value = format!("Cars left: {}", game_state.cars_left);

            let car_label = format!("car{}", game_state.cars_left);
            let car_choices = vec![
                SpritePreset::RacingCarBlue,
                SpritePreset::RacingCarGreen,
                SpritePreset::RacingCarRed,
                SpritePreset::RacingCarBlack,
                SpritePreset::RacingCarYellow,
            ];

            let sprite_preset = car_choices
                .iter()
                .choose(&mut thread_rng())
                .unwrap()
                .clone();
            let car = engine.add_sprite(car_label, sprite_preset);
            car.translation.x = -740.0;
            car.translation.y = thread_rng().gen_range(-100.0..325.0);
            car.collision = true;
        }
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("car") {
            sprite.translation.x += CAR_SPEED * engine.delta_f32;
        }
    }

    for event in engine.collision_events.drain(..) {
        if event.state.is_end() {
            continue;
        }
        if !event.pair.one_starts_with("marble") {
            continue;
        }

        let label = event.pair;
        engine.sprites.remove(&label.0);
        engine.sprites.remove(&label.1);

        if let Some(marble_label) = [label.0, label.1].iter().find(|&l| l.starts_with("marble")) {
            game_state.marble_labels.push(marble_label.clone());
        }

        engine.audio_manager.play_sfx(SfxPreset::Confirmation1, 0.2);
    }
}
