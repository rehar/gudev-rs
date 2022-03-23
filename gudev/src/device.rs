use self::traits::DeviceExtManual;
use crate::{prelude::DeviceExt, Device};
use glib::{IsA, GString};

impl<O: IsA<Device>> DeviceExtManual for O {
    fn properties(&self) -> Vec<(GString, GString)> {
        let keys = self.property_keys();
        let mut out: Vec<(GString, GString)> = Vec::new();
        for key in keys {
            match self.property(key.as_str()) {
                Some(v) => out.push((key, v)),
                None => {}
            }
        }
        out
    }

    fn attributes(&self) -> Vec<(GString, GString)> {
        let keys = self.sysfs_attr_keys();
        let mut out: Vec<(GString, GString)> = Vec::new();
        for key in keys {
            match self.sysfs_attr(key.as_str()) {
                Some(v) => out.push((key, v)),
                None => {}
            }
        }
        out
    }
}

#[doc(hidden)]
pub mod traits {
    use glib::GString;
    /// Trait containing [`Device`](crate::Device) specific functions.
    pub trait DeviceExtManual {
        /// Retrieves a list of all available properties
        /// 
        /// # Returns
        /// 
        /// A vector containing a tuple of (Key, Value)
        fn properties(&self) -> Vec<(GString, GString)>;

        /// Retrieves a list of all available sysfs attributes
        /// 
        /// # Returns
        /// 
        /// A vector containing a tuple of (Key, Value)
        fn attributes(&self) -> Vec<(GString, GString)>;
    }
}
