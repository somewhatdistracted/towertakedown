use macroquad::prelude::*;

//const SHIP_HEIGHT: f32 = 25.;
//const SHIP_BASE: f32 = 22.;
const HEX_SIZE: f32 = 36.;

pub enum Direction {
    UpLeft,
    UpRight,
    Left,
    Right,
    DownLeft,
    DownRight,
}

pub struct Game {
    pub map: HexGrid,
    pub entities: Vec<Box<dyn Entity>>,
    pub ui: Vec<Box<dyn Clickable>>,
    pub select: usize,
}

#[derive(Clone)]
pub struct HexGrid {
    pub center: Vec2,
    pub size: usize,
    pub grid: Vec<Vec<Hex>>,
}

#[derive(Clone, Copy)]
pub struct Hex {
    pub idx: usize,
    pub idy: usize,
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

#[derive(Clone)]
pub struct Enemy {
    pub posx: i8,
    pub posy: i8,
    pub hex: Hex,
    pub sprite: Texture2D,
}

impl Game {
    pub fn process_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx,my) = mouse_position();
            let mpos = Vec2::new(mx,my);
            for ui in self.ui.iter_mut() {
                ui.process_click(mpos);
            }
            self.map.process_click(mpos, &mut *self.entities[self.select]);
        }
    }

    pub fn process_ai(&mut self) {
        for entity in self.entities.iter_mut() {
            if entity.needs_updating() {
                entity.take_turn(&mut self.map);
            }
        }
    }

    pub fn render(&self) {
        self.map.render();
        for entity in self.entities.iter() {
            entity.render();
        }
        for ui in self.ui.iter() {
            ui.render();
        }
    }

    pub fn spawn_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }
}

impl HexGrid {
    pub fn new(center: Vec2, size: usize, gen: Vec<u8>) -> HexGrid {
        let map_top = center + Vec2::new(- 2. * HEX_SIZE * (size - 2) as f32, - 2. * HEX_SIZE * (size - 1) as f32);
        let mut hexes = vec![Vec::new(); gen.len()];
        for row in 0..(2*size-1) {
            for col in 0..(2*size-1) {
                hexes[row as usize].push(Hex {
                    idx: col,
                    idy: row,
                    pos: map_top + Vec2::new(2. * HEX_SIZE * col as f32 - 1. * HEX_SIZE * row as f32, 2. * HEX_SIZE * row as f32),
                    rot: 90.,
                })  
            }   
        }
        return HexGrid {
            center: center,
            size: size,
            grid: hexes,
        };
    }

    pub fn process_click(&self, mpos: Vec2, c: &mut dyn Entity) {
        for row in self.grid.iter() {
            for hex in row.iter() {
                hex.process_click(mpos, c);
            }
        }
    }

    pub fn get_path_between_hexes(&self, src_hex: &Hex, dest_hex: &Hex) -> Vec<Direction> {
        let mut dx = dest_hex.idx as i8 - src_hex.idx as i8;
        let mut dy = dest_hex.idy as i8 - src_hex.idy as i8;
        let mut path: Vec<Direction> = Vec::new();
        while dx != 0 && dy != 0 {
            if dx > 0 && dy > 0 {
                path.push(Direction::DownRight);
                dx -= 1;
                dy -= 1;
            } else if dx < 0 && dy < 0 {
                path.push(Direction::UpLeft);
                dx += 1;
                dy += 1;
            } else if dy > 0 {
                path.push(Direction::DownLeft);
                dy -= 1;
            } else if dy < 0 {
                path.push(Direction::UpRight);
                dy += 1;
            } else if dx > 0 {
                path.push(Direction::Right);
                dx -= 1;
            } else if dx < 0 {
                path.push(Direction::Left);
                dx += 1;
            }
        }
        return path;
    }
}

impl Hex {
    pub fn process_click(&self, mpos: Vec2, c: &mut dyn Entity) {
        if (mpos - self.pos).length() < HEX_SIZE {
            c.move_to_hex(self);
        }
    }
}

pub trait Visual {
    fn render(&self);
}

pub trait Clickable: Visual {
    fn process_click(&mut self, mpos: Vec2);
}

pub trait Entity: Visual {
    fn move_on_map(&mut self, dx: i8, dy: i8, map: &HexGrid);
    fn move_to_hex(&mut self, hex: &Hex);
    fn move_direction(&mut self, dir: Direction, dist: i8, map: &HexGrid);
    fn needs_updating(&self) -> bool {
        return false;
    }
    fn take_turn(&mut self, map: &mut HexGrid) {
        return;
    }
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
    }
}

impl Entity for Character {
    fn move_on_map(&mut self, dx: i8, dy: i8, map: &HexGrid) {
        if self.posy + dy >= 0 && ((self.posy + dy) as usize) < map.grid.len() && self.posx + dx >= 0 && ((self.posx + dx) as usize) < map.grid[(self.posy + dy) as usize].len() {
            self.posy += dy;
            self.posx += dx;
            self.hex = map.grid[self.posy as usize][self.posx as usize];
        }
    }

    fn move_to_hex(&mut self, hex: &Hex) {
        self.posy = hex.idy as i8;
        self.posx = hex.idx as i8;
        self.hex = *hex;
    }

    fn move_direction(&mut self, dir: Direction, dist: i8, map: &HexGrid) {
        let (mut dx,mut dy): (i8, i8);
        match dir {
            Direction::UpLeft => (dx, dy) = (-1,-1),
            Direction::UpRight => (dx, dy) = (0,-1),
            Direction::Left => (dx, dy) = (-1,0),
            Direction::Right => (dx, dy) = (1,0),
            Direction::DownLeft => (dx, dy) = (0,1),
            Direction::DownRight => (dx, dy) = (1,1),
        }
        dx *= dist;
        dy *= dist;
        self.move_on_map(dx, dy, map);
    }
}

impl Visual for Enemy {
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
    }
}

impl Entity for Enemy {
    fn move_on_map(&mut self, dx: i8, dy: i8, map: &HexGrid) {
        if self.posy + dy >= 0 && ((self.posy + dy) as usize) < map.grid.len() && self.posx + dx >= 0 && ((self.posx + dx) as usize) < map.grid[(self.posy + dy) as usize].len() {
            self.posy += dy;
            self.posx += dx;
            self.hex = map.grid[self.posy as usize][self.posx as usize];
        }
    }

    fn move_to_hex(&mut self, hex: &Hex) {
        self.posy = hex.idy as i8;
        self.posx = hex.idx as i8;
        self.hex = *hex;
    }

    fn move_direction(&mut self, dir: Direction, dist: i8, map: &HexGrid) {
        let (mut dx,mut dy): (i8, i8);
        match dir {
            Direction::UpLeft => (dx, dy) = (-1,-1),
            Direction::UpRight => (dx, dy) = (0,-1),
            Direction::Left => (dx, dy) = (-1,0),
            Direction::Right => (dx, dy) = (1,0),
            Direction::DownLeft => (dx, dy) = (0,1),
            Direction::DownRight => (dx, dy) = (1,1),
        }
        dx *= dist;
        dy *= dist;
        self.move_on_map(dx, dy, map);
    }
}
