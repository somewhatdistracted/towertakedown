use macroquad::prelude::*;
use crate::entities::*;
use crate::ui::*;

pub mod entities;
pub mod ui;

const HEX_SIZE: f32 = 30.;
const BUTTON_WAIT: f64 = 0.01;

fn draw_fps() {
    let t_fps = format!("FPS: {}", get_fps());
    let font_size = 30.;
    let ts_fps = measure_text(&t_fps, None, font_size as _, 1.0);
    draw_text(
        &t_fps,
        screen_width() - ts_fps.width * 1.2,
        ts_fps.height * 1.2,
        font_size,
        DARKGRAY,
    );
}

#[macroquad::main("TowerTakedown")]
async fn main() {
    let tile_set = load_texture("resources/piskelite.png").await.unwrap();

    let mut last_move = get_time();
    let mut load_screen = true;

    let mut screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);

    let mut map = HexGrid {
        size: 5,
        grid: vec![Vec::new(); 9],
    };
    let map_top = screen_center + Vec2::new(- 2. * HEX_SIZE * (map.size - 2) as f32, - 2. * HEX_SIZE * (map.size - 1) as f32);
    
    for row in 0..(2*map.size-1) {
        for col in 0..(2*map.size-1) {
            map.grid[row as usize].push(Hex {
                pos: map_top + Vec2::new(2. * HEX_SIZE * col as f32 - 1. * HEX_SIZE * row as f32, 2. * HEX_SIZE * row as f32),
                rot: 90.,
            })
        }
    }
    
    let mut character = Character {
        posx: 2,
        posy: 3,
        hex: map.grid[3][2],
        sprite: tile_set,
    };

    let menu = UIButton {
        rect: Rect::new(50., 50., 200., 50.),
        text: format!("Menu"),
        font_size: 48.,
    };

    loop {
        if load_screen {
            clear_background(LIGHTGRAY);
            let text = "Hi! Press [enter] to play.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
            if is_key_pressed(KeyCode::Enter) {
                load_screen = false;
                screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
            }

            draw_texture_ex(
                tile_set,
                screen_center.x,
                screen_center.y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
            draw_fps();

            next_frame().await;
            continue;
        }
        let frame_t = get_time();

        // my additions
        if is_key_pressed(KeyCode::W) && frame_t - last_move > BUTTON_WAIT {
            character.move_on_map(-1,-1,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::A) && frame_t - last_move > BUTTON_WAIT {
            character.move_on_map(-1,0,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::E) && frame_t - last_move > BUTTON_WAIT {
            character.move_on_map(0,-1,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::Z) && frame_t - last_move > BUTTON_WAIT {
            character.move_on_map(0,1,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::D) && frame_t - last_move > BUTTON_WAIT {
            character.move_on_map(1,0,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::X) && frame_t - last_move > BUTTON_WAIT {
            character.move_on_map(1,1,&map);
            last_move = frame_t;
        }

        if load_screen {
            continue;
        }

        clear_background(LIGHTGRAY);

        map.render();
        character.render();
        menu.render();
        draw_fps();

        next_frame().await
    }
}
