#[macro_use]
#[cfg(any(target_os = "linux", target_os="macos", target_os="windows"))]
pub mod cx_desktop;

#[macro_use]
pub mod cx_shared;

pub mod cx_stdin;

#[cfg(target_os = "macos")]
pub mod apple;

#[cfg(target_os = "macos")]
pub use crate::os::apple::*;

#[cfg(target_os = "macos")]
pub use crate::os::apple::apple_media::*;

#[cfg(target_os = "windows")]
pub mod mswindows;

#[cfg(target_os = "windows")]
pub use crate::os::mswindows::*;

#[cfg(target_os = "windows")]
pub use crate::os::mswindows::win32_media::*;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use crate::os::linux::*;

#[cfg(target_os = "linux")]
pub use crate::os::linux::linux_media::*;


#[cfg(target_arch = "wasm32")]
pub mod web_browser;

#[cfg(target_arch = "wasm32")]
pub use crate::os::web_browser::*;



