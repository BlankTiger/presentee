use presenteelib::{
    begin_drawing, clear_background, close_window, draw_text_ex, end_drawing, init_window,
    load_font, set_target_fps, unload_font, window_should_close,
};
use raylib_ffi::{
    colors::{BLACK, LIME},
    Vector2,
};

fn main() {
    let width = 800;
    let height = 600;
    init_window(width, height, "Presentee");
    set_target_fps(120);
    // let mut codepoints = 0;
    // let iosevka = load_font_ex("resources/iosevka.ttf", 50, &mut codepoints, 0);
    let iosevka = load_font("resources/iosevka.ttf");
    let default_height = height as f32 / 10.0;
    let mut text_pos = Vector2 {
        x: width as f32 / 5.0,
        y: default_height,
    };
    let msg = r#"while !window_should_close() {
    begin_drawing();

    clear_background(BLACK);
    draw_text_ex(
        iosevka,
        \"hello!\",
        text_pos,
        iosevka.baseSize as f32,
        2.0,
        LIME,
    );

    end_drawing();
}
"#;
    while !window_should_close() {
        begin_drawing();

        clear_background(BLACK);
        for line in msg.lines() {
            text_pos.y += 30.0;
            draw_text_ex(iosevka, line, text_pos, iosevka.baseSize as f32, 2.0, LIME);
        }
        text_pos.y = default_height;

        end_drawing();
    }

    unload_font(iosevka);
    close_window();
}
