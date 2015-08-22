use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Events {

    event_pipe: EventPump,

    pub quit: bool,
    pub key_escape: bool
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            event_pipe: pump,
            quit: false,
            key_escape: false
        }
    }

    pub fn pump(&mut self) {
        for event in self.event_pipe.poll_iter() {
            match event {
                Event::Quit {..} => self.quit = true,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => self.key_escape = true,
                    _ => ()
                },
                Event::KeyUp{ keycode, .. } => match keycode {
                    Some(Keycode::Escape) => self.key_escape = false,
                    _ => ()
                },
                _ => ()
            }
        }
    }
}
