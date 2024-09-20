use std::{
    fmt::{Debug, Display},
    path::PathBuf,
};

use windows::Win32::Foundation::HWND;

use crate::{
    error_handler::Result,
    modules::virtual_desk::{get_vd_manager, VirtualDesktop},
    seelen_bar::FancyToolbar,
    seelen_rofi::SeelenRofi,
    seelen_wall::SeelenWall,
    seelen_weg::SeelenWeg,
    seelen_wm_v2::instance::WindowManagerV2,
};

use super::{monitor::Monitor, WindowEnumerator, WindowsApi};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window(HWND);

unsafe impl Send for Window {}

impl From<HWND> for Window {
    fn from(hwnd: HWND) -> Self {
        Self(hwnd)
    }
}

impl From<isize> for Window {
    fn from(addr: isize) -> Self {
        Self(HWND(addr as _))
    }
}

impl Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("handle", &self.0 .0)
            .field("title", &self.title())
            .field("class", &self.class())
            .field("exe", &self.exe())
            .finish()
    }
}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Window({:?})", self.0)
    }
}

pub const APP_FRAME_HOST_PATH: &str = "C:\\Windows\\System32\\ApplicationFrameHost.exe";
impl Window {
    pub fn hwnd(&self) -> HWND {
        self.0
    }

    pub fn address(&self) -> isize {
        self.0 .0 as isize
    }

    pub fn app_user_model_id(&self) -> Option<String> {
        WindowsApi::get_window_app_user_model_id(self.0).ok()
    }

    pub fn title(&self) -> String {
        WindowsApi::get_window_text(self.0)
    }

    pub fn class(&self) -> String {
        WindowsApi::get_class(self.0).unwrap_or_default()
    }

    /// will fail if process is restricted and the invoker is not running as admin
    pub fn exe(&self) -> Result<PathBuf> {
        WindowsApi::exe_path_v2(self.0)
    }

    pub fn app_display_name(&self) -> Result<String> {
        WindowsApi::get_window_display_name(self.0)
    }

    pub fn parent(&self) -> Option<Window> {
        let parent = WindowsApi::get_parent(self.0);
        if !parent.is_invalid() {
            Some(Window(parent))
        } else {
            None
        }
    }

    pub fn children(&self) -> Result<Vec<Window>> {
        WindowEnumerator::new()
            .with_parent(self.0)
            .map(Window::from)
    }

    pub fn monitor(&self) -> Monitor {
        Monitor::from(WindowsApi::monitor_from_window(self.0))
    }

    pub fn workspace(&self) -> Result<VirtualDesktop> {
        get_vd_manager().get_by_window(self.address())
    }

    pub fn is_window(&self) -> bool {
        WindowsApi::is_window(self.0)
    }

    pub fn is_visible(&self) -> bool {
        WindowsApi::is_window_visible(self.0)
    }

    pub fn is_minimized(&self) -> bool {
        WindowsApi::is_iconic(self.0)
    }

    pub fn is_maximized(&self) -> bool {
        WindowsApi::is_maximized(self.0)
    }

    pub fn is_cloaked(&self) -> bool {
        WindowsApi::is_cloaked(self.0).unwrap_or(false)
    }

    pub fn is_foreground(&self) -> bool {
        WindowsApi::get_foreground_window() == self.0
    }

    pub fn is_fullscreen(&self) -> bool {
        WindowsApi::is_fullscreen(self.0).unwrap_or(false)
    }

    /// is the window an Application Frame Host
    pub fn is_frame(&self) -> Result<bool> {
        Ok(self.exe()? == PathBuf::from(APP_FRAME_HOST_PATH))
    }

    /// will fail if the window is not a frame
    pub fn get_frame_creator(&self) -> Result<Option<Window>> {
        if !self.is_frame()? {
            return Err("Window is not a frame".into());
        }
        for window in self.children()? {
            if !window.class().starts_with("ApplicationFrame") {
                return Ok(Some(window));
            }
        }
        Ok(None)
    }

    pub fn is_desktop(&self) -> bool {
        WindowsApi::get_desktop_window() == self.0 || self.class() == "Progman"
    }

    pub fn is_seelen_overlay(&self) -> bool {
        if let Ok(exe) = self.exe() {
            return exe.ends_with("seelen-ui.exe")
                && [
                    FancyToolbar::TITLE,
                    WindowManagerV2::TITLE,
                    SeelenWeg::TITLE,
                    SeelenRofi::TITLE,
                    SeelenWall::TITLE,
                ]
                .contains(&self.title().as_str());
        }
        false
    }
}
