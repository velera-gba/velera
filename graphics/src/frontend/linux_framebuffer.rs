// Linux framebuffer graphics frontend

use super::*;

#[cfg(not(target_os = "linux"))]
compile_error!("This feature requires Linux system calls");

pub struct Frontend {
    // No choice but be static even though it is not
    framebuffer: Option<&'static mut [u32]>,

    fb_info: fb_var_screeninfo,
    scale: usize,
}

/// You must ensure that this structure gets dropped before program termination
#[cfg(target_os = "linux")]
impl Frontend {
    /// Setup the graphics stack with the Linux framebuffer frontend
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
        }

        if fb_info.bits_per_pixel != 32 {
            return Err(format!(
                "Unsupported format: {} bits per pixel is unsupported",
                fb_info.bits_per_pixel
            ));
        };

        let framebuffer_length = fb_info.xres as usize * fb_info.yres as usize;

        // map the framebuffer into memory
        let framebuffer = Some(unsafe {
            let framebuffer = mmap(
                std::ptr::null(),
                framebuffer_length * fb_info.bits_per_pixel as usize / 8,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fb_fd,
                0,
            ) as *mut u32;

            if framebuffer == -1 as isize as *mut u32 {
                return Err("Unable to mmap framebuffer".to_string());
            }

            std::slice::from_raw_parts_mut(framebuffer, framebuffer_length)
        });

        // Do not use fb_fd past this point
        if 0 != unsafe { close(fb_fd) } {
            eprintln!("Failed to close /dev/fb0. Please file a bug report."); // Should only fail if program is terminating, in which case this should not print
        }

        unsafe { Self::kd_raw()? };
        unsafe { signal(SIGINT, catch_signal) };
        unsafe { signal(SIGKILL, catch_signal) };

        Ok(Self {
            framebuffer,

            fb_info,
            scale: scale as usize,
        })
    }

    /// Enter MEDIUMRAW mode
    pub unsafe fn kd_raw() -> Result<(), String> {
        const STDIN: i32 = 0;

        if INIT_TC_STATE.is_some() || INIT_KB_MODE.is_some() { return Err("Terminal attempted to change state twice which is not allowed.".to_string()) }
        
        let mut term_state = termios::none();
        if tcgetattr(STDIN, &term_state as *const termios) != 0 {
            return Err("Unable to get terminal attributes".to_string())
        }
        INIT_TC_STATE = Some(term_state);

        // Make stdin nonblocking
        fcntl(STDIN, F_SETFL, fcntl(STDIN, F_GETFL) | O_NONBLOCK);

        let mut kb_mode = 0;
        if ioctl(STDIN, KDGKBMODE, &kb_mode as *const i32) < 0 {
            return Err("Unable to get keyboard mode. This application must be run in a tty".to_string())
        }
        INIT_KB_MODE = Some(kb_mode);

        term_state.c_lflag &= !(TC_ICANON | TC_ECHO | ISIG);
        term_state.c_iflag &= !(ICRNL | INLCR | ISTRIP) | IGNBRK;
        tcsetattr(STDIN, TCSANOW, &term_state as *const termios);

        if ioctl(STDIN, KDSKBMODE, K_MEDIUMRAW) < 0 {
            return Err("Unable to change keyboard mode to MEDIUMRAW".to_string())
        }

        Ok(())
    }

    /// Set the pixel at (x,y) to colour
    pub fn draw_pixel(&mut self, position: (usize, usize), colour: super::RGBA) {
        const FB_WIDTH: usize = 4;
        for x_scaled in 0..self.scale {
            for y_scaled in 0..self.scale {
                self.framebuffer.as_mut().unwrap()[position.0 * self.scale
                    + (self.fb_info.xres as usize * (position.1 * self.scale + y_scaled))
                    + x_scaled] = *colour;
            }
        }
    }

    /// Get input from the user
    pub fn get_input(&mut self) -> InputStates {
        let mut states = InputStates::new();

        if unsafe { SIG_END } { states.exit = true }

        use std::io::Read;

        // TODO: Allow rebinding as not all keyboards are alike
        let mut key = [0];
        while let Ok(_) = std::io::stdin().read(&mut key[..]) {
            match key[0] {
                1 => states.exit = true,
                _ => ()
            }
        };

        states
    }
}

// I would not consider this use unsafe because of how it is used
/// set when an SIGINT or SIGKILL was recieved
static mut SIG_END: bool = false;
extern "C" fn catch_signal(signal: i32) {
    unsafe { SIG_END = true }
}

// TODO: Actual unsafe usage if a signal is handled during exit
// The chance of this is astrronomically small; Inconcievable to get even if I tried
// Consider a lock anyway just to be sure
static mut INIT_KB_MODE: Option<i32> = None;
static mut INIT_TC_STATE: Option<termios> = None;


// Ensure that the terminal and keyboard mode get restored and that the framebuffer gets deallocated
impl Drop for Frontend {
    fn drop(&mut self) {
        const STDIN: i32 = 0;
        unsafe {
            if let Some(tc_state) = INIT_TC_STATE {
                tcsetattr(STDIN, TCSANOW, &tc_state as *const termios);
                INIT_TC_STATE = None;
            }
            if let Some(kb_mode) = INIT_KB_MODE {
                ioctl(STDIN, KDSKBMODE, kb_mode);
                INIT_KB_MODE = None;
            }
        }

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
    pub fn fcntl(fd: i32, cmd: i32, ...) -> i32;

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

// kd IOCTL constants
const KDGKBMODE: u64 = 0x4B44;
const KDSKBMODE: u64 = 0x4B45;
    // modes
const K_XLATE: u32 = 0x01;
const K_MEDIUMRAW: u32 = 0x02;

// fcntl constants
const O_RDWR: i32 = 2;
const O_NONBLOCK: i32 = 0o4000;
const F_GETFL: i32 = 3;
const F_SETFL: i32 = 4;

// mmap consts
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;

const MAP_FAILED: i32 = -1;
const MAP_SHARED: i32 = 0x1;

// termios constants
const IGNBRK: u32 = 0o001;
const ISTRIP: u32 = 0o040;
const INLCR: u32 = 0o100;
const ICRNL: u32 = 0o400;

const ISIG: u32 = 0o0001;
const TC_ECHO: u32 = 0o0010;
const TC_ICANON: u32 = 0o0002;
const TCSANOW: i32 = 0;
const TCIOFLUSH: i32 = 2;

// Signals
const SIGINT: i32 = 2;
const SIGKILL: i32 = 9;
