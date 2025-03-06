/*


*/


use std::ops::Range;

use raylib::prelude::*;

const MAX_COLUMNS: usize = 20;
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
    
    let mut heights: [f32;MAX_COLUMNS] = [0.0;MAX_COLUMNS];
    let mut positions: [Vector3;MAX_COLUMNS] = [Default::default();MAX_COLUMNS];
    let mut colors: [Color;MAX_COLUMNS] = [Default::default(); 20];

    let infobox_color: Color = rl.gui_fade(Color::SKYBLUE, 0.5);

    
    for i in 0..MAX_COLUMNS {
        heights[i] = rl.get_random_value::<i32>(Range { start: 0, end: 12 }) as f32;
        positions[i] = Vector3::new(
            rl.get_random_value::<i32>(Range { start: -15, end: 15 }) as f32,
            heights[i]/2.0,
            rl.get_random_value::<i32>(Range { start: -15, end: 15 }) as f32
        );
        colors[i] = Color::new(
            rl.get_random_value::<i32>(Range { start: 20, end: 255 }) as u8,
            rl.get_random_value::<i32>(Range { start: 20, end: 255 }) as u8,
            30,
            255
        );
    }
    
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
        draw_3d.update_camera(&mut camera, camera_mode);

        draw_3d.clear_background(Color::RAYWHITE);
        draw_3d.draw_plane(Vector3::new(0.0, 0.0, 0.0), Vector2::new(32.0, 32.0), Color::LIGHTGRAY);
        draw_3d.draw_cube(Vector3::new(-16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::BLUE);
        draw_3d.draw_cube(Vector3::new(16.0, 2.5, 0.0), 1.0, 5.0, 32.0, Color::LIME);
        draw_3d.draw_cube(Vector3::new(0.0, 2.5, 16.0), 32.0, 5.0, 1.0, Color::GOLD);

        
        
        for i in 0..MAX_COLUMNS {
            draw_3d.draw_cube(positions[i], 2.0, heights[i], 2.0, colors[i]);
            draw_3d.draw_cube_wires(positions[i], 2.0, heights[i], 2.0, Color::MAROON);
        }
        if camera_mode == CameraMode::CAMERA_THIRD_PERSON {
            draw_3d.draw_cube(camera.target, 0.5, 0.5, 0.5, Color::PURPLE);
            draw_3d.draw_cube_wires(camera.target, 0.5, 0.5, 0.5, Color::DARKPURPLE);
        }
        
        //this is the only way to "escape" 3D mode
        std::mem::drop(draw_3d);

        //infoboxes
        draw_handler.draw_rectangle(5, 5, 330, 100, infobox_color);
        draw_handler.draw_rectangle_lines(5, 5, 330, 100, Color::BLUE);
        
        draw_handler.draw_text("Camera controls:", 15, 15, 10, Color::BLACK);
        draw_handler.draw_text("- Move keys: W, A, S, D, Space, Left-Ctrl", 15, 30, 10, Color::BLACK);
        draw_handler.draw_text("- Look around: arrow keys or mouse", 15, 45, 10, Color::BLACK);
        draw_handler.draw_text("- Camera mode keys: 1, 2, 3, 4", 15, 60, 10, Color::BLACK);
        draw_handler.draw_text("- Zoom keys: num-plus, num-minus or mouse scroll", 15, 75, 10, Color::BLACK);
        draw_handler.draw_text("- Camera projection key: P", 15, 90, 10, Color::BLACK);
    
        draw_handler.draw_rectangle(600, 5, 195, 100, infobox_color);
        draw_handler.draw_rectangle_lines(600, 5, 195, 100, Color::BLUE);

        draw_handler.draw_text("Camera status:", 610, 15, 10, Color::BLACK);
        draw_handler.draw_text(&format!("Mode: {:?}", camera_mode), 610, 30, 10, Color::BLACK);
        draw_handler.draw_text(&format!("Projection: {:?}", camera.camera_type()), 610, 45, 10, Color::BLACK);
        draw_handler.draw_text(&format!("- Position: ({:.3}, {:.3}, {:.3})", camera.position.x, camera.position.y, camera.position.z), 610, 60, 10, Color::BLACK);
        draw_handler.draw_text(&format!("- Target: ({:.3}, {:.3}, {:.3})", camera.target.x, camera.target.y, camera.target.z), 610, 75, 10, Color::BLACK);
        draw_handler.draw_text(&format!("- Up: ({:.3}, {:.3}, {:.3})", camera.up.x, camera.up.y, camera.up.z), 610, 90, 10, Color::BLACK);
    }
}
