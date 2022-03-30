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
}

pub struct HUD {
    pub rect: Rect,
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
            &self.text,
            text_size.width * 1.,
            text_size.height * 2.,
            self.font_size,
            DARKGRAY,
        );  
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
