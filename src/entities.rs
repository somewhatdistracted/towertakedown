use macroquad::prelude::*;

//const SHIP_HEIGHT: f32 = 25.;
//const SHIP_BASE: f32 = 22.;
const HEX_SIZE: f32 = 30.;

#[derive(Clone)]
pub struct HexGrid {
    pub size: u8,
    pub grid: Vec<Vec<Hex>>,
}

#[derive(Clone, Copy)]
pub struct Hex {
    pub pos: Vec2,
    pub rot: f32,
}

#[derive(Clone)]
pub struct Character {
    pub posx: i8,
    pub posy: i8,
    pub hex: Hex,
    pub sprite: Texture2D,
}

impl Character {
    pub fn move_on_map(&mut self, dx: i8, dy: i8, map: &HexGrid) {
        if self.posy + dy >= 0 && ((self.posy + dy) as usize) < map.grid.len() && self.posx + dx >= 0 && ((self.posx + dx) as usize) < map.grid[(self.posy + dy) as usize].len() {
            self.posy += dy;
            self.posx += dx;
            self.hex = map.grid[self.posy as usize][self.posx as usize];
        }
    }
}

pub trait Visual {
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
        );
        let (mx, my) = mouse_position();
        if (Vec2::new(mx,my) - self.pos).length() < HEX_SIZE {
            draw_poly_lines(
                self.pos.x,
                self.pos.y,
                6,
                HEX_SIZE + 2.,
                self.rot,
                2.,
                GREEN,
            )
        }
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
        let draw_rect: Rect = Rect::new(32. * 0., 32. * ((2. * get_time()) as i64 % 2) as f32, 32., 32.);

        draw_texture_ex(
            self.sprite,
            pos.x - draw_rect.size().x / 2.,
            pos.y - draw_rect.size().y / 2.,
            WHITE,
            DrawTextureParams {
                source: Some(draw_rect),
                ..Default::default()
            },
        );  
        /*
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
        */
    }
}
