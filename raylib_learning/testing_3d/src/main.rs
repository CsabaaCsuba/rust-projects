use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

enum BlockType {
    
}

struct Block {
    block_type: BlockType,
    pos: Vector3,
}

fn draw_plain(d: &mut RaylibMode3D<'_, RaylibDrawHandle<'_>>, plain_length: i32, plain_width: i32){
    for i in 0..plain_length {
        for j in 0..plain_width {
            d.draw_cube_v(
                Vector3::new(0.0 as f32, (i*2) as f32 - 10.0, (j*2) as f32),
                Vector3::new(2.0, 2.0, 2.0),
                Color::RED
            );
        }  
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .msaa_4x()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Omg we are in 3D")
        .build();

    let cam = Camera3D::perspective(
        Vector3::new(0.0, 2.0, 0.0), 
        Vector3::new(4.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0), 
        60.0
    );
    
    
    
    while !rl.window_should_close(){
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);

        d.clear_background(Color::BLUE);
        d.draw_fps(5, 5);

        let mut draw_3d: RaylibMode3D<'_, RaylibDrawHandle<'_>> = d.begin_mode3D(cam);

        draw_plain(&mut draw_3d, 10, 10);


        
    }
}
