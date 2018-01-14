extern crate sdl2;

// The event loop handles... events -- and also basic drawing.
    
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;


use sdl2::video;
use sdl2::render;

use sdl2::rect::Rect;

use sdl2::pixels::Color;

use std::time;
use std::thread;

use landscape;
use direction::Direction;
use sprite::SpriteAtlas;

fn drawmap(canvas: &mut render::WindowCanvas, sprite_atlas: &SpriteAtlas, map: &landscape::Island, orientation: Direction) {
    canvas.set_draw_color(Color::RGB(0,112,160));
    canvas.clear();

    // these should be actual center minus half a hex
    let center_x=8192-128;
    let center_y=4608-96;
    
    let drawstart = time::Instant::now();

    let map = map.get_ranked(orientation);
    println!("  Got Ranked {:?}: {}",orientation,(time::Instant::now()-drawstart).subsec_nanos()/1000000);


    for &(offset,hexstack,decorstack) in map.iter() {
    
        // long term improvement: read this from a 
        // text file describing the texture, rather
        // than hard-coding.
        
        let mut elevation=0;
        if hexstack.is_some() {
            for tile in hexstack.unwrap().iter() {
                //canvas.copy(&sprite_sheet, Rect::new(texturecol*256,texturerow.unwrap()*160,256,160), Rect::new(center_x+offset.0*32,center_y+offset.1*24-elevation*8,64,40)).expect("Render failed");
                //fixme: don't hardcode elevation (or scale!)
                sprite_atlas.draw(canvas, tile, 1, center_x+offset.0*128,center_y+offset.1*96-elevation*32,orientation);
                elevation += 1;
            }
        }
        if decorstack.is_some() {
            for decor in decorstack.unwrap().iter() {
                // FIXME: "draw-offset should be in sprite (but private to that sprite)
                sprite_atlas.draw(canvas, decor, 1, center_x+offset.0*128,center_y+offset.1*96-elevation*32,orientation);
                elevation += 1;
            }
        }
        
    }
    println!("  Map drawn:  {}",(time::Instant::now()-drawstart).subsec_nanos()/1000000);

    
    //sprite_atlas.draw(canvas, "compass", 1, 1664, 968,orientation);    

    //println!("  Compass:    {}",(time::Instant::now()-drawstart).subsec_nanos()/1000000);

}



