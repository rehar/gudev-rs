
extern crate libc;
#[macro_use]
extern crate glib;
extern crate gobject_sys as gobject_ffi;
extern crate glib_sys as glib_ffi;
extern crate gudev_sys as ffi;

pub use ffi::GUdevDeviceNumber as DeviceNumber;

pub use auto::*;
mod auto;
