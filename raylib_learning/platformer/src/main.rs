use std::ops::Range;

use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 450.0;
const GRAVITY: f32 = 1.0;

struct Player {
    position: Vector3,

}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("raylib [core] example - 3d camera first person")
        .build();

    rl.disable_cursor();

    rl.set_target_fps(60);

    let mut camera: Camera3D = Camera3D::perspective(
        Vector3::new(0.0, 2.0, 0.0),
        Vector3::new(1.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    let mut camera_mode: CameraMode = CameraMode::CAMERA_THIRD_PERSON;

    while !rl.window_should_close() {

        // rl.update_camera(&mut camera, camera_mode);
        // let movement_vec = {
        //     let mut v = Vector3::new(0.0, 0.0, 0.0);
        //     if rl.is_key_down(KeyboardKey::KEY_W)
        //     {
        //         v.x += 1.0;
        //     }
        //     if rl.is_key_down(KeyboardKey::KEY_S)
        //     {
        //         v.x -= 1.0;
        //     }
        //     if rl.is_key_down(KeyboardKey::KEY_D)
        //     {
        //         v.y += 1.0;
        //     }
        //     if rl.is_key_down(KeyboardKey::KEY_A)
        //     {
        //         v.y -= 1.0;
        //     }
        //     //todo: implement falling based on collision to the ground
        //     v
        // };
        //
        // rl.update_camera_pro(
        //     &mut camera,
        //     movement_vec,
        //     Vector3::new(
        //         rl.get_mouse_delta().x * 0.05,
        //         rl.get_mouse_delta().y * 0.05,
        //         0.0,
        //     ),
        //     rl.get_mouse_wheel_move() * -2.0,
        // );

        let mut draw_handler: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

        let mut draw_3d: RaylibMode3D<'_, RaylibDrawHandle<'_>> = draw_handler.begin_mode3D(camera);
        // draw_3d.update_camera(&mut camera, camera_mode);

        draw_3d.clear_background(Color::RAYWHITE);
        draw_3d.draw_plane(
            Vector3::new(0.0, 0.0, 0.0),
            Vector2::new(32.0, 32.0),
            Color::LIGHTGRAY,
        );


        //this is the only way to "escape" 3D mode
        // std::mem::drop(draw_3d);
    }
}
