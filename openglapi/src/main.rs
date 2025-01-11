use sdl2::{Sdl, video::Window, event::Event, EventPump};
use std::time::Instant;

mod winsdl;
use winsdl::*;
mod objects;
use objects::*;

fn main() {

    let mut winsdl = Winsdl::new(800, 600).unwrap();

    let instant = Instant::now();

    'running: loop {
        for event in winsdl.event_pump.poll_iter(){
            match event {
                Event::Quit { .. } => break 'running,
                _  => {}
            }
        }

        unsafe {
            gl::ClearColor(0.9, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        winsdl.window.gl_swap_window();
    }

}
