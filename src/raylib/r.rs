use std::ffi::CString;

use raylib_ffi::{
    BeginDrawing, ClearBackground, CloseWindow, Color, DrawTextEx, EndDrawing, Font, InitWindow,
    LoadFont, LoadFontEx, SetTargetFPS, UnloadFont, Vector2, WindowShouldClose,
};

pub fn init_window(width: i32, height: i32, title: &str) {
    let title = CString::new(title).unwrap();
    unsafe { InitWindow(width, height, title.as_ptr()) };
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

pub fn close_window() {
    unsafe { CloseWindow() };
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) };
}

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) };
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() };
}

pub fn end_drawing() {
    unsafe { EndDrawing() };
}

pub fn unload_font(font: Font) {
    unsafe { UnloadFont(font) };
}

pub fn load_font(file_name: &str) -> Font {
    let file_name = CString::new(file_name).unwrap();
    unsafe { LoadFont(file_name.as_ptr()) }
}
pub fn load_font_ex(
    file_name: &str,
    font_size: i32,
    codepoints: &mut i32,
    codepoint_count: i32,
) -> Font {
    let file_name = CString::new(file_name).unwrap();
    unsafe { LoadFontEx(file_name.as_ptr(), font_size, codepoints, codepoint_count) }
}

pub fn draw_text_ex(
    font: Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    let text = CString::new(text.as_bytes()).unwrap();
    unsafe { DrawTextEx(font, text.as_ptr(), position, font_size, spacing, tint) };
}
