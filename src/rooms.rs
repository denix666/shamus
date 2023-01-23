use macroquad::prelude::*;

use crate::{points::Point, resources::Resources};

pub fn draw_info(font: Font, score: &str, room: &str, level: &str) {
    draw_text_ex("SCORE: ", 2.0, 655.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 95.0, 655.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("ROOM: ", 2.0, 675.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(room, 85.0, 675.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );

    draw_text_ex("LEVEL: ", 2.0, 695.0, 
        TextParams {
            font,
            font_size: 20,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(level, 95.0, 695.0, 
        TextParams {
            font,
            font_size: 20,
            color: ORANGE,
            ..Default::default()
        },
    );
}

pub fn draw_room(points: &Vec<Point>, resources: &Resources) {
    for point in points {
        match point.value.as_str() {
            "a" => {
                draw_texture(resources.bg_a, point.x, point.y, WHITE);
            },
            "b" => {
                draw_texture(resources.bg_b, point.x, point.y, WHITE);
            },
            "c" => {
                draw_texture(resources.bg_c, point.x, point.y, WHITE);
            },
            "d" => {
                draw_texture(resources.bg_d, point.x, point.y, WHITE);
            },
            "e" => {
                draw_texture(resources.bg_e, point.x, point.y, WHITE);
            },
            "f" => {
                draw_texture(resources.bg_f, point.x, point.y, WHITE);
            },
            "g" => {
                draw_texture(resources.bg_g, point.x, point.y, WHITE);
            },
            "i" => {
                draw_texture(resources.bg_i, point.x, point.y, WHITE);
            },
            "j" => {
                draw_texture(resources.bg_j, point.x, point.y, WHITE);
            },
            "k" => {
                draw_texture(resources.bg_k, point.x, point.y, WHITE);
            },
            "r" => {
                draw_texture(resources.bg_r, point.x, point.y, WHITE);
            },
            "u" => {
                draw_texture(resources.bg_u, point.x, point.y, WHITE);
            },
            _ => {},
        };
    }
}

pub fn room_direction(room: i32, from_side: &str) -> crate::room_properties::RoomProperty {
    let mut property = crate::room_properties::RoomProperty::new();
    
    match (room, from_side) {
        (0, "right") => {
            property.room_to = 1;
        },
        (1, "left") => {
            property.room_to = 0;
        },
        (1, "right") => {
            property.room_to = 2;
        },
        (1, "down") => {
            property.room_to = 8;
        },
        (2, "left") => {
            property.room_to = 1;
        },
        (2, "right") => {
            property.room_to = 3;
        },
        (3, "left") => {
            property.room_to = 2;
        },
        (3, "up") => {
            property.room_to = 4;
        },
        (4, "right") => {
            property.room_to = 5;
        },
        (4, "down") => {
            property.room_to = 3;
        },
        (5, "left") => {
            property.room_to = 4;
        },
        (5, "right") => {
            property.room_to = 6;
        },
        (6, "right") => {
            property.room_to = 7;
        },
        (6, "left") => {
            property.room_to = 5;
        },
        (7, "left") => {
            property.room_to = 6;
        },
        (7, "down") => {
            property.room_to = 36;
        },
        (8, "up") => {
            property.room_to = 1;
        },
        (8, "right") => {
            property.room_to = 9;
        },
        (9, "right") => {
            property.room_to = 10;
        },
        (9, "left") => {
            property.room_to = 8;
        },
        (10, "left") => {
            property.room_to = 9;
        },
        (10, "right") => {
            property.room_to = 11;
        },
        (10, "down") => {
            property.room_to = 16;
        },
        (11, "left") => {
            property.room_to = 10;
        },
        (11, "right") => {
            property.room_to = 12;
        },
        (12, "left") => {
            property.room_to = 11;
        },
        (12, "down") => {
            property.room_to = 25;
        },
        (13, "down") => {
            property.room_to = 19;
        },
        (13, "right") => {
            property.room_to = 14;
        },
        (14, "right") => {
            property.room_to = 15;
        },
        (14, "left") => {
            property.room_to = 13;
        },
        (14, "down") => {
            property.room_to = 21;
        },
        (15, "left") => {
            property.room_to = 14;
        },
        (15, "right") => {
            property.room_to = 16;
        },
        (16, "up") => {
            property.room_to = 10;
        },
        (16, "left") => {
            property.room_to = 15;
        },
        (17, "up") => {
            property.room_to = 22;
        },
        (17, "right") => {
            property.room_to = 18;
        },
        (18, "right") => {
            property.room_to = 19;
        },
        (18, "left") => {
            property.room_to = 17;
        },
        (19, "up") => {
            property.room_to = 13;
        },
        (19, "left") => {
            property.room_to = 18;
        },
        (19, "right") => {
            property.room_to = 20;
        },
        (20, "right") => {
            property.room_to = 21;
        },
        (20, "left") => {
            property.room_to = 19;
        },
        (21, "left") => {
            property.room_to = 20;
        },
        (21, "up") => {
            property.room_to = 14;
        },
        (22, "down") => {
            property.room_to = 17;
        },
        (23, "down") => {
            property.room_to = 30;
        },
        (23, "right") => {
            property.room_to = 24;
        },
        (24, "right") => {
            property.room_to = 25;
        },
        (24, "left") => {
            property.room_to = 23;
        },
        (25, "left") => {
            property.room_to = 24;
        },
        (25, "right") => {
            property.room_to = 26;
        },
        (25, "up") => {
            property.room_to = 12;
        },
        (26, "right") => {
            property.room_to = 27;
        },
        (26, "left") => {
            property.room_to = 25;
        },
        (27, "left") => {
            property.room_to = 26;
        },
        (27, "right") => {
            property.room_to = 28;
        },
        (27, "up") => {
            property.room_to = 35;
        },
        (28, "right") => {
            property.room_to = 29;
        },
        (28, "left") => {
            property.room_to = 27;
        },
        (29, "left") => {
            property.room_to = 28;
        },
        (29, "down") => {
            property.room_to = 34;
        },
        (30, "up") => {
            property.room_to = 23;
        },
        (30, "right") => {
            property.room_to = 31;
        },
        (31, "right") => {
            property.room_to = 32;
        },
        (31, "left") => {
            property.room_to = 30;
        },
        (32, "left") => {
            property.room_to = 31;
        },
        (32, "right") => {
            property.room_to = 33;
        },
        (33, "left") => {
            property.room_to = 32;
        },
        (33, "right") => {
            property.room_to = 34;
        },
        (34, "left") => {
            property.room_to = 33;
        },
        (34, "up") => {
            property.room_to = 29;
        },
        (35, "down") => {
            property.room_to = 27;
        },
        (35, "right") => {
            property.room_to = 36;
        },
        (36, "up") => {
            property.room_to = 07;
        },
        (36, "right") => {
            property.room_to = 37;
        },
        (36, "left") => {
            property.room_to = 35;
        },
        (37, "left") => {
            property.room_to = 36;
        },
        (37, "right") => {
            property.room_to = 38;
        },
        (38, "right") => {
            property.room_to = 39;
        },
        (38, "left") => {
            property.room_to = 37;
        },
        (39, "left") => {
            property.room_to = 38;
        },
        (39, "right") => {
            property.room_to = 40;
        },
        (39, "down") => {
            property.room_to = 48;
        },
        (40, "left") => {
            property.room_to = 39;
        },
        (40, "right") => {
            property.room_to = 41;
        },
        (41, "left") => {
            property.room_to = 40;
        },
        (41, "right") => {
            property.room_to = 42;
        },
        (41, "up") => {
            property.room_to = 43;
        },
        (42, "up") => {
            property.room_to = 45;
        },
        (42, "left") => {
            property.room_to = 41;
        },
        (43, "right") => {
            property.room_to = 44;
        },
        (43, "down") => {
            property.room_to = 41;
        },
        (44, "left") => {
            property.room_to = 43;
        },
        (44, "right") => {
            property.room_to = 45;
        },
        (45, "left") => {
            property.room_to = 44;
        },
        (45, "down") => {
            property.room_to = 42;
        },
        (46, "right") => {
            property.room_to = 47;
        },
        (46, "down") => {
            property.room_to = 50;
        },
        (47, "right") => {
            property.room_to = 48;
        },
        (47, "left") => {
            property.room_to = 46;
        },
        (48, "up") => {
            property.room_to = 39;
        },
        (48, "left") => {
            property.room_to = 47;
        },
        (49, "down") => {
            property.room_to = 57;
        },
        (49, "right") => {
            property.room_to = 50;
        },
        (50, "right") => {
            property.room_to = 51;
        },
        (50, "left") => {
            property.room_to = 49;
        },
        (50, "up") => {
            property.room_to = 46;
        },
        (51, "right") => {
            property.room_to = 52;
        },
        (51, "left") => {
            property.room_to = 50;
        },
        (52, "right") => {
            property.room_to = 53;
        },
        (52, "left") => {
            property.room_to = 51;
        },
        (52, "down") => {
            property.room_to = 59;
        },
        (53, "right") => {
            property.room_to = 54;
        },
        (53, "left") => {
            property.room_to = 52;
        },
        (54, "right") => {
            property.room_to = 55;
        },
        (54, "left") => {
            property.room_to = 53;
        },
        (55, "right") => {
            property.room_to = 56;
        },
        (55, "left") => {
            property.room_to = 54;
        },
        (56, "down") => {
            property.room_to = 56; // to next level
        },
        (56, "left") => {
            property.room_to = 55;
        },
        (57, "right") => {
            property.room_to = 58;
        },
        (57, "up") => {
            property.room_to = 49;
        },
        (58, "right") => {
            property.room_to = 59;
        },
        (58, "left") => {
            property.room_to = 57;
        },
        (59, "right") => {
            property.room_to = 60;
        },
        (59, "left") => {
            property.room_to = 58;
        },
        (59, "up") => {
            property.room_to = 52;
        },
        (60, "right") => {
            property.room_to = 61;
        },
        (60, "left") => {
            property.room_to = 59;
        },
        (60, "down") => {
            property.room_to = 65;
        },
        (61, "right") => {
            property.room_to = 62;
        },
        (61, "left") => {
            property.room_to = 60;
        },
        (62, "right") => {
            property.room_to = 63;
        },
        (62, "left") => {
            property.room_to = 61;
        },
        (63, "right") => {
            property.room_to = 64;
        },
        (63, "left") => {
            property.room_to = 62;
        },
        (64, "down") => {
            property.room_to = 66;
        },
        (64, "left") => {
            property.room_to = 63;
        },
        (65, "up") => {
            property.room_to = 60;
        },
        (65, "right") => {
            property.room_to = 66;
        },
        (66, "up") => {
            property.room_to = 64;
        },
        (66, "left") => {
            property.room_to = 65;
        },
        _ => {
            panic!("no instruction where to go to...")
        }
    };
    return property
}

pub fn make_room_array(lvl_num: i32) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    let map = match lvl_num {
        0 => vec![
            "aaaaaaaarrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaa     ",
            "aaaaaaaarrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaa     ",
            "aaaaaaaarr                              rraaa     ",
            "aaaaaaaarr                              rraaa     ",
            "aaaaaaaarr                              rraaaaaaaa",
            "aaaaaaaarr                              rraaaaaaaa",
            "aaaaaaaarr                              rraaaaaaaa",
            "aaaaaaaarr                              rraaaaaaaa",
            "aaaaaaaarr                              rraaaaaaaa",
            "aaaaaaaarr                              rraaaaaaaa",
            "rrrrrrrrrr        rrrrrrrrrrrr          rrrrrrrrrr",
            "rrrrrrrrrr        rrrrrrrrrrrr          rrrrrrrrrr",
            "rr                rr        rr                    ",
            "rr                rr        rr                    ",
            "rr                rr        rr                    ",
            "rr                rr        rr                    ",
            "rr                rr        rr                    ",
            "rr                rr        rr                    ",
            "rrrrrrrrrrrrrrrrrrrr        rr          rrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrr        rr          rrrrrrrrrr",
            "aaaaaaaaaarr                            rraaaaaaaa",
            "aaaaaaaaaarr                            rraaaaaaaa",
            "aaaaaaaaaarr                            rraaaaaaaa",
            "aaaaaaaaaarr                            rraaaaaaaa",
            "aaaaaaaaaarr                            rraaaaaaaa",
            "aaaaaaaaaarr                            rraaaaaaaa",
            "        aarr                            rraaaaaaaa",
            "        aarr                            rraaaaaaaa",
            "        aarrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaa     ",
            "        aarrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaa     ",
        ],
        1 | 10 | 14 => vec![
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbb     ",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbb     ",
        ],
        2 => vec![
            "ccccccccccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
            "ccccccccccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
            "ccccccccccrr                rr        rrccccc     ",
            "ccccccccccrr                rr        rrccccc     ",
            "ccccccccccrr                rr        rrcccccccccc",
            "ccccccccccrr                rr        rrcccccccccc",
            "ccccccccccrr                rr        rrcccccccccc",
            "ccccccccccrr                rr        rrcccccccccc",
            "ccccccccccrr                rr        rrcccccccccc",
            "ccccccccccrr                rr        rrcccccccccc",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "ccccccccccrr        rr                rrcccccccccc",
            "ccccccccccrr        rr                rrcccccccccc",
            "ccccccccccrr        rr                rrcccccccccc",
            "ccccccccccrr        rr                rrcccccccccc",
            "ccccccccccrr        rr                rrcccccccccc",
            "ccccccccccrr        rr                rrcccccccccc",
            "        ccrr        rr                rrcccccccccc",
            "        ccrr        rr                rrcccccccccc",
            "        ccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
            "        ccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
        ],
        3 => vec![
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrr      rrkkkkkkkkkkkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrr      rrkkkkkkkkkkkkkkkkkkkk",
            "                            rrkkkkkkkkkkkkkkkkkkkk",
            "                            rrkkkkkkkkkkkkkkkkkkkk",
            "                            rrkkkkkkkkkkkkkkkkkkkk",
            "                            rrkkkkkkkkkkkkkkkkkkkk",
            "                            rrkkkkkkkkkkkkkkkkkkkk",
            "                            rrkkkkkkkkkkkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkkkkkkkkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
        ],
        4 | 13 | 35 => vec![
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "jjjjjjjjjjjjjjjjjjjjrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "jjjjjjjjjjjjjjjjjjjjrr                            ",
            "jjjjjjjjjjjjjjjjjjjjrr                            ",
            "jjjjjjjjjjjjjjjjjjjjrr                            ",
            "jjjjjjjjjjjjjjjjjjjjrr                            ",
            "jjjjjjjjjjjjjjjjjjjjrr                            ",
            "jjjjjjjjjjjjjjjjjjjjrr                            ",
            "jjjjjjjjjjjjjjjjjjjjrr      rrrrrrrrrrrrrrrrrrrrrr",
            "jjjjjjjjjjjjjjjjjjjjrr      rrrrrrrrrrrrrrrrrrrrrr",
            "jjjjjjjjjjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "        jjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "        jjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjjjjjjj",
            "        jjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjj     ",
            "        jjjjjjjjjjjjrr      rrjjjjjjjjjjjjjjj     ",
        ],
        5 | 31 => vec![
            "iiiiiiiiiirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
            "iiiiiiiiiirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
            "iiiiiiiiiirr                          rriiiii     ",
            "iiiiiiiiiirr                          rriiiii     ",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "        iirr                          rriiiiiiiiii",
            "        iirr                          rriiiiiiiiii",
            "        iirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
            "        iirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
        ],
        6 => vec![
            "iiiiiiiiiirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
            "iiiiiiiiiirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
            "iiiiiiiiiirr                          rriiiii     ",
            "iiiiiiiiiirr                          rriiiii     ",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "rrrrrrrrrrrr        rrrrrrrrrr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rrrrrrrrrr        rrrrrrrrrrrr",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "rrrrrrrrrrrr        rrrrrrrrrr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rrrrrrrrrr        rrrrrrrrrrrr",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "iiiiiiiiiirr                          rriiiiiiiiii",
            "        iirr                          rriiiiiiiiii",
            "        iirr                          rriiiiiiiiii",
            "        iirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
            "        iirrrrrrrrrrrrrrrrrrrrrrrrrrrrrriiiii     ",
        ],
        7 => vec![
            "fffffffffffffffffffffffffffffffffffffffffffff     ",
            "fffffffffffffffffffffffffffffffffffffffffffff     ",
            "fffffffffffffffffffffffffffffffffffffffffffff     ",
            "fffffffffffffffffffffffffffffffffffffffffffff     ",
            "ffffffffffffffffffffffffffffffffffffffffffffffffff",
            "ffffffffffffffffffffffffffffffffffffffffffffffffff",
            "ffffffffffffffffffffffffffffffffffffffffffffffffff",
            "ffffffffffffffffffffffffffffffffffffffffffffffffff",
            "ffffffffffffffffffffffffffffffffffffffffffffffffff",
            "ffffffffffffffffffffffffffffffffffffffffffffffffff",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrffffffffffffffffffff",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrffffffffffffffffffff",
            "                            rrffffffffffffffffffff",
            "                            rrffffffffffffffffffff",
            "                            rrffffffffffffffffffff",
            "                            rrffffffffffffffffffff",
            "                            rrffffffffffffffffffff",
            "                            rrffffffffffffffffffff",
            "rrrrrrrrrrrrrrrrrrrrrr      rrffffffffffffffffffff",
            "rrrrrrrrrrrrrrrrrrrrrr      rrffffffffffffffffffff",
            "ffffffffffffffffffffrr      rrffffffffffffffffffff",
            "ffffffffffffffffffffrr      rrffffffffffffffffffff",
            "ffffffffffffffffffffrr      rrffffffffffffffffffff",
            "ffffffffffffffffffffrr      rrffffffffffffffffffff",
            "ffffffffffffffffffffrr      rrffffffffffffffffffff",
            "ffffffffffffffffffffrr      rrffffffffffffffffffff",
            "        ffffffffffffrr      rrffffffffffffffffffff",
            "        ffffffffffffrr      rrffffffffffffffffffff",
            "        ffffffffffffrr      rrfffffffffffffff     ",
            "        ffffffffffffrr      rrfffffffffffffff     ",
        ],
        8 | 30 => vec![
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaa     ",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaa     ",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaa     ",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaa     ",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaarr      rraaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaarr      rrrrrrrrrrrrrrrrrrrrrr",
            "aaaaaaaaaaaaaaaaaaaarr      rrrrrrrrrrrrrrrrrrrrrr",
            "aaaaaaaaaaaaaaaaaaaarr                            ",
            "aaaaaaaaaaaaaaaaaaaarr                            ",
            "aaaaaaaaaaaaaaaaaaaarr                            ",
            "aaaaaaaaaaaaaaaaaaaarr                            ",
            "aaaaaaaaaaaaaaaaaaaarr                            ",
            "aaaaaaaaaaaaaaaaaaaarr                            ",
            "aaaaaaaaaaaaaaaaaaaarrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "aaaaaaaaaaaaaaaaaaaarrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa     ",
            "        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa     ",
        ],
        9 => vec![
            "bbbbbbbbbbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
            "bbbbbbbbbbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
            "bbbbbbbbbbrr                          rrbbbbb     ",
            "bbbbbbbbbbrr                          rrbbbbb     ",
            "bbbbbbbbbbrr                          rrbbbbbbbbbb",
            "bbbbbbbbbbrr                          rrbbbbbbbbbb",
            "bbbbbbbbbbrr                          rrbbbbbbbbbb",
            "bbbbbbbbbbrr                          rrbbbbbbbbbb",
            "bbbbbbbbbbrr                          rrbbbbbbbbbb",
            "bbbbbbbbbbrr                          rrbbbbbbbbbb",
            "rrrrrrrrrrrr        rrrrrrrrrrrr      rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rrrrrrrrrrrr      rrrrrrrrrrrr",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "rrrrrrrrrrrr        rr        rrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr        rrrrrrrrrrrrrrrrrrrr",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "        bbrr        rr                rrbbbbbbbbbb",
            "        bbrr        rr                rrbbbbbbbbbb",
            "        bbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
            "        bbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
        ],
        11 => vec![
            "bbbbbbbbbbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
            "bbbbbbbbbbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
            "bbbbbbbbbbrr        rr                rrbbbbb     ",
            "bbbbbbbbbbrr        rr                rrbbbbb     ",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr                rrbbbbbbbbbb",
            "rrrrrrrrrrrr        rr        rr      rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr        rr      rrrrrrrrrrrr",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "                              rr                  ",
            "rrrrrrrrrrrr        rr        rr      rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr        rr      rrrrrrrrrrrr",
            "bbbbbbbbbbrr        rr        rr      rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr        rr      rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr        rr      rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr        rr      rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr        rr      rrbbbbbbbbbb",
            "bbbbbbbbbbrr        rr        rr      rrbbbbbbbbbb",
            "        bbrr        rr        rr      rrbbbbbbbbbb",
            "        bbrr        rr        rr      rrbbbbbbbbbb",
            "        bbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
            "        bbrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbb     ",
        ],
        12 | 29 => vec![
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb     ",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbbbbbbbbbbbbbbbbb",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrbbbbbbbbbbbbbbbbbbbb",
            "                            rrbbbbbbbbbbbbbbbbbbbb",
            "                            rrbbbbbbbbbbbbbbbbbbbb",
            "                            rrbbbbbbbbbbbbbbbbbbbb",
            "                            rrbbbbbbbbbbbbbbbbbbbb",
            "                            rrbbbbbbbbbbbbbbbbbbbb",
            "                            rrbbbbbbbbbbbbbbbbbbbb",
            "rrrrrrrrrrrrrrrrrrrrrr      rrbbbbbbbbbbbbbbbbbbbb",
            "rrrrrrrrrrrrrrrrrrrrrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "bbbbbbbbbbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbbbbbbb",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbb     ",
            "        bbbbbbbbbbbbrr      rrbbbbbbbbbbbbbbb     ",
        ],
        15 | 37 => vec![
            "aaaaaaaaaarrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaaaa     ",
            "aaaaaaaaaarrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaaaa     ",
            "aaaaaaaaaarr                          rraaaaa     ",
            "aaaaaaaaaarr                          rraaaaa     ",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "rrrrrrrrrrrr        rr      rrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rrrrrrrrrrrrrrrrrrrrrr",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "rrrrrrrrrrrrrrrrrrrrrr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rr        rrrrrrrrrrrr",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "aaaaaaaaaarr                          rraaaaaaaaaa",
            "        aarr                          rraaaaaaaaaa",
            "        aarr                          rraaaaaaaaaa",
            "        aarrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaaaa     ",
            "        aarrrrrrrrrrrrrrrrrrrrrrrrrrrrrraaaaa     ",
        ],
        16 | 21 | 34 => vec![
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "rrrrrrrrrrrrrrrrrrrrrr      rrcccccccccccccccccccc",
            "rrrrrrrrrrrrrrrrrrrrrr      rrcccccccccccccccccccc",
            "                            rrcccccccccccccccccccc",
            "                            rrcccccccccccccccccccc",
            "                            rrcccccccccccccccccccc",
            "                            rrcccccccccccccccccccc",
            "                            rrcccccccccccccccccccc",
            "                            rrcccccccccccccccccccc",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrcccccccccccccccccccc",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrcccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "        cccccccccccccccccccccccccccccccccccccccccc",
            "        cccccccccccccccccccccccccccccccccccccccccc",
            "        ccccccccccccccccccccccccccccccccccccc     ",
            "        ccccccccccccccccccccccccccccccccccccc     ",
        ],
        17 => vec![
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiirr      rriiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiirr      rrrrrrrrrrrrrrrrrrrrrr",
            "iiiiiiiiiiiiiiiiiiiirr      rrrrrrrrrrrrrrrrrrrrrr",
            "iiiiiiiiiiiiiiiiiiiirr                            ",
            "iiiiiiiiiiiiiiiiiiiirr                            ",
            "iiiiiiiiiiiiiiiiiiiirr                            ",
            "iiiiiiiiiiiiiiiiiiiirr                            ",
            "iiiiiiiiiiiiiiiiiiiirr                            ",
            "iiiiiiiiiiiiiiiiiiiirr                            ",
            "iiiiiiiiiiiiiiiiiiiirrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "iiiiiiiiiiiiiiiiiiiirrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
        ],
        18 | 20 => vec![
            "ccccccccccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
            "ccccccccccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
            "ccccccccccrr                          rrccccc     ",
            "ccccccccccrr                          rrccccc     ",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "rrrrrrrrrrrr                          rrrrrrrrrrrr",
            "rrrrrrrrrrrr                          rrrrrrrrrrrr",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "rrrrrrrrrrrr                          rrrrrrrrrrrr",
            "rrrrrrrrrrrr                          rrrrrrrrrrrr",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "ccccccccccrr                          rrcccccccccc",
            "        ccrr                          rrcccccccccc",
            "        ccrr                          rrcccccccccc",
            "        ccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
            "        ccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrccccc     ",
        ],
        19 | 36 => vec![
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrccccccccccccccc     ",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "        cccccccccccccccccccccccccccccccccccccccccc",
            "        cccccccccccccccccccccccccccccccccccccccccc",
            "        ccccccccccccccccccccccccccccccccccccc     ",
            "        ccccccccccccccccccccccccccccccccccccc     ",
        ],
        22 => vec![
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrrrrrrrrrrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrrrrrrrrrrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "        kkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
        ],
        23 => vec![
            "ccccccccccccccccccccccccccccccccccccccccccccc     ",
            "ccccccccccccccccccccccccccccccccccccccccccccc     ",
            "ccccccccccccccccccccccccccccccccccccccccccccc     ",
            "ccccccccccccccccccccccccccccccccccccccccccccc     ",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cccccccccccccccccccccccccccccccccccccccccccccccccc",
            "ccccccccccccccccccccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "ccccccccccccccccccccrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "ccccccccccccccccccccrr                            ",
            "ccccccccccccccccccccrr                            ",
            "ccccccccccccccccccccrr                            ",
            "ccccccccccccccccccccrr                            ",
            "ccccccccccccccccccccrr                            ",
            "ccccccccccccccccccccrr                            ",
            "ccccccccccccccccccccrr      rrrrrrrrrrrrrrrrrrrrrr",
            "ccccccccccccccccccccrr      rrrrrrrrrrrrrrrrrrrrrr",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "ccccccccccccccccccccrr      rrcccccccccccccccccccc",
            "        ccccccccccccrr      rrcccccccccccccccccccc",
            "        ccccccccccccrr      rrcccccccccccccccccccc",
            "        ccccccccccccrr      rrccccccccccccccc     ",
            "        ccccccccccccrr      rrccccccccccccccc     ",
        ],
        24 | 33 => vec![
            "kkkkkkkkkkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "kkkkkkkkkkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "kkkkkkkkkkrr                          rrkkkkk     ",
            "kkkkkkkkkkrr                          rrkkkkk     ",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrr",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrr",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "        kkrr                          rrkkkkkkkkkk",
            "        kkrr                          rrkkkkkkkkkk",
            "        kkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "        kkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
        ],
        25 | 27 => vec![
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkk     ",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkrr      rrkkkkkkkkkkkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rrrrrrrrrrrrrrrrrrrrrr",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
            "        kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk     ",
        ],
        26 => vec![
            "kkkkkkkkkkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "kkkkkkkkkkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "kkkkkkkkkkrr                rr        rrkkkkk     ",
            "kkkkkkkkkkrr                rr        rrkkkkk     ",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "rrrrrrrrrrrrrrrrrrrrrr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrrrrrrrrrrrr      rr        rrrrrrrrrrrr",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "kkkkkkkkkkrr                          rrkkkkkkkkkk",
            "        kkrr                          rrkkkkkkkkkk",
            "        kkrr                          rrkkkkkkkkkk",
            "        kkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "        kkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
        ],
        28 => vec![
            "kkkkkkkkkkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "kkkkkkkkkkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "kkkkkkkkkkrr        rr      rr        rrkkkkk     ",
            "kkkkkkkkkkrr        rr      rr        rrkkkkk     ",
            "kkkkkkkkkkrr        rr      rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr        rr      rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr        rr      rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr        rr      rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr        rr      rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr        rr      rr        rrkkkkkkkkkk",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "                    rr                            ",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "kkkkkkkkkkrr                rr        rrkkkkkkkkkk",
            "        kkrr                rr        rrkkkkkkkkkk",
            "        kkrr                rr        rrkkkkkkkkkk",
            "        kkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
            "        kkrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrkkkkk     ",
        ],
        32 => vec![
            "jjjjjjjjjjrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrjjjjj     ",
            "jjjjjjjjjjrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrjjjjj     ",
            "jjjjjjjjjjrr                rr        rrjjjjj     ",
            "jjjjjjjjjjrr                rr        rrjjjjj     ",
            "jjjjjjjjjjrr                rr        rrjjjjjjjjjj",
            "jjjjjjjjjjrr                rr        rrjjjjjjjjjj",
            "jjjjjjjjjjrr                rr        rrjjjjjjjjjj",
            "jjjjjjjjjjrr                rr        rrjjjjjjjjjj",
            "jjjjjjjjjjrr                rr        rrjjjjjjjjjj",
            "jjjjjjjjjjrr                rr        rrjjjjjjjjjj",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "                    rr      rr                    ",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "rrrrrrrrrrrr        rr      rr        rrrrrrrrrrrr",
            "jjjjjjjjjjrr        rr                rrjjjjjjjjjj",
            "jjjjjjjjjjrr        rr                rrjjjjjjjjjj",
            "jjjjjjjjjjrr        rr                rrjjjjjjjjjj",
            "jjjjjjjjjjrr        rr                rrjjjjjjjjjj",
            "jjjjjjjjjjrr        rr                rrjjjjjjjjjj",
            "jjjjjjjjjjrr        rr                rrjjjjjjjjjj",
            "        jjrr        rr                rrjjjjjjjjjj",
            "        jjrr        rr                rrjjjjjjjjjj",
            "        jjrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrjjjjj     ",
            "        jjrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrjjjjj     ",
        ],
        38 | 53 => vec![
            "jjjjjjjjjjuuuuuuuuuuuuuuuuuuuuuuuuuuuuuujjjjj     ",
            "jjjjjjjjjjuuuuuuuuuuuuuuuuuuuuuuuuuuuuuujjjjj     ",
            "jjjjjjjjjjuu        uu      uu        uujjjjj     ",
            "jjjjjjjjjjuu        uu      uu        uujjjjj     ",
            "jjjjjjjjjjuu        uu      uu        uujjjjjjjjjj",
            "jjjjjjjjjjuu        uu      uu        uujjjjjjjjjj",
            "jjjjjjjjjjuu        uu      uu        uujjjjjjjjjj",
            "jjjjjjjjjjuu        uu      uu        uujjjjjjjjjj",
            "jjjjjjjjjjuu        uu      uu        uujjjjjjjjjj",
            "jjjjjjjjjjuu        uu      uu        uujjjjjjjjjj",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "uuuuuuuuuuuu        uuuuuuuuuu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uuuuuuuuuu        uuuuuuuuuuuu",
            "jjjjjjjjjjuu                          uujjjjjjjjjj",
            "jjjjjjjjjjuu                          uujjjjjjjjjj",
            "jjjjjjjjjjuu                          uujjjjjjjjjj",
            "jjjjjjjjjjuu                          uujjjjjjjjjj",
            "jjjjjjjjjjuu                          uujjjjjjjjjj",
            "jjjjjjjjjjuu                          uujjjjjjjjjj",
            "        jjuu                          uujjjjjjjjjj",
            "        jjuu                          uujjjjjjjjjj",
            "        jjuuuuuuuuuuuuuuuuuuuuuuuuuuuuuujjjjj     ",
            "        jjuuuuuuuuuuuuuuuuuuuuuuuuuuuuuujjjjj     ",
        ],
        39 | 52 | 60 => vec![
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj     ",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "uuuuuuuuuuuuuuuuuuuuuu      uuuuuuuuuuuuuuuuuuuuuu",
            "uuuuuuuuuuuuuuuuuuuuuu      uuuuuuuuuuuuuuuuuuuuuu",
            "jjjjjjjjjjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "jjjjjjjjjjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "        jjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "        jjjjjjjjjjjjuu      uujjjjjjjjjjjjjjjjjjjj",
            "        jjjjjjjjjjjjuu      uujjjjjjjjjjjjjjj     ",
            "        jjjjjjjjjjjjuu      uujjjjjjjjjjjjjjj     ",
        ],
        40 | 54 | 61 => vec![
            "iiiiiiiiiiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
            "iiiiiiiiiiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
            "iiiiiiiiiiuu        uu      uu        uuiiiii     ",
            "iiiiiiiiiiuu        uu      uu        uuiiiii     ",
            "iiiiiiiiiiuu        uu      uu        uuiiiiiiiiii",
            "iiiiiiiiiiuu        uu      uu        uuiiiiiiiiii",
            "iiiiiiiiiiuu        uu      uu        uuiiiiiiiiii",
            "iiiiiiiiiiuu        uu      uu        uuiiiiiiiiii",
            "iiiiiiiiiiuu        uu      uu        uuiiiiiiiiii",
            "iiiiiiiiiiuu        uu      uu        uuiiiiiiiiii",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "        iiuu                          uuiiiiiiiiii",
            "        iiuu                          uuiiiiiiiiii",
            "        iiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
            "        iiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
        ],
        41 | 50 | 59 => vec![
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "uuuuuuuuuuuuuuuuuuuuuu      uuuuuuuuuuuuuuuuuuuuuu",
            "uuuuuuuuuuuuuuuuuuuuuu      uuuuuuuuuuuuuuuuuuuuuu",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
            "        iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
        ],
        42 | 48 | 66 => vec![
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "uuuuuuuuuuuuuuuuuuuuuu      uudddddddddddddddddddd",
            "uuuuuuuuuuuuuuuuuuuuuu      uudddddddddddddddddddd",
            "                            uudddddddddddddddddddd",
            "                            uudddddddddddddddddddd",
            "                            uudddddddddddddddddddd",
            "                            uudddddddddddddddddddd",
            "                            uudddddddddddddddddddd",
            "                            uudddddddddddddddddddd",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuudddddddddddddddddddd",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuudddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "        dddddddddddddddddddddddddddddddddddddddddd",
            "        dddddddddddddddddddddddddddddddddddddddddd",
            "        ddddddddddddddddddddddddddddddddddddd     ",
            "        ddddddddddddddddddddddddddddddddddddd     ",
        ],
        43 | 46 | 49 => vec![
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii     ",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "iiiiiiiiiiiiiiiiiiiiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "iiiiiiiiiiiiiiiiiiiiuu                            ",
            "iiiiiiiiiiiiiiiiiiiiuu                            ",
            "iiiiiiiiiiiiiiiiiiiiuu                            ",
            "iiiiiiiiiiiiiiiiiiiiuu                            ",
            "iiiiiiiiiiiiiiiiiiiiuu                            ",
            "iiiiiiiiiiiiiiiiiiiiuu                            ",
            "iiiiiiiiiiiiiiiiiiiiuu      uuuuuuuuuuuuuuuuuuuuuu",
            "iiiiiiiiiiiiiiiiiiiiuu      uuuuuuuuuuuuuuuuuuuuuu",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "iiiiiiiiiiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiuu      uuiiiiiiiiiiiiiiiiiiii",
            "        iiiiiiiiiiiiuu      uuiiiiiiiiiiiiiii     ",
            "        iiiiiiiiiiiiuu      uuiiiiiiiiiiiiiii     ",
        ],
        44 | 58 | 55 => vec![
            "eeeeeeeeeeuuuuuuuuuuuuuuuuuuuuuuuuuuuuuueeeee     ",
            "eeeeeeeeeeuuuuuuuuuuuuuuuuuuuuuuuuuuuuuueeeee     ",
            "eeeeeeeeeeuu        uu      uu        uueeeee     ",
            "eeeeeeeeeeuu        uu      uu        uueeeee     ",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "                                                  ",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "eeeeeeeeeeuu        uu      uu        uueeeeeeeeee",
            "        eeuu        uu      uu        uueeeeeeeeee",
            "        eeuu        uu      uu        uueeeeeeeeee",
            "        eeuuuuuuuuuuuuuuuuuuuuuuuuuuuuuueeeee     ",
            "        eeuuuuuuuuuuuuuuuuuuuuuuuuuuuuuueeeee     ",
        ],
        45 | 56 | 64 => vec![
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee     ",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee     ",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee     ",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee     ",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuueeeeeeeeeeeeeeeeeeee",
            "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuueeeeeeeeeeeeeeeeeeee",
            "                            uueeeeeeeeeeeeeeeeeeee",
            "                            uueeeeeeeeeeeeeeeeeeee",
            "                            uueeeeeeeeeeeeeeeeeeee",
            "                            uueeeeeeeeeeeeeeeeeeee",
            "                            uueeeeeeeeeeeeeeeeeeee",
            "                            uueeeeeeeeeeeeeeeeeeee",
            "uuuuuuuuuuuuuuuuuuuuuu      uueeeeeeeeeeeeeeeeeeee",
            "uuuuuuuuuuuuuuuuuuuuuu      uueeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "eeeeeeeeeeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "        eeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "        eeeeeeeeeeeeuu      uueeeeeeeeeeeeeeeeeeee",
            "        eeeeeeeeeeeeuu      uueeeeeeeeeeeeeee     ",
            "        eeeeeeeeeeeeuu      uueeeeeeeeeeeeeee     ",
        ],
        57 | 65 => vec![
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uuddddddddddddddd     ",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uudddddddddddddddddddd",
            "dddddddddddddddddddduu      uuuuuuuuuuuuuuuuuuuuuu",
            "dddddddddddddddddddduu      uuuuuuuuuuuuuuuuuuuuuu",
            "dddddddddddddddddddduu                            ",
            "dddddddddddddddddddduu                            ",
            "dddddddddddddddddddduu                            ",
            "dddddddddddddddddddduu                            ",
            "dddddddddddddddddddduu                            ",
            "dddddddddddddddddddduu                            ",
            "dddddddddddddddddddduuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "dddddddddddddddddddduuuuuuuuuuuuuuuuuuuuuuuuuuuuuu",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "dddddddddddddddddddddddddddddddddddddddddddddddddd",
            "        dddddddddddddddddddddddddddddddddddddddddd",
            "        dddddddddddddddddddddddddddddddddddddddddd",
            "        ddddddddddddddddddddddddddddddddddddd     ",
            "        ddddddddddddddddddddddddddddddddddddd     ",
        ],
        47 | 63 => vec![
            "iiiiiiiiiiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
            "iiiiiiiiiiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
            "iiiiiiiiiiuu                          uuiiiii     ",
            "iiiiiiiiiiuu                          uuiiiii     ",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "uuuuuuuuuuuu        uuuuuuuuuu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uuuuuuuuuu        uuuuuuuuuuuu",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "                    uu                            ",
            "uuuuuuuuuuuu        uuuuuuuuuu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uuuuuuuuuu        uuuuuuuuuuuu",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "iiiiiiiiiiuu                          uuiiiiiiiiii",
            "        iiuu                          uuiiiiiiiiii",
            "        iiuu                          uuiiiiiiiiii",
            "        iiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
            "        iiuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuiiiii     ",
        ],
        51 | 62 => vec![
            "dddddddddduuuuuuuuuuuuuuuuuuuuuuuuuuuuuuddddd     ",
            "dddddddddduuuuuuuuuuuuuuuuuuuuuuuuuuuuuuddddd     ",
            "dddddddddduu                uu        uuddddd     ",
            "dddddddddduu                uu        uuddddd     ",
            "dddddddddduu                uu        uudddddddddd",
            "dddddddddduu                uu        uudddddddddd",
            "dddddddddduu                uu        uudddddddddd",
            "dddddddddduu                uu        uudddddddddd",
            "dddddddddduu                uu        uudddddddddd",
            "dddddddddduu                uu        uudddddddddd",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "                    uu      uu                    ",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "uuuuuuuuuuuu        uu      uu        uuuuuuuuuuuu",
            "dddddddddduu        uu                uudddddddddd",
            "dddddddddduu        uu                uudddddddddd",
            "dddddddddduu        uu                uudddddddddd",
            "dddddddddduu        uu                uudddddddddd",
            "dddddddddduu        uu                uudddddddddd",
            "dddddddddduu        uu                uudddddddddd",
            "        dduu        uu                uudddddddddd",
            "        dduu        uu                uudddddddddd",
            "        dduuuuuuuuuuuuuuuuuuuuuuuuuuuuuuddddd     ",
            "        dduuuuuuuuuuuuuuuuuuuuuuuuuuuuuuddddd     ",
        ],
        _ => panic!("no such level"),
    };

    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;
    for line in map {
        for c in line.chars() {
            match c.to_string().as_str() {
                " " => {},
                _ => {
                    points.push(
                        Point::new(mx,my,c.to_string()),
                    );
                },
            };
            mx += 24.0;
        }
        my += 24.0;
        mx = 0.0;
    }

    return points
}