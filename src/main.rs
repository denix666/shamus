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

mod player_bullet;
use player_bullet::PlayerBullet;

mod enemy;
use enemy::Enemy;

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

fn show_text(font: Font, header_text: &str, message_text: &str) {
    let header_dims = measure_text(header_text, Some(font), 50, 1.0);
    let message_dims = measure_text(message_text, Some(font), 20, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 50,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        280.0,
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub enum GameState {
    Intro,
    InitLevel,
    Game,
    LevelFailed,
    GameOver,
}

async fn load_enemies(points: &Vec<Point>) -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = Vec::new();
    let amount_of_enemies = rand::thread_rng().gen_range(3..=12);

    for idx in 0..=amount_of_enemies {
        let mut item_placed: bool = false;

        let enemy_type = match rand::thread_rng().gen_range(0..=3) {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => "D",
        };

        enemies.push(
            Enemy::new(0.0, 0.0, enemy_type).await
        );

        while !item_placed {
            let mut place_is_a_wall = false;

            enemies[idx].x = rand::thread_rng().gen_range(400.0..=800.0);
            enemies[idx].y = rand::thread_rng().gen_range(200.0..=500.0);
            enemies[idx].update();

            for point in points {
                if let Some(_i) = enemies[idx].rect.intersect(point.rect) {
                    place_is_a_wall = true;
                    break;
                }
            }

            if !place_is_a_wall {
                item_placed = true;
            }
        }
    }

    return enemies
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Intro;
    let mut game = Game::new().await;
    let mut points: Vec<Point> = make_room_array(1);
    let resources = Resources::new().await;
    let mut player = Player::new().await;
    let mut questions: Vec<Question> = Vec::new();
    let mut question_placed: bool = false;
    let mut waters: Vec<Water> = Vec::new();
    let mut water_placed: bool = false;
    let mut keys: Vec<Key> = Vec::new();
    let mut key_placed: bool = false;
    let mut keyholes: Vec<KeyHole> = Vec::new();
    let mut keyhole_placed: bool = false;
    let mut picked_up_keys: Vec<Key> = Vec::new();
    let mut doors: Vec<Door> = Vec::new();
    let mut intro_water = Water::new(-1).await;
    let mut intro_question = Question::new(-1).await;
    let mut intro_enemy_a = Enemy::new(690.0, 125.0, "A").await;
    let mut intro_enemy_c = Enemy::new(690.0, 185.0, "C").await;
    let mut intro_enemy_d = Enemy::new(690.0, 15.0, "D").await;
    let mut player_bullets: Vec<PlayerBullet> = Vec::new();
    let mut player_last_pos_x: f32 = resources::PLAYER_START_X_POS;
    let mut player_last_pos_y: f32 = resources::PLAYER_START_Y_POS;
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut switched_room: bool = true;
    

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro, 0.0, 0.0, WHITE);
                intro_question.draw();
                intro_water.draw();
                intro_enemy_a.draw(); intro_enemy_a.update();
                intro_enemy_c.draw(); intro_enemy_c.update();
                intro_enemy_d.draw(); intro_enemy_d.update();

                if is_key_pressed(KeyCode::Space) {
                    game.level = 1;
                    game.score = 0;
                    game.room = 0;
                    game.lives = 5;

                    // Load water for all rooms
                    for room in [2, 5, 22, 38, 53, 63, 69, 74, 76] {
                        waters.push(
                            Water::new(room).await
                        );
                    }

                    // Load questions for all rooms
                    for room in [0, 9, 11, 20, 22, 24, 28, 31, 33, 51, 54, 61, 62, 68, 70, 75, 80, 81] {
                        questions.push(
                            Question::new(room).await
                        );
                    }

                    // Load keys for all rooms
                    for room in [6, 18, 32, 44, 58, 79] {
                        let key_color = match room {
                            6 => "blue",
                            18 => "gold",
                            32 => "red",
                            44 => "cyan",
                            58 => "purple",
                            79 => "green",
                            _ => "",
                        };
                        keys.push(
                            Key::new(room, key_color).await
                        );
                    }

                    // Load keyholes for all rooms
                    for room in [15, 37, 40, 47, 55, 92] {
                        let keyhole_color = match room {
                            15 => "blue",
                            37 => "gold",
                            40 => "purple",
                            47 => "red",
                            55 => "cyan",
                            92 => "green",
                            _ => "",
                        };
                        keyholes.push(
                            KeyHole::new(room, keyhole_color).await
                        );
                    }

                    // Load doors for all rooms
                    for room in [15, 37, 40, 47, 55, 92] {
                        doors.push(
                            Door::new(room).await
                        );
                    }

                    game_state = GameState::InitLevel;
                }
            },
            GameState::InitLevel => {
                points = make_room_array(game.room);
                game_state = GameState::Game;
            },
            GameState::Game => {
                draw_room(&points, &resources);
                player.draw_lives(game.lives);
                
                draw_info(resources.font, 
                          game.score.to_string().as_str(), 
                          game.room.to_string().as_str(), 
                          game.level.to_string().as_str());

                if player.x as i32 + 28 > RES_WIDTH {
                    points.clear();
                    game.room = room_direction(game.room, "right").room_to;
                    points = make_room_array(game.room);
                    player.x = 1.0;
                    switched_room = true;
                }

                if player.x < 0.0 {
                    points.clear();
                    game.room = room_direction(game.room, "left").room_to;
                    points = make_room_array(game.room);
                    player.x = (RES_WIDTH - 28) as f32;
                    switched_room = true;
                }

                if player.y < 0.0 {
                    points.clear();
                    game.room = room_direction(game.room, "up").room_to;
                    points = make_room_array(game.room);
                    player.y = (RES_HEIGHT - 28) as f32;
                    switched_room = true;
                }

                if player.y as i32 + 24 > RES_HEIGHT {
                    points.clear();
                    game.room = room_direction(game.room, "down").room_to;
                    points = make_room_array(game.room);
                    player.y = 1.0;
                    switched_room = true;
                }

                if switched_room {
                    question_placed = false;
                    water_placed = false;
                    key_placed = false;
                    keyhole_placed = false;
                    player_last_pos_x = player.x;
                    player_last_pos_y = player.y;
                    enemies = load_enemies(&points).await;

                    switched_room = false;
                }

                for enemy in &mut enemies {
                    enemy.update();
                    enemy.draw();
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
                                Key::new(-1, key_color).await
                            );
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

                player.update(get_frame_time());
                player.draw();

                // Level fail
                for point in &mut points {
                    if let Some(_i) = player.rect.intersect(point.rect) {
                        game_state = GameState::LevelFailed;
                    }
                }

                if is_key_pressed(KeyCode::LeftAlt) {
                    if player_bullets.len() <= 3 && player.dir != player::Dir::Idle {
                        let dir = match player.dir {
                            player::Dir::Up=>{player_bullet::PlayerBulletDir::Up},
                            player::Dir::Down => {player_bullet::PlayerBulletDir::Down},
                            player::Dir::Left => {player_bullet::PlayerBulletDir::Left},
                            player::Dir::Right => {player_bullet::PlayerBulletDir::Right},
                            player::Dir::UpLeft => {player_bullet::PlayerBulletDir::UpLeft},
                            player::Dir::UpRight => {player_bullet::PlayerBulletDir::UpRight},
                            player::Dir::DownLeft => {player_bullet::PlayerBulletDir::DownLeft},
                            player::Dir::DownRight => {player_bullet::PlayerBulletDir::DownRight},
                            player::Dir::Idle => {player_bullet::PlayerBulletDir::Idle}, 
                        };

                        player_bullets.push(
                            PlayerBullet::new(player.x, player.y, dir).await,
                        );
                    }
                }

                for player_bullet in &mut player_bullets {
                    player_bullet.draw();
                }
            },
            GameState::LevelFailed => {
                if game.lives > 0 {
                    game.lives -= 1;
                    player.x = player_last_pos_x;
                    player.y = player_last_pos_y;
                    player_bullets.clear();
                    game_state = GameState::Game;
                } else {
                    game_state = GameState::GameOver;
                }
            },
            GameState::GameOver => {
                draw_room(&points, &resources);

                draw_info(resources.font, 
                          game.score.to_string().as_str(), 
                          game.room.to_string().as_str(), 
                          game.level.to_string().as_str());

                show_text(resources.font, "GAME OVER", "press 'space' to continue...");

                if is_key_pressed(KeyCode::Space) {
                    player.x = resources::PLAYER_START_X_POS;
                    player.y = resources::PLAYER_START_Y_POS;
                    switched_room = true;
                    game_state = GameState::Intro;
                }
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

        match player_bullets.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                player_bullets.remove(idx);
            },
            None => {},
        };

        match enemies.iter().position(|val| val.destroyed == true) {
            Some(idx) => {
                enemies.remove(idx);
            },
            None => {},
        };

        next_frame().await
    }
}