pub use ffi;

pub type DeviceNumber = ffi::GUdevDeviceNumber;

#[allow(unused_imports)]
mod auto;
mod device;

pub use auto::*;
pub use device::*;

pub mod prelude {
    pub use super::auto::traits::*;
    pub use super::device::traits::*;
}
