use std::ops::Range;

use raylib::{ffi::GetRandomValue, prelude::*};

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;
const TARGET_FPS: u32 = 60;

struct Ball {
    position: Vector2,
    speed: Vector2,
    radius: f32,
    color: Color,
}

fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(
        text,
        (SCREEN_WIDTH as i32) / 2 - (text_length / 2),
        y,
        font_size,
        color,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello World!")
        .build();

    let pizza_texture = rl.load_texture(&thread, "src/pizza_better.png").unwrap();

    rl.set_target_fps(TARGET_FPS);
    let mut value: i32 = rl.get_random_value(Range {
        start: -100,
        end: 100,
    });
    let mut frame_count = 0;
    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: Vector2::new(10.0, 10.0),
        radius: 40.0,
        color: Color::GREEN,
    };
    let mut pause = false;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        // if rl.is_key_down(KeyboardKey::KEY_RIGHT){
        //     ball.position.x += ball.speed * dt;
        // }
        // if rl.is_key_down(KeyboardKey::KEY_DOWN){
        //     ball.position.y += ball.speed * dt;
        // }
        // if rl.is_key_down(KeyboardKey::KEY_LEFT){
        //     ball.position.x -= ball.speed * dt;
        // }
        // if rl.is_key_down(KeyboardKey::KEY_UP){
        //     ball.position.y -= ball.speed * dt;
        // }

        // if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        //     if ball.color == Color::GREEN {
        //         ball.color = Color::YELLOW;
        //     }
        //     else {
        //         ball.color = Color::YELLOW;
        //     }
        // }

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            pause = !pause;
        }
        if !pause {
            ball.position += ball.speed;

            if ball.position.x >= SCREEN_WIDTH - ball.radius || ball.position.x <= ball.radius {
                ball.speed.x *= -1.0;
            }
            if ball.position.y >= SCREEN_HEIGHT - ball.radius || ball.position.y <= ball.radius {
                ball.speed.y *= -1.0;
            }
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            ball.position = ball.position.lerp(rl.get_mouse_position(), dt);
        }

        if frame_count % TARGET_FPS == 0 {
            value = rl.get_random_value(Range {
                start: -100,
                end: 100,
            });
        }

        let mut d = rl.begin_drawing(&thread);

        draw_text_center(
            &mut d,
            "Every second a new number is generated:",
            (SCREEN_HEIGHT as i32) / 4,
            20,
            Color::WHITE,
        );
        draw_text_center(
            &mut d,
            &format!("{}", value),
            (SCREEN_HEIGHT as i32) / 4 + 25,
            32,
            Color::BLUE,
        );

        d.clear_background(Color::BLACK);
        d.draw_text("Hello world", 12, 12, 24, Color::WHITE);

        d.draw_circle_v(ball.position, ball.radius, ball.color);
        d.draw_fps(10, 10);

        // d.draw_texture_v(&pizza_texture, Vector2::new(0.0, 0.0), Color::WHITE);
        // d.draw_texture_rec(
        //     &pizza_texture,
        //     Rectangle::new(
        //         0.0,
        //         0.0,
        //         92.0,
        //         92.0,
        //     ),
        //     Vector2::new(200.0, 70.0),
        //     Color::WHITE,
        // );
        d.draw_texture_pro(&pizza_texture, Rectangle::new(0.0, 0.0, 100.0, 100.0), Rectangle::new(10.0, 10.0, 110.0, 110.0), Vector2::new(10.0, 10.0), 0.0, Color::WHITE);

        frame_count += 1;
    }
}
