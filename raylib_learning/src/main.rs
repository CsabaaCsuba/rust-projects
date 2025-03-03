use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello World!")
        .build();

    while !rl.window_should_close(){
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello world", 12, 12, 24, Color::WHITE);
        
        let cam = Camera3D::perspective(0, 0, Vector3::new(0, 1, 0), 90.0);

        
    }
}
