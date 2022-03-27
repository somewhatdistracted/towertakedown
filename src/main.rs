use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const HEX_SIZE: f32 = 50.;
const BUTTON_WAIT: f64 = 0.01;

#[derive(Clone)]
struct HexGrid {
    size: u8,
    grid: Vec<Vec<Hex>>,
}

#[derive(Clone, Copy)]
struct Hex {
    pos: Vec2,
    rot: f32,
}

#[derive(Clone)]
struct Character {
    posx: i8,
    posy: i8,
    hex: Hex,
}

impl Character {
    fn move_on_map(&mut self, dx: i8, dy: i8, map: &HexGrid) {
        if self.posy + dy >= 0 && ((self.posy + dy) as usize) < map.grid.len() && self.posx + dx >= 0 && ((self.posx + dx) as usize) < map.grid[(self.posy + dy) as usize].len() {
            self.posy += dy;
            self.posx += dx;
            self.hex = map.grid[self.posy as usize][self.posx as usize];
        }
    }
}

trait Visual {
    fn render(&self);
}

impl Visual for Hex {
    fn render(&self) {
        draw_poly_lines(
            self.pos.x,
            self.pos.y,
            6,
            HEX_SIZE,
            self.rot,
            2.,
            BLACK,
        )
    }
}

impl Visual for HexGrid {
    fn render(&self) {
        for row in self.grid.iter() {
            for hex in row.iter() {
                hex.render();
            }
        }
    }
}

impl Visual for Character {
    fn render(&self) {
        let pos = self.hex.pos;
        let v1 = Vec2::new(
            pos.x,
            pos.y - 1. * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            pos.x - 1. * SHIP_BASE / 2.,
            pos.y + 1. * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            pos.x + 1. * SHIP_BASE / 2.,
            pos.y + 1. * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
    }
}

#[macroquad::main("TowerTakedown")]
async fn main() {
    let mut last_move = get_time();
    let mut load_screen = true;

    let mut screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);

    let mut map = HexGrid {
        size: 3,
        grid: vec![Vec::new(); 5],
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
    
    let mut char = Character {
        posx: 2,
        posy: 3,
        hex: map.grid[3][2],
    };

    loop {
        if load_screen {
            clear_background(LIGHTGRAY);
            let mut text = "Hi! Press [enter] to play.";
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
            next_frame().await;
            continue;
        }
        let frame_t = get_time();

        // my additions
        if is_key_pressed(KeyCode::W) && frame_t - last_move > BUTTON_WAIT {
            char.move_on_map(-1,-1,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::A) && frame_t - last_move > BUTTON_WAIT {
            char.move_on_map(-1,0,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::E) && frame_t - last_move > BUTTON_WAIT {
            char.move_on_map(0,-1,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::Z) && frame_t - last_move > BUTTON_WAIT {
            char.move_on_map(0,1,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::D) && frame_t - last_move > BUTTON_WAIT {
            char.move_on_map(1,0,&map);
            last_move = frame_t;
        }
        if is_key_pressed(KeyCode::X) && frame_t - last_move > BUTTON_WAIT {
            char.move_on_map(1,1,&map);
            last_move = frame_t;
        }

        if load_screen {
            continue;
        }

        clear_background(LIGHTGRAY);

        map.render();
        char.render();

        next_frame().await
    }
}