pub fn gameloop(canvas: &mut render::WindowCanvas, event_pump: &mut sdl2::EventPump) {

    canvas.set_logical_size(1920,1080).unwrap();


    let texture_creator = canvas.texture_creator();

    // load the sprite atlas
    let sprite_atlas = SpriteAtlas::new(&texture_creator);

    // this is what the background gets rendered onto 
    // FIXME: put these constants somewhere as constants.
    let mut background_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 16384, 9216).unwrap();

    // create the map. in the future, we probably want some game-setup
    // function first before we go right into the game loop
    let mut islandmap = landscape::Island::new();
    
    // start 100 ms ago, so that we go right into drawing at the
    // bottom of the loop    
    let mut event_ticker = time::Instant::now() - time::Duration::from_millis(1000);
    let mut frame_ticker = event_ticker;
    
    // FIXME: add more sophisticated data structure for interface state
    // like zoom and stuff too
    let mut orientation=Direction::SE; // FIXME: use a diagonal to start?
    let mut map_x = 0;
    let mut map_y = 0;
    let mut zoom=4;
    
    
    let mut background_refresh_needed = true;
    
    islandmap.generate();

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop
                },
                /* AWEDXZ for panning in hex directions */
                Event::KeyDown { keycode: Some(Keycode::A), .. } |
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    // West
                    if map_x > -1024 {
                        map_x -= 8;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    // East
                    if map_x < 1024 {
                        map_x += 8;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    // North West
                    if map_x > -128 {
                        map_x -= 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    // North East
                    if map_x < 128 {
                        map_x += 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    // South West
                    if map_x > -128 {
                        map_x -= 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    // South East
                    if map_x < 128 {
                        map_x += 1;
                    }
                },
                /* Up and down for vertical scroll. Not sure I'll keep this. */
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if map_y > -1024 {
                        map_y -= 8;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if map_y < 1024 {
                        map_y += 8;
                    }
                },
                /* S is in the middle, so center ("senter"?) */
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    map_x = 0;
                    map_y = 0;
                },
                
                /* use Q and R for rotation. */
                Event::KeyDown { keycode: Some(Keycode::Q), .. } |
                Event::KeyDown { keycode: Some(Keycode::PageUp), .. } => {
                    orientation = orientation.counterclockwise();
                    background_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } |
                Event::KeyDown { keycode: Some(Keycode::PageDown), .. } => {
                    orientation = orientation.clockwise();
                    background_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::Equals), .. } => {
                    if zoom > 1 {
                        zoom -= 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                    if zoom < 8 {
                        zoom += 1;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, x: mx, y: my, .. } => {
                    if mx > 1112 && my > 688 {
                        orientation = orientation.counterclockwise();
                        background_refresh_needed = true;
                    }
                },
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, x: mx, y: my,.. } => {
                    if mx > 1112 && my > 688 {
                        orientation = orientation.clockwise();
                        background_refresh_needed = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    islandmap.generate();
                    background_refresh_needed = true;
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    match canvas.window_mut().fullscreen_state() {
                        video::FullscreenType::Off => canvas.window_mut().set_fullscreen(video::FullscreenType::Desktop).unwrap(),
                        video::FullscreenType::Desktop => canvas.window_mut().set_fullscreen(video::FullscreenType::Off).unwrap(),
                        video::FullscreenType::True => unreachable!(),
                    };
                },
                Event::Window {win_event,..} => {
                    match win_event {
                        WindowEvent::SizeChanged(_wx,_wy) => {
                            // Keep 16×9 aspect ratio
                            // FIXME: this doesn't really work (leaves strip of desktop in fullscreen!)
                            // Need to change the copy call instead
                            //canvas.set_viewport(Rect::new(0,0,wx as u32,((wx as u32)*9)/16));
                            canvas.set_logical_size(1920,1080).unwrap();
                        },
                        _ => { /* println!("{:?}",win_event); */ }
                    }
                    //println!("{:?}",win_event);
                
                },

                _ => {}
            }
        }
        
        // The rest of the game loop goes here...
        // Approximately 20fps        
        let next_tick = frame_ticker + time::Duration::from_millis(50);
        let now = time::Instant::now(); // fixme: better to call this only once per loop, but
        if now >= next_tick {
            if background_refresh_needed {
                canvas.with_texture_canvas(&mut background_texture, |texture_canvas| {
                    drawmap(texture_canvas, &sprite_atlas, &islandmap, orientation);
                }).unwrap();
                background_refresh_needed = false;
                println!("Background Refresh     : {}",(time::Instant::now()-now).subsec_nanos()/1000000);
            }

            let visible_w=1920*zoom; // FIXME: allow more zoom steps 
            let visible_h=1080*zoom;
            let background_x = 16384/2-visible_w/2+((map_x*(16384-visible_w))/2048);  // 2048 is our scroll range
            let background_y = 9216/2 -visible_h/2+((map_y*(9216 -visible_h))/2048);
            println!("Z: {} X: ({}-{}) Y: ({}-{})",zoom,background_x,background_x+visible_w,background_y,background_y+visible_h);
            canvas.copy(&background_texture,
                        Rect::new(background_x as i32, 
                                  background_y as i32,
                                  visible_w as u32,
                                  visible_h as u32),
                        None).expect("Render failed");
            sprite_atlas.draw(canvas, "compass", 1, 1664, 968,orientation);    


            canvas.present();

            frame_ticker = next_tick;
        }


        // but sleep around 10ms for event loop, because that's responsive enough
        let next_tick = event_ticker + time::Duration::from_millis(10);
        let now = time::Instant::now();
        if now < next_tick {
            thread::sleep(next_tick-now);
        }
        event_ticker = next_tick;
    }
}
