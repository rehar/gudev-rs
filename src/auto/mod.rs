// Generated by gir (https://github.com/gtk-rs/gir @ 6ed30fc)
// from gir-files (https://github.com/hfiguiere/gir-files.git @ e03533e)
// DO NOT EDIT

mod client;
pub use self::client::{Client, NONE_CLIENT};

mod device;
pub use self::device::{Device, NONE_DEVICE};

mod enumerator;
pub use self::enumerator::{Enumerator, NONE_ENUMERATOR};

mod enums;
pub use self::enums::DeviceType;

#[doc(hidden)]
pub mod traits {
    pub use super::client::ClientExt;
    pub use super::device::DeviceExt;
    pub use super::enumerator::EnumeratorExt;
}
