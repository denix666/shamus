use macroquad::prelude::*;
extern crate rand;
use rand::{Rng};

mod resources;
use resources::*;

mod points;
use points::Point;

mod rooms;
use rooms::*;

mod game;
use game::*;

mod player;
use player::Player;

mod room_properties;

mod water;
use water::Water;

mod question;
use question::Question;

mod key;
use key::Key;

mod keyhole;
use keyhole::KeyHole;

mod door;
use door::Door;

fn window_conf() -> Conf {
    let mut title = String::from("Shamus v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: resources::RES_WIDTH,
        window_height: resources::RES_HEIGHT,
        ..Default::default()
    }
}

pub enum GameState {
    Intro,
    InitLevel,
    Game,
    LevelCompleted,
    LevelFailed,
    GameOver,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new().await;
    let mut points: Vec<Point> = make_room_array(1);
    let resources = Resources::new().await;
    let mut player = Player::new().await;
    let mut questions: Vec<Question> = vec![];
    let mut question_placed: bool = false;
    let mut waters: Vec<Water> = vec![];
    let mut water_placed: bool = false;
    let mut keys: Vec<Key> = vec![];
    let mut key_placed: bool = false;
    let mut keyholes: Vec<KeyHole> = vec![];
    let mut keyhole_placed: bool = false;
    let mut picked_up_keys: Vec<Key> = vec![];
    let mut doors: Vec<Door> = vec![];

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                game.level = 1;
                game.score = 0;
                game.room = 0;
                game.lives = 5;

                // Load water for all rooms
                for room in [2, 5, 22, 38, 53, 63, 69, 74, 76] {
                    waters.push(
                        Water::new().await
                    );
                    let idx = waters.len() - 1;
                    waters[idx].room = room;
                }

                // Load questions for all rooms
                for room in [0, 9, 11, 20, 22, 24, 28, 31, 33, 51, 54, 61, 62, 68, 70, 75, 80, 81] {
                    questions.push(
                        Question::new().await
                    );
                    let idx = questions.len() - 1;
                    questions[idx].room = room;
                }

                // Load keys for all rooms
                for room in [6, 18, 32, 44, 58, 79] {
                    keys.push(
                        Key::new().await
                    );
                    let key_color = match room {
                        6 => "blue",
                        18 => "gold",
                        32 => "red",
                        44 => "cyan",
                        58 => "purple",
                        79 => "green",
                        _ => "",
                    };
                    let idx = keys.len() - 1;
                    keys[idx].room = room;
                    keys[idx].key_color = key_color.to_string();
                }

                // Load keyholes for all rooms
                for room in [15, 37, 40, 47, 55, 92] {
                    keyholes.push(
                        KeyHole::new().await
                    );
                    let keyhole_color = match room {
                        15 => "blue",
                        37 => "gold",
                        40 => "purple",
                        47 => "red",
                        55 => "cyan",
                        92 => "green",
                        _ => "",
                    };
                    let idx = keyholes.len() - 1;
                    keyholes[idx].room = room;
                    keyholes[idx].keyhole_color = keyhole_color.to_string();
                }

                // Load doors for all rooms
                for room in [15, 37, 40, 47, 55, 92] {
                    doors.push(
                        Door::new(room).await
                    );
                }

