use std::sync::Arc;

use sdl3_sys::{
    events::{self, SDL_Event},
    video,
};

use crate::{device::GPUDevice, math::Rect};

#[derive(Debug)]
pub struct Window {
    window: *mut video::SDL_Window,
}

#[allow(dead_code)]
impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Arc<Self> {
        let cstr = std::ffi::CString::new(title).unwrap();

        let window = unsafe { video::SDL_CreateWindow(cstr.as_ptr(), width, height, 0) };

        Arc::new(Self { window })
    }

    pub fn should_close(&self) -> bool {
        let mut raw: SDL_Event = unsafe { std::mem::zeroed() };
        unsafe { events::SDL_PollEvent(&mut raw) };

        unsafe {
            match events::SDL_EventType(raw.r#type) {
                events::SDL_EVENT_QUIT => true,
                _ => false,
            }
        }
    }

    pub fn get_window(&self) -> *mut video::SDL_Window {
        self.window
    }

    pub fn set_window_position(&self, x: i32, y: i32) {
        unsafe {
            video::SDL_SetWindowPosition(self.window, x, y);
        }
    }

    pub fn set_window_size(&self, width: i32, height: i32) {
        unsafe {
            video::SDL_SetWindowSize(self.window, width, height);
        }
    }

    pub fn get_window_size(&self) -> Rect {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            video::SDL_GetWindowSize(self.window, &mut width, &mut height);
        }

        Rect::new(0, 0, width, height)
    }

    pub fn set_window_title(&self, title: &str) {
        let cstr = std::ffi::CString::new(title).unwrap();
        unsafe {
            video::SDL_SetWindowTitle(self.window, cstr.as_ptr());
        }
    }

    pub fn get_window_title(&self) -> String {
        let title = unsafe { video::SDL_GetWindowTitle(self.window) };
        let cstr = unsafe { std::ffi::CStr::from_ptr(title) };
        cstr.to_str().unwrap().to_string()
    }

    pub fn set_window_fullscreen(&self, fullscreen: bool) {
        unsafe {
            video::SDL_SetWindowFullscreen(self.window, fullscreen);
        }
    }

    pub fn create_gpu_device(self: &Arc<Window>) -> Result<Arc<GPUDevice>, String> {
        GPUDevice::new(self)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            video::SDL_DestroyWindow(self.window);
        }
    }
}
