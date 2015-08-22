use sdl2;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Graphics<'g> {
    screen: Renderer<'g>
}

impl<'g> Graphics<'g> {

    pub fn new(sdl_context: &sdl2::Sdl) -> Graphics {
        let video_subsystem = sdl_context.video().unwrap();
        let window: Window = video_subsystem
            .window("Cave Story", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let renderer = window.renderer().build().unwrap();

        Graphics {
            screen: renderer
        }
    }

    pub fn clear_buffers(&mut self) {
        self.screen.clear();
        println!("Clearing buffers...");
    }

    pub fn switch_buffers(&mut self) {
        self.screen.set_draw_color(Color::RGB(0,0,255));
        self.screen.present();
        println!("Switching buffers...");
    }

}