                game_state = GameState::InitLevel;
            },
            GameState::InitLevel => {
                points = make_room_array(game.room);
                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_room(&points, &resources);
                player.update(get_frame_time());
                player.draw();
                player.draw_lives(game.lives);
                draw_info(resources.font, 
                          game.score.to_string().as_str(), 
                          game.room.to_string().as_str(), 
                          game.level.to_string().as_str());


                // Level fail
                for point in &mut points {
                    if let Some(_i) = player.rect.intersect(point.rect) {
                        todo!();
                    }
                }

                if player.x as i32 + 28 > RES_WIDTH {
                    points.clear();
                    game.room = room_direction(game.room, "right").room_to;
                    points = make_room_array(game.room);
                    player.x = 1.0;
                    question_placed = false;
                    water_placed = false;
                    key_placed = false;
                    keyhole_placed = false;
                }

                if player.x < 0.0 {
                    points.clear();
                    game.room = room_direction(game.room, "left").room_to;
                    points = make_room_array(game.room);
                    player.x = (RES_WIDTH - 28) as f32;
                    question_placed = false;
                    water_placed = false;
                    key_placed = false;
                    keyhole_placed = false;
                }

                if player.y < 0.0 {
                    points.clear();
                    game.room = room_direction(game.room, "up").room_to;
                    points = make_room_array(game.room);
                    player.y = (RES_HEIGHT - 28) as f32;
                    question_placed = false;
                    water_placed = false;
                    key_placed = false;
                    keyhole_placed = false;
                }

                if player.y as i32 + 24 > RES_HEIGHT {
                    points.clear();
                    game.room = room_direction(game.room, "down").room_to;
                    points = make_room_array(game.room);
                    player.y = 1.0;
                    question_placed = false;
                    water_placed = false;
                    key_placed = false;
                    keyhole_placed = false;
                }

                // QUESTIONS
                match questions.iter().position(|val| val.room == game.room) {
                    Some(idx) => {
                        while !question_placed {
                            let mut place_is_a_wall = false;
                            questions[idx].x = rand::thread_rng().gen_range(400.0..=800.0);
                            questions[idx].y = rand::thread_rng().gen_range(200.0..=500.0);
                            questions[idx].update();
                            for point in &mut points {
                                if let Some(_i) = questions[idx].rect.intersect(point.rect) {
                                    place_is_a_wall = true;
                                    break;
                                }
                            }
                            if !place_is_a_wall {
                                question_placed = true;
                            }
                        }
                        
                        questions[idx].draw();

                        if let Some(_i) = player.rect.intersect(questions[idx].rect) {
                            questions[idx].destroyed = true;
                            let _ = match rand::thread_rng().gen::<bool>() {
                                true => game.lives += 1,
                                false => game.score += 100,
                            };
                        }
                    },
                    None => {},
                };

                // WATER
                match waters.iter().position(|val| val.room == game.room) {
                    Some(idx) => {
                        while !water_placed {
                            let mut place_is_a_wall = false;
                            waters[idx].x = rand::thread_rng().gen_range(400.0..=800.0);
                            waters[idx].y = rand::thread_rng().gen_range(200.0..=500.0);
                            waters[idx].update();
                            for point in &mut points {
                                if let Some(_i) = waters[idx].rect.intersect(point.rect) {
                                    place_is_a_wall = true;
                                    break;
                                }
                            }
                            if !place_is_a_wall {
                                water_placed = true;
                            }
                        }
                        
                        waters[idx].draw();

                        if let Some(_i) = player.rect.intersect(waters[idx].rect) {
                            waters[idx].destroyed = true;
                            game.lives += 1;
                        }
                    },
                    None => {},
                };

                // KEYS
                match keys.iter().position(|val| val.room == game.room) {
                    Some(idx) => {
                        while !key_placed {
                            let mut place_is_a_wall = false;
                            keys[idx].x = rand::thread_rng().gen_range(400.0..=800.0);
                            keys[idx].y = rand::thread_rng().gen_range(200.0..=500.0);
                            keys[idx].update();
                            for point in &mut points {
                                if let Some(_i) = keys[idx].rect.intersect(point.rect) {
                                    place_is_a_wall = true;
                                    break;
                                }
                            }
                            if !place_is_a_wall {
                                key_placed = true;
                            }
                        }
                        
                        keys[idx].draw();

                        if let Some(_i) = player.rect.intersect(keys[idx].rect) {
                            keys[idx].destroyed = true;
                            let key_color = &keys[idx].key_color;
                            picked_up_keys.push(
                                Key::new().await
                            );
                            let picked_up_keys_idx = picked_up_keys.len() - 1;
                            picked_up_keys[picked_up_keys_idx].key_color = key_color.as_str().to_string();
                        }
                    },
                    None => {},
                };

                // KEYSHOLES
                match keyholes.iter().position(|val| val.room == game.room) {
                    Some(idx) => {
                        while !keyhole_placed {
                            let mut place_is_a_wall = false;
                            keyholes[idx].x = rand::thread_rng().gen_range(400.0..=800.0);
                            keyholes[idx].y = rand::thread_rng().gen_range(200.0..=500.0);
                            keyholes[idx].update();
                            for point in &mut points {
                                if let Some(_i) = keyholes[idx].rect.intersect(point.rect) {
                                    place_is_a_wall = true;
                                    break;
                                }
                            }
                            if !place_is_a_wall {
                                keyhole_placed = true;
                            }
                        }
                        
                        keyholes[idx].draw();

                        // check if we have a right key to open this door
                        if let Some(_i) = player.rect.intersect(keyholes[idx].rect) {
                            match picked_up_keys.iter().position(|val|val.key_color==keyholes[idx].keyhole_color) {
                                Some(key_idx) => {
                                    keyholes[idx].destroyed = true;
                                    picked_up_keys[key_idx].destroyed = true;
                                    
                                    match doors.iter().position(|val| val.room == game.room) {
                                        Some(idx) => {
                                            doors[idx].show_open_animation = true;
                                        },
                                        None => {},
                                    }
                                },
                                None => {}
                            }
                        }
                    },
                    None => {},
                };

                // DOORS
                match doors.iter().position(|val| val.room == game.room) {
                    Some(idx) => {
                        doors[idx].draw();
                    },
                    None => {},
                }

                // SHOW INVENTORY
                let mut keys_amount: i32 = 0;
                for key in &mut picked_up_keys {
                    keys_amount += 1;
                    key.x = (1200 - keys_amount * 42 - 3) as f32;
                    key.y = 675.0;
                    key.draw();
                }
            },
            GameState::LevelCompleted => {

            },
            GameState::LevelFailed => {
                
            },
            GameState::GameOver => {
                
            },
        };

        // GC
        match questions.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                questions.remove(idx);
            },
            None => {},
        };

        match waters.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                waters.remove(idx);
            },
            None => {},
        };

        match keys.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                keys.remove(idx);
            },
            None => {},
        };

        match keyholes.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                keyholes.remove(idx);
            },
            None => {},
        };

        match picked_up_keys.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                picked_up_keys.remove(idx);
            },
            None => {},
        };

        match doors.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                doors.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}