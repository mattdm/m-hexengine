extern crate sdl2;

//use std::thread;

mod interface;


pub fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Little Island", 1280, 800)
        //.fullscreen_desktop()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas()
         .target_texture()
         .present_vsync()
        .build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    interface::gameloop(&mut canvas,&mut event_pump)

}
