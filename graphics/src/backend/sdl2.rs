extern crate sdl2;
use sdl2::IntegerOrSdlError::*;

use sdl2::{
    event::Event,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{ TextureAccess, WindowCanvas},
    video::WindowContext,
    EventPump, Sdl, VideoSubsystem,
};

#[cfg(feature = "vulkan")]
extern crate vulkano;

use super::*;

pub struct Backend {
    context: Sdl,
    video: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
}

impl Backend {
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

    pub fn draw_pixel(&mut self, position: (usize, usize), colour: RGBA) {
        self.canvas.set_draw_color::<(u8, u8, u8)>(colour.into());
        self.canvas.draw_point((position.0 as i32, position.1 as i32)).unwrap();
        self.canvas.present();
    }
}

/*#[cfg(test)]
mod tests {
    #[test]
    fn drawline_test() -> Result<(), String> {
        let mut video = super::Graphics::setup(4)?;
        let cache = video.graphics_cache();
        let mut texture = super::Graphics::instanciate_cache(&cache);

        let now = std::time::Instant::now();

        use super::State;
        loop {
            if now.elapsed() > std::time::Duration::from_secs(5) {
                break Ok(());
            }

            match video.drawline(&mut texture, 10, &[0xFF; 240 * 2]) {
                State::Exited => break Ok(()),
                _ => continue,
            }
        }
    }
}*/
