use std::ops::Range;

use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 450.0;

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
        60.0
    );
    let mut camera_mode: CameraMode = CameraMode::CAMERA_FIRST_PERSON;
    
    let infobox_color: Color = rl.gui_fade(Color::SKYBLUE, 0.5);

    
    while !rl.window_should_close(){
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            camera_mode = CameraMode::CAMERA_FREE;
            camera.up = Vector3::new(0.0, 1.0, 0.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            camera_mode = CameraMode::CAMERA_FIRST_PERSON;
            camera.up = Vector3::new(0.0, 1.0, 0.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            camera_mode = CameraMode::CAMERA_THIRD_PERSON;
            camera.up = Vector3::new(0.0, 1.0, 0.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            camera_mode = CameraMode::CAMERA_ORBITAL;
            camera.up = Vector3::new(0.0, 1.0, 0.0);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_P){
            if camera.camera_type() == CameraProjection::CAMERA_PERSPECTIVE {
                camera_mode = CameraMode::CAMERA_THIRD_PERSON;
                camera = Camera3D::orthographic(
                    Vector3::new(0.0, 2.0, -100.0), 
                    Vector3::new(0.0, 2.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0), 
                    20.0
                );
                
            }
            else if camera.camera_type() == CameraProjection::CAMERA_ORTHOGRAPHIC {
                camera_mode = CameraMode::CAMERA_THIRD_PERSON;
                camera = Camera3D::perspective(
                    Vector3::new(0.0, 2.0, 10.0), 
                    Vector3::new(0.0, 2.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0), 
                    60.0
                    
                );
            }
        }

        let mut draw_handler: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        

        let mut draw_3d: RaylibMode3D<'_, RaylibDrawHandle<'_>> = draw_handler.begin_mode3D(camera);
        // draw_3d.update_camera(&mut camera, camera_mode);

        draw_3d.update_camera_pro(&mut camera, 
            {
                //todo: implement this camera movement:
                /*
                UpdateCameraPro(&camera,
                    (Vector3){
                        (IsKeyDown(KEY_W) || IsKeyDown(KEY_UP))*0.1f -      // Move forward-backward
                        (IsKeyDown(KEY_S) || IsKeyDown(KEY_DOWN))*0.1f,    
                        (IsKeyDown(KEY_D) || IsKeyDown(KEY_RIGHT))*0.1f -   // Move right-left
                        (IsKeyDown(KEY_A) || IsKeyDown(KEY_LEFT))*0.1f,
                        0.0f                                                // Move up-down
                    },
                    (Vector3){
                        GetMouseDelta().x*0.05f,                            // Rotation: yaw
                        GetMouseDelta().y*0.05f,                            // Rotation: pitch
                        0.0f                                                // Rotation: roll
                    },
                GetMouseWheelMove()*2.0f);                              // Move to target (zoom)
                */

                let mut v = Vector3::new(0.0, 0.0, 0.0);
                if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_UP){

                }

                v
            }
            , rotation, zoom);

        draw_3d.clear_background(Color::RAYWHITE);
        draw_3d.draw_plane(Vector3::new(0.0, 0.0, 0.0), Vector2::new(32.0, 32.0), Color::LIGHTGRAY);
        draw_3d.draw_cube(Vector3::new(-16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::BLUE);
        draw_3d.draw_cube(Vector3::new(16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::LIME);
        draw_3d.draw_cube(Vector3::new(0.0, 2.5, 16.0), 32.0, 5.0, 1.0, Color::GOLD);

        
        if camera_mode == CameraMode::CAMERA_THIRD_PERSON {
            draw_3d.draw_cube(camera.target, 0.5, 0.5, 0.5, Color::PURPLE);
            draw_3d.draw_cube_wires(camera.target, 0.5, 0.5, 0.5, Color::DARKPURPLE);
        }
        
        //this is the only way to "escape" 3D mode
        std::mem::drop(draw_3d);
    }
}