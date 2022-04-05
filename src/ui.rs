use macroquad::prelude::*;
use crate::entities::*;

pub struct UIMenu {
    pub rect: Rect,
    pub buttons: Vec<UIButton>,
}

pub struct UIButton {
    pub rect: Rect,
    pub text: String,
    pub font_size: f32,
    pub toggle: bool,
}

pub struct HUD {
    pub rect: Rect,
}

impl UIButton {
    pub fn press(&mut self) {
        self.toggle = !self.toggle;
    }
}

impl Visual for UIMenu {
    fn render(&self) {
        draw_rectangle(
            self.rect.point().x,
            self.rect.point().y,
            self.rect.size().x,
            self.rect.size().y,
            GRAY,
        );
        draw_rectangle_lines(
            self.rect.point().x,
            self.rect.point().y,
            self.rect.size().x,
            self.rect.size().y,
            2.,
            DARKGRAY,
        );
        for button in self.buttons.iter() {
            button.render();
        }
    }
}

impl Visual for UIButton {
    fn render(&self) {
        let text_size = measure_text(&self.text, None, self.font_size as _, 1.0);
        draw_rectangle(
            self.rect.point().x,
            self.rect.point().y,
            self.rect.size().x,
            self.rect.size().y,
            GRAY,
        );
        draw_rectangle_lines(
            self.rect.point().x,
            self.rect.point().y,
            self.rect.size().x,
            self.rect.size().y,
            4.,
            DARKGRAY,
        );
        let (mx,my) = mouse_position();
        if self.rect.contains(Vec2::new(mx,my)) {
            draw_rectangle(
                self.rect.point().x + self.rect.size().x * 0.1,
                self.rect.point().y,
                self.rect.size().x * 0.8,
                self.rect.size().y,
                LIGHTGRAY,
            );
        }
        draw_text(
            &self.text,
            self.rect.point().x + (self.rect.size().x - text_size.width) / 2.,
            self.rect.point().y + (self.rect.size().y + text_size.height) / 2.,
            self.font_size,
            DARKGRAY,
        );  
    }
}

impl Clickable for UIButton {
    fn process_click(&mut self, mpos: Vec2) {
        if self.rect.contains(mpos) {
            self.press();
        }
    }
}

pub fn draw_menu() {
    let text = format!("Menu");
    let font_size = 48.;
    let text_size = measure_text(&text, None, font_size as _, 1.0);
    draw_rectangle(
        text_size.width * 1. - text_size.width * 0.25,
        text_size.height * 2. - text_size.height * 1.25,
        text_size.width * 1.5,
        text_size.height * 1.5,
        GRAY,
    );
    draw_rectangle_lines(
        text_size.width * 1. - text_size.width * 0.25,
        text_size.height * 2. - text_size.height * 1.25,
        text_size.width * 1.5,
        text_size.height * 1.5,
        2.,
        DARKGRAY,
    );
    draw_text(
        &text,
        text_size.width * 1.,
        text_size.height * 2.,
        font_size,
        DARKGRAY,
    );  
}
