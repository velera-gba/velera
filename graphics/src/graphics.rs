extern crate sdl2;
use sdl2::IntegerOrSdlError::*;

use sdl2::{
    event::Event,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Texture, TextureAccess, TextureCreator, WindowCanvas},
    video::WindowContext,
    EventPump, Sdl, VideoSubsystem,
};

#[cfg(feature = "vulkan")]
extern crate vulkano;

// Hide the SDL2 structures from the callee
pub type CacheObject = TextureCreator<WindowContext>;
pub type CacheInstance<'r> = Texture<'r>;

pub struct Graphics {
    context: Sdl,
    video: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl Graphics {
    pub fn setup(scale: u32) -> Result<Self, String> {
        let context = sdl2::init()?;
        let video = context.video()?;
        let window = match {
            let mut window_builder = video.window("Velera", 240 * scale, 160 * scale);

            #[cfg(feature = "vulkan")]
            window_builder.vulkan();

            window_builder.position_centered().build()
        } {
            Ok(window) => window,
            Err(error) => return Err(format!("Error building window: {}", error)),
        };
        let mut canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(IntegerOverflows(error, integer)) => {
                return Err(format!("{}: Caused by {}", error, integer))
            }
            Err(SdlError(error)) => return Err(error),
        };
        let event_pump = context.event_pump()?;

        // Initialise the window
        // TODO: Draw the logo to act as loading splash?
        canvas.set_scale(scale as f32, scale as f32)?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Self {
            context,
            video,
            canvas,
            event_pump,
        })
    }

    pub fn graphics_cache(&self) -> CacheObject {
        self.canvas.texture_creator()
    }

    pub fn instanciate_cache<'r>(cache: &'r CacheObject) -> CacheInstance<'r> {
        cache
            .create_texture(PixelFormatEnum::BGR555, TextureAccess::Streaming, 240, 1)
            .unwrap()
    }

    pub fn drawline<'r>(&mut self, cache_instance: &mut CacheInstance<'r>, y: usize, scanline: &[u8]) -> State {
        for event in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return State::Exited;
            }
        }

        cache_instance
            .update(None, scanline, 2)
            .expect("Fatal error in graphics stack: Issue with scanline texture");
        self.canvas
            .copy(cache_instance, None, Some(Rect::new(0, y as i32, 240, 1)))
            .expect("Fatal error in graphics stack: Issue with renderer");

        self.canvas.present();

        State::Running
    }
}

pub struct Interrupt {
    pub vblank:     bool,
    pub vcounter:   bool,
    pub hblank:     bool,
}

impl Interrupt {
    pub const fn none() -> Self {
        Self {
            vblank:     false,
            vcounter:   false,
            hblank:     false,
        }
    }

    pub fn vblank   (&mut self) { self.vblank   = true }
    pub fn vcounter (&mut self) { self.vcounter = true }
    pub fn hblank   (&mut self) { self.hblank   = true }
}

pub enum State {
    Exited,
    Running,
    Blanking
}

#[cfg(test)]
mod tests {
    #[test]
    fn drawline_test() -> Result<(), String> {
        let mut video = super::Graphics::setup(4)?;
        let cache = video.graphics_cache();
        let mut texture = super::Graphics::instanciate_cache(&cache);

        use super::State;
        loop {
            match video.drawline(&mut texture, 10, &[0xFF; 240 * 2]) {
                State::Exited => break Ok(()),
                _ => continue,
            }
        }
    }
}
