// Linux framebuffer graphics backend

#[cfg(not(target_os = "linux"))]
compile_error!("This feature requires Linux system calls");

pub struct Graphics {
    // No choice but be static even though it is not
    framebuffer: Option<&'static mut [u32]>,

    fb_info: fb_var_screeninfo,
    scale: usize,
}

/// Dummy structs to comply with the SDL2 Graphics interface
pub type CacheObject = ();
/// Dummy structs to comply with the SDL2 Graphics interface
pub type CacheInstance<'r> = &'r ();

#[cfg(target_os = "linux")]
impl Graphics {
    /// Setup the graphics stack with the Linux framebuffer backend
    /// Uses Linux system calls
    pub fn setup(scale: u32) -> Result<Self, String> {
        // Unsafe as using system calls
        let fb_fd = unsafe { open("/dev/fb0\0".as_ptr(), O_RDWR) };
        if fb_fd == 0 {
            return Err(format!("Unable to open /dev/fb0"));
        }

        let fb_info: fb_var_screeninfo = unsafe { std::mem::zeroed() };
        if 0 != unsafe { ioctl(fb_fd, FBIOGET_VSCREENINFO, &fb_info) } {
            return Err("Error getting framebuffer info".to_string());
        };

        let framebuffer_length = fb_info.xres as usize * fb_info.yres as usize;

        // map the framebuffer into memory
        let framebuffer = Some(unsafe {
            let framebuffer = mmap(
                std::ptr::null(),
                framebuffer_length * 4,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fb_fd,
                0,
            ) as *mut u32;

            if framebuffer == -1 as isize as *mut u32 {
                return Err("Unable to mmap framebuffer".to_string());
            }

            unsafe { std::slice::from_raw_parts_mut(framebuffer, framebuffer_length) }
        });

        // Do not use fb_fd past this point
        if 0 != unsafe { close(fb_fd) } {
            eprintln!("Failed to close /dev/fb0. Please file a bug report."); // Should only fail if program is terminating, in which case this should not print
        }

        Ok(Self {
            framebuffer,

            fb_info,
            scale: scale as usize,
        })
    }

    pub fn graphics_cache(&self) -> CacheObject {
        ()
    }

    pub fn instanciate_cache<'r>(cache: &'r CacheObject) -> CacheInstance<'r> {
        cache
    }

    pub fn drawline<'r>(
        &mut self,
        cache_instance: &mut CacheInstance<'r>,
        y: usize,
        scanline: &[u8],
    ) -> State {
        for (x, pixel) in scanline
            .chunks_exact(2)
            .map(|bgr| Self::bgr555_to_rgb888(bgr))
            .enumerate()
        {
            self.set_pixel((x,y), pixel).unwrap();
        }

        State::Running
    }

    /// Set the pixel at (x,y) to colour
    /// Returns None if an error occured
    pub fn set_pixel(&mut self, pos: (usize, usize), mut colour: u32) -> Option<()> {

        const FB_WIDTH: usize = 4;
        for x_scaled in 0..self.scale {
            for y_scaled in 0..self.scale {
                let index = pos.0 * self.scale + (self.fb_info.xres as usize * (pos.1 * self.scale + y_scaled)) + x_scaled;
                unsafe {
                    self.framebuffer.as_mut().unwrap()[index] = colour;
                }
            }
        }

        Some(())
    }

    pub fn bgr555_to_rgb888(bgr: &[u8]) -> u32 {
        let (msb, lsb) = (bgr[1] as u32, bgr[0] as u32);
        let red     = (lsb & 0b11111) as f32;
        let green   = ((lsb & 0b11100000) >> 3 | msb & 0b11) as f32;
        let blue    = (msb & 0b01111100 >> 2) as f32;
        ((red/31.0*255.0) as u32) << 16 | ((green/31.0*255.0) as u32) << 8 | (blue/31.0*255.0) as u32
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {

        let ptr = self.framebuffer.as_mut().unwrap().as_ptr() as *mut void;
        let len = self.framebuffer.as_mut().unwrap().len();
        self.framebuffer = None;

        match unsafe { munmap(ptr, len) } {
            0 => (),
            // Prevent a memory leak
            -1 => panic!("Failed to unmap framebuffer memory. Please file a bug report"),
            // If this occurs we have probably linked to the wrong munmap
            _ => panic!("munmap returned a value which does not match the specifications"),
        }
    }
}

pub struct Interrupt {
    pub vblank: bool,
    pub vcounter: bool,
    pub hblank: bool,
}

impl Interrupt {
    pub const fn none() -> Self {
        Self {
            vblank: false,
            vcounter: false,
            hblank: false,
        }
    }

    pub fn vblank(&mut self) {
        self.vblank = true
    }
    pub fn vcounter(&mut self) {
        self.vcounter = true
    }
    pub fn hblank(&mut self) {
        self.hblank = true
    }
}

pub enum State {
    Exited,
    Running,
    Blanking,
}

// Syscall definitions
#[cfg(target_os = "linux")]
extern "C" {
    // Linux syscalls
    pub fn ioctl(fd: i32, request: u64, ...) -> i32;
    pub fn open(file: *const u8, oflag: i32, ...) -> i32;
    pub fn close(fd: i32) -> i32;
    pub fn signal(sig: i32, handler: extern "C" fn(i32)) -> extern "C" fn(i32);
    pub fn mmap(
        addr: *const void,
        length: usize,
        prot: i32,
        flags: i32,
        fd: i32,
        offset: isize,
    ) -> *mut void;
    pub fn munmap(addr: *const void, length: usize) -> i32;

    // termios functions for capturing input better
    pub fn tcgetattr(fd: i32, termios_p: *const termios) -> i32;
    pub fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const termios) -> i32;
    pub fn tcflush(fd: i32, queue_selector: i32) -> i32;
}

use std::ffi::c_void as void;

#[repr(C)]
pub struct fb_bitfield {
    offset: u32,
    length: u32,
    msb_right: u32,
}

#[repr(C)]
pub struct fb_var_screeninfo {
    xres: u32,
    yres: u32,
    xres_virtual: u32,
    yres_virtual: u32,
    xoffset: u32,
    yoffset: u32,

    bits_per_pixel: u32,
    grayscale: u32,

    red: fb_bitfield,
    green: fb_bitfield,
    blue: fb_bitfield,
    transp: fb_bitfield,

    non_std: u32,
    activate: u32,

    height: u32,
    width: u32,

    accel_flags: u32,

    pixclock: u32,
    left_margin: u32,
    right_margin: u32,
    upper_margin: u32,
    lower_margin: u32,
    hsync_len: u32,
    vsync_len: u32,
    sync: u32,
    vmode: u32,
    rotate: u32,
    colorspace: u32,
    reserved: [u32; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,

    c_line: u8,
    c_cc: [u8; 32],

    c_ispeed: u32,
    c_ospeed: u32,
}

impl termios {
    pub const fn none() -> Self {
        Self {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,

            c_line: 0,
            c_cc: [0; 32],

            c_ispeed: 0,
            c_ospeed: 0,
        }
    }
}

// Constants

// fb IOCTL constants
const FBIOGET_VSCREENINFO: u64 = 0x4600;

// fcntl modes
const O_RDWR: i32 = 2;

// mmap consts
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;

const MAP_FAILED: i32 = -1;
const MAP_SHARED: i32 = 0x1;

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
