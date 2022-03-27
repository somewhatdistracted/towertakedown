use macroquad::prelude::*;

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;
const HEX_SIZE: f32 = 50.;

struct Ship {
    pos: Vec2,
    rot: f32,
    vel: Vec2,
}

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
    posx: isize,
    posy: isize,
    hex: Hex,
}

impl Character {
    fn move_on_map(&mut self, dx: isize, dy: isize, map: &HexGrid) {
        if self.posy + dy >= 0 && ((self.posy + dy) as usize) < map.grid.len() && self.posx + dx >= 0 && ((self.posx + dx) as usize) < map.grid[(self.posy + dy) as usize].len() {
            self.posy += dy;
            self.posx += dx;
            self.hex = map.grid[self.posy as usize][self.posx as usize];
        }
    }
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    shot_at: f64,
    collided: bool,
}

struct Asteroid {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    size: f32,
    sides: u8,
    collided: bool,
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

fn wrap_around(v: &Vec2) -> Vec2 {
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > screen_width() {
        vr.x = 0.;
    }
    if vr.x < 0. {
        vr.x = screen_width()
    }
    if vr.y > screen_height() {
        vr.y = 0.;
    }
    if vr.y < 0. {
        vr.y = screen_height()
    }
    vr
}

#[macroquad::main("TowerTakedown")]
async fn main() {
    let mut ship = Ship {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        rot: 0.,
        vel: Vec2::new(0., 0.),
    };

    let mut bullets = Vec::new();
    let mut last_shot = get_time();
    let mut last_move = get_time();
    let mut asteroids = Vec::new();
    let mut gameover = false;

    let mut screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
    for _ in 0..1 {
        asteroids.push(Asteroid {
            pos: screen_center
                + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)).normalize()
                    * screen_width().min(screen_height())
                    / 2.,
            vel: Vec2::new(rand::gen_range(-1., 1.), 0.),
            rot: 0.,
            rot_speed: rand::gen_range(-2., 2.),
            size: screen_width().min(screen_height()) / 10.,
            sides: 6,
            collided: false,
        })
    }

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
        if gameover {
            clear_background(LIGHTGRAY);
            let mut text = "You Win!. Press [enter] to play again.";
            let font_size = 30.;

            if asteroids.len() > 0 {
                text = "Game Over. Press [enter] to play again.";
            }
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. - text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
            if is_key_down(KeyCode::Enter) {
                ship = Ship {
                    pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
                    rot: 0.,
                    vel: Vec2::new(0., 0.),
                };
                bullets = Vec::new();
                asteroids = Vec::new();
                gameover = false;
                screen_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
                for _ in 0..10 {
                    asteroids.push(Asteroid {
                        pos: screen_center
                            + Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.))
                                .normalize()
                                * screen_width().min(screen_height())
                                / 2.,
                        vel: Vec2::new(rand::gen_range(-1., 1.), rand::gen_range(-1., 1.)),
                        rot: 0.,
                        rot_speed: rand::gen_range(-2., 2.),
                        size: screen_width().min(screen_height()) / 10.,
                        sides: 6,
                        collided: false,
                    })
                }
            }
            next_frame().await;
            continue;
        }
        let frame_t = get_time();
        let rotation = ship.rot.to_radians();

        let mut acc = -ship.vel / 10.;
        if is_key_down(KeyCode::Up) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }

        if is_key_down(KeyCode::Space) && frame_t - last_shot > 0.1 {
            let rot_vec = Vec2::new(rotation.sin(), -rotation.cos());
            bullets.push(Bullet {
                pos: ship.pos + rot_vec * SHIP_HEIGHT / 2.,
                vel: rot_vec * 7.,
                shot_at: frame_t,
                collided: false,
            });
            last_shot = frame_t;
        }
        if is_key_down(KeyCode::Right) {
            ship.rot += 5.;
        } else if is_key_down(KeyCode::Left) {
            ship.rot -= 5.;
        }

        // my additions
        if is_key_down(KeyCode::W) && frame_t - last_move > 0.5 {
            char.move_on_map(-1,-1,&map);
            last_move = frame_t;
        }
        if is_key_down(KeyCode::A) && frame_t - last_move > 0.5 {
            char.move_on_map(-1,0,&map);
            last_move = frame_t;
        }
        if is_key_down(KeyCode::E) && frame_t - last_move > 0.5 {
            char.move_on_map(0,-1,&map);
            last_move = frame_t;
        }
        if is_key_down(KeyCode::Z) && frame_t - last_move > 0.5 {
            char.move_on_map(0,1,&map);
            last_move = frame_t;
        }
        if is_key_down(KeyCode::D) && frame_t - last_move > 0.5 {
            char.move_on_map(1,0,&map);
            last_move = frame_t;
        }
        if is_key_down(KeyCode::X) && frame_t - last_move > 0.5 {
            char.move_on_map(1,1,&map);
            last_move = frame_t;
        }
        // end my additions

        ship.vel += acc;
        if ship.vel.length() > 5. {
            ship.vel = ship.vel.normalize() * 5.;
        }
        ship.pos += ship.vel;
        ship.pos = wrap_around(&ship.pos);
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel;
        }
        for asteroid in asteroids.iter_mut() {
            asteroid.pos += asteroid.vel;
            asteroid.pos = wrap_around(&asteroid.pos);
            asteroid.rot += asteroid.rot_speed;
        }

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);

        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            /*
            if (asteroid.pos - ship.pos).length() < asteroid.size + SHIP_HEIGHT / 3. {
                gameover = true;
                break;
            }
            */
            for bullet in bullets.iter_mut() {
                if (asteroid.pos - bullet.pos).length() < asteroid.size {
                    asteroid.collided = true;
                    bullet.collided = true;
                    if asteroid.sides > 4 {
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(bullet.vel.y, -bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        });
                        new_asteroids.push(Asteroid {
                            pos: asteroid.pos,
                            vel: Vec2::new(-bullet.vel.y, bullet.vel.x).normalize()
                                * rand::gen_range(1., 3.),
                            rot: rand::gen_range(0., 360.),
                            rot_speed: rand::gen_range(-2., 2.),
                            size: asteroid.size * 0.8,
                            sides: asteroid.sides - 1,
                            collided: false,
                        })
                    }
                    break;
                }
            }
        }

        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t && !bullet.collided);
        asteroids.retain(|asteroid| !asteroid.collided);
        asteroids.append(&mut new_asteroids);

        if asteroids.len() == 0 {
            gameover = true;
        }

        if gameover {
            continue;
        }

        clear_background(LIGHTGRAY);
        for bullet in bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, 2., BLACK);
        }

        map.render();
        char.render();

        let v1 = Vec2::new(
            ship.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            ship.pos.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            ship.pos.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            ship.pos.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
        next_frame().await
    }
}
