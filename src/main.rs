use macroquad::prelude::*;
use crate::entities::*;
use crate::ui::*;

pub mod entities;
pub mod ui;

//const BUTTON_WAIT: f64 = 0.01;

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

    let mut turn_end = get_time();
    let mut load_screen = true;

    let mut screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);

    let map = HexGrid::new(screen_center, 5, vec![9; 9]);
    
    let character = Character {
        posx: 2,
        posy: 3,
        hex: map.grid[3][2],
        sprite: tile_set,
    };

    let mut menu_button = UIButton {
        rect: Rect::new(50., 50., 200., 50.),
        text: format!("Menu"),
        font_size: 48.,
        toggle: false,
    };

    let reset_button = UIButton {
        rect: Rect::new(50., 125., 200., 50.),
        text: format!("Reset"),
        font_size: 48.,
        toggle: false,
    };

    let mut game = Game {
        map: map,
        entities: vec![Box::new(character)],
        ui: vec![Box::new(reset_button)],
        select: 0,
        turn: Turn::Player,
    };

    loop {
        if load_screen || menu_button.toggle {
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
                menu_button.press();
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

        if load_screen {
            continue;
        }

        clear_background(LIGHTGRAY);

        game.render();
        menu_button.render();
        draw_fps();

        if get_time() - turn_end > 1.5 {
            println!("trig frame");
            if game.turn == Turn::Player {
                game.process_input();
                turn_end = get_time();
            } else {
                game.process_ai();
                turn_end = get_time();
            }
        }

        next_frame().await
    }
}
