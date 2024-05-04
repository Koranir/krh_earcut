use krh_earcut::*;
use macroquad::color;

#[macroquad::main("Outline Example")]
async fn main() {
    let polygon = [
        0, 80, 100, 0, 190, 85, 270, 35, 345, 140, 255, 130, 215, 210, 140, 70, 45, 95, 50, 185,
    ]
    .chunks(2)
    .map(|f| Point::new(f[0] as f32, f[1] as f32))
    .collect::<Vec<_>>();

    let earcut = Earcut::new(&polygon);
    let tris = earcut.earcut();

    dbg!(&tris);

    macroquad::window::request_new_screen_size(800.0, 600.0);

    loop {
        macroquad::window::clear_background(color::WHITE);

        for tri in &tris {
            macroquad::shapes::draw_triangle_lines(
                tri.0.to_array().into(),
                tri.1.to_array().into(),
                tri.2.to_array().into(),
                5.0,
                color::BLACK,
            );
        }

        macroquad::camera::set_camera(&macroquad::camera::Camera2D::from_display_rect(
            macroquad::math::Rect {
                x: macroquad::window::screen_width() / -4.0,
                y: macroquad::window::screen_height() / 1.5,
                w: macroquad::window::screen_width(),
                h: -macroquad::window::screen_height(),
            },
        ));

        macroquad::window::next_frame().await;
    }
}
