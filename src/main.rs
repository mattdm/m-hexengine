extern crate sdl2;
extern crate rand;
extern crate toml;
extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

//use std::thread;
use docopt::Docopt;

mod interface;
mod direction;
mod hexgeometry;
mod landscape;
mod sprite;


const USAGE: &'static str = "
IsoHexEn Island Demo

Usage: isohexen [options]

-f, --fullscreen  Start Full Screen
";

pub fn main() {

    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(sdl2::image::INIT_PNG).unwrap();
    
    let mut window_builder = video_subsystem.window("Little Island", 960, 540);
    if args.get_bool("--fullscreen") {
        window_builder.fullscreen_desktop();
    }
    window_builder.resizable();
    let window = window_builder.build().unwrap();
 
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build().unwrap();
    canvas.window_mut().set_minimum_size(240,135);
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mouse_util = sdl_context.mouse();
    
//    thread::Builder::new().name("thread".to_string()).spawn(move || {
//        do_whatever();
//    }).unwrap();
    mouse_util.show_cursor(false);
    interface::splash(&mut canvas);
    interface::gameloop(&mut canvas,&mut event_pump, &mouse_util);
}
