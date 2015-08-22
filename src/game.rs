use std::thread;

use sdl2;
use sdl2::pixels::Color;
use sdl2::video::Window;

use events::Events;
use graphics::Graphics;

const FRAME_RATE: i64 = 60; //fps

pub struct Game<'engine> {
    sdl_context: &'engine sdl2::Sdl,
    display: Graphics<'engine>
}

impl<'e> Game<'e> {

    pub fn new(sdl_context: &'e sdl2::Sdl) -> Game<'e> {
        Game {
            sdl_context: sdl_context,
            display: Graphics::new(sdl_context)
        }
    }

    pub fn start(&mut self) {
        self.event_loop();
    }

    pub fn event_loop(&mut self) {

        let frame_delay: i64 = 1000 / FRAME_RATE as i64;
        let mut event_pipe = Events::new(self.sdl_context.event_pump().unwrap());

        'game_loop: loop {

            let start_time_ms = self.sdl_context.timer().unwrap().ticks();

            event_pipe.pump();

            if event_pipe.quit || event_pipe.key_escape {
                break 'game_loop;
            }

            self.update();

            self.render();

            let end_time_ms = self.sdl_context.timer().unwrap().ticks();
            let elapsed_time_ms = end_time_ms as i64 - start_time_ms as i64;
            let time_to_sleep = if frame_delay > elapsed_time_ms {
                frame_delay - elapsed_time_ms
            } else { 0 as i64 };

            thread::sleep_ms(time_to_sleep as u32);

        }

    }

    fn update(&self) {
        println!("updating...");
    }

    fn render(&mut self) {

        self.display.clear_buffers();
        println!("rendering...");
        self.display.switch_buffers();

    }

}
