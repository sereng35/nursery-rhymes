/*
By: <Seren Greige>
Date: 2026-02-27
Program Details: <A nursery rhyme program to display three different poems when the corresponding button is clicked.>
*/

mod modules;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "nursery-rhymes".to_string(),
        window_width: 1400,
        window_height: 1100,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
     clear_background(WHITE);
    draw_grid(50.0, BROWN);
   
    let font = load_ttf_font("assets/sparky.ttf").await.unwrap();
    let mut lbl_msg = Label::new("", 100.0, 650.0, 50);
    lbl_msg.with_fixed_size(1200.0, 300.0).with_border(BLACK, 5.0);
    lbl_msg.with_colors(BLACK, Some(WHITE));
    lbl_msg.with_font(font);
    lbl_msg.with_alignment(modules::label::TextAlign::Center);

    let font = load_ttf_font("assets/sparky.ttf").await.unwrap();
    let btn_exit = TextButton::new(1300.0, 50.0, 50.0, 50.0, "x", RED, GRAY, 30);

    let mut btn_star = TextButton::new(100.0, 950.0, 400.0, 80.0, "Twinkle, Twinkle Little Star", GRAY, DARKGRAY, 30);
    btn_star.with_font(font.clone());

    let mut btn_tea = TextButton::new(500.0, 950.0, 400.0, 80.0, "I'm a Little Teapot", GRAY, DARKGRAY, 30);
    btn_tea.with_font(font.clone());

    let mut btn_lamb = TextButton::new(900.0, 950.0, 400.0, 80.0, "Mary Had a Little Lamb", GRAY, DARKGRAY, 30);
    btn_lamb.with_font(font.clone());

    let mut img_out = StillImage::new("", 650.0, 500.0, 400.0, 90.0, true, 1.0).await;

    let texture_manager = TextureManager::new();
    texture_manager
        .preload_with_loading_screen(["assets/star.png", "assets/tea.png", "assets/lamb.png"], None)
        .await;

    loop {
        if btn_star.click() {
            lbl_msg.set_text("Twinkle, twinkle, little star\nHow I wonder what you are\nUp above the world so high, like a diamond in the sky\nTwinkle, twinkle little star, how I wonder what you are");
            img_out.set_preload(texture_manager.get_preload("assets/star.png").unwrap());
        }
        if btn_tea.click() {
            lbl_msg.set_text(
                "I'm a little teapot, short and stout\n Here is my handle and here is my spout\n when you tip me over, people shout\n tip me over and pour me out!",
            );
            img_out.set_preload(texture_manager.get_preload("assets/tea.png").unwrap());
        }
        if btn_lamb.click() {
            lbl_msg.set_text(
                "Mary had a little lamb, little lamb, little lamb\n Mary had a little lamb who's fleece was white as snow\nEverywhere that Mary went, Mary went, Mary went\n Everywhere that Mary went, the lamb was sure to go",
            );
            img_out.set_preload(texture_manager.get_preload("assets/lamb.png").unwrap());
        }
        if btn_exit.click() {
            break;
        }


        img_out.draw();
        lbl_msg.draw();
        next_frame().await;
    }
}
