// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 3ff4d3275258)
// from gudev-gir
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Aliases
pub type GUdevDeviceNumber = u64;

// Enums
pub type GUdevDeviceType = c_int;
pub const G_UDEV_DEVICE_TYPE_NONE: GUdevDeviceType = 0;
pub const G_UDEV_DEVICE_TYPE_BLOCK: GUdevDeviceType = 98;
pub const G_UDEV_DEVICE_TYPE_CHAR: GUdevDeviceType = 99;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUdevClientClass {
    pub parent_class: gobject::GObjectClass,
    pub uevent: Option<unsafe extern "C" fn(*mut GUdevClient, *const c_char, *mut GUdevDevice)>,
    pub reserved1: Option<unsafe extern "C" fn()>,
    pub reserved2: Option<unsafe extern "C" fn()>,
    pub reserved3: Option<unsafe extern "C" fn()>,
    pub reserved4: Option<unsafe extern "C" fn()>,
    pub reserved5: Option<unsafe extern "C" fn()>,
    pub reserved6: Option<unsafe extern "C" fn()>,
    pub reserved7: Option<unsafe extern "C" fn()>,
    pub reserved8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for GUdevClientClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GUdevClientClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("uevent", &self.uevent)
            .field("reserved1", &self.reserved1)
            .field("reserved2", &self.reserved2)
            .field("reserved3", &self.reserved3)
            .field("reserved4", &self.reserved4)
            .field("reserved5", &self.reserved5)
            .field("reserved6", &self.reserved6)
            .field("reserved7", &self.reserved7)
            .field("reserved8", &self.reserved8)
            .finish()
    }
}

#[repr(C)]
pub struct _GUdevClientPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GUdevClientPrivate = *mut _GUdevClientPrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUdevDeviceClass {
    pub parent_class: gobject::GObjectClass,
    pub reserved1: Option<unsafe extern "C" fn()>,
    pub reserved2: Option<unsafe extern "C" fn()>,
    pub reserved3: Option<unsafe extern "C" fn()>,
    pub reserved4: Option<unsafe extern "C" fn()>,
    pub reserved5: Option<unsafe extern "C" fn()>,
    pub reserved6: Option<unsafe extern "C" fn()>,
    pub reserved7: Option<unsafe extern "C" fn()>,
    pub reserved8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for GUdevDeviceClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GUdevDeviceClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("reserved1", &self.reserved1)
            .field("reserved2", &self.reserved2)
            .field("reserved3", &self.reserved3)
            .field("reserved4", &self.reserved4)
            .field("reserved5", &self.reserved5)
            .field("reserved6", &self.reserved6)
            .field("reserved7", &self.reserved7)
            .field("reserved8", &self.reserved8)
            .finish()
    }
}

#[repr(C)]
pub struct _GUdevDevicePrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GUdevDevicePrivate = *mut _GUdevDevicePrivate;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUdevEnumeratorClass {
    pub parent_class: gobject::GObjectClass,
    pub reserved1: Option<unsafe extern "C" fn()>,
    pub reserved2: Option<unsafe extern "C" fn()>,
    pub reserved3: Option<unsafe extern "C" fn()>,
    pub reserved4: Option<unsafe extern "C" fn()>,
    pub reserved5: Option<unsafe extern "C" fn()>,
    pub reserved6: Option<unsafe extern "C" fn()>,
    pub reserved7: Option<unsafe extern "C" fn()>,
    pub reserved8: Option<unsafe extern "C" fn()>,
}

impl ::std::fmt::Debug for GUdevEnumeratorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GUdevEnumeratorClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("reserved1", &self.reserved1)
            .field("reserved2", &self.reserved2)
            .field("reserved3", &self.reserved3)
            .field("reserved4", &self.reserved4)
            .field("reserved5", &self.reserved5)
            .field("reserved6", &self.reserved6)
            .field("reserved7", &self.reserved7)
            .field("reserved8", &self.reserved8)
            .finish()
    }
}

#[repr(C)]
pub struct _GUdevEnumeratorPrivate {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GUdevEnumeratorPrivate = *mut _GUdevEnumeratorPrivate;

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUdevClient {
    pub parent: gobject::GObject,
    pub priv_: *mut GUdevClientPrivate,
}

impl ::std::fmt::Debug for GUdevClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GUdevClient @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUdevDevice {
    pub parent: gobject::GObject,
    pub priv_: *mut GUdevDevicePrivate,
}

impl ::std::fmt::Debug for GUdevDevice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GUdevDevice @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GUdevEnumerator {
    pub parent: gobject::GObject,
    pub priv_: *mut GUdevEnumeratorPrivate,
}

impl ::std::fmt::Debug for GUdevEnumerator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GUdevEnumerator @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[link(name = "gudev-1.0")]
extern "C" {

    //=========================================================================
    // GUdevDeviceType
    //=========================================================================
    pub fn g_udev_device_type_get_type() -> GType;

    //=========================================================================
    // GUdevClient
    //=========================================================================
    pub fn g_udev_client_get_type() -> GType;
    pub fn g_udev_client_new(subsystems: *const *const c_char) -> *mut GUdevClient;
    pub fn g_udev_client_query_by_device_file(
        client: *mut GUdevClient,
        device_file: *const c_char,
    ) -> *mut GUdevDevice;
    pub fn g_udev_client_query_by_device_number(
        client: *mut GUdevClient,
        type_: GUdevDeviceType,
        number: GUdevDeviceNumber,
    ) -> *mut GUdevDevice;
    pub fn g_udev_client_query_by_subsystem(
        client: *mut GUdevClient,
        subsystem: *const c_char,
    ) -> *mut glib::GList;
    pub fn g_udev_client_query_by_subsystem_and_name(
        client: *mut GUdevClient,
        subsystem: *const c_char,
        name: *const c_char,
    ) -> *mut GUdevDevice;
    pub fn g_udev_client_query_by_sysfs_path(
        client: *mut GUdevClient,
        sysfs_path: *const c_char,
    ) -> *mut GUdevDevice;

    //=========================================================================
    // GUdevDevice
    //=========================================================================
    pub fn g_udev_device_get_type() -> GType;
    pub fn g_udev_device_get_action(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_device_file(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_device_file_symlinks(device: *mut GUdevDevice)
        -> *const *const c_char;
    pub fn g_udev_device_get_device_number(device: *mut GUdevDevice) -> GUdevDeviceNumber;
    pub fn g_udev_device_get_device_type(device: *mut GUdevDevice) -> GUdevDeviceType;
    pub fn g_udev_device_get_devtype(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_driver(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_is_initialized(device: *mut GUdevDevice) -> gboolean;
    pub fn g_udev_device_get_name(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_number(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_parent(device: *mut GUdevDevice) -> *mut GUdevDevice;
    pub fn g_udev_device_get_parent_with_subsystem(
        device: *mut GUdevDevice,
        subsystem: *const c_char,
        devtype: *const c_char,
    ) -> *mut GUdevDevice;
    pub fn g_udev_device_get_property(
        device: *mut GUdevDevice,
        key: *const c_char,
    ) -> *const c_char;
    pub fn g_udev_device_get_property_as_boolean(
        device: *mut GUdevDevice,
        key: *const c_char,
    ) -> gboolean;
    pub fn g_udev_device_get_property_as_double(
        device: *mut GUdevDevice,
        key: *const c_char,
    ) -> c_double;
    pub fn g_udev_device_get_property_as_int(device: *mut GUdevDevice, key: *const c_char)
        -> c_int;
    pub fn g_udev_device_get_property_as_strv(
        device: *mut GUdevDevice,
        key: *const c_char,
    ) -> *const *const c_char;
    pub fn g_udev_device_get_property_as_uint64(
        device: *mut GUdevDevice,
        key: *const c_char,
    ) -> u64;
    pub fn g_udev_device_get_property_keys(device: *mut GUdevDevice) -> *const *const c_char;
    pub fn g_udev_device_get_seqnum(device: *mut GUdevDevice) -> u64;
    pub fn g_udev_device_get_subsystem(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_sysfs_attr(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> *const c_char;
    pub fn g_udev_device_get_sysfs_attr_as_boolean(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> gboolean;
    pub fn g_udev_device_get_sysfs_attr_as_boolean_uncached(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> gboolean;
    pub fn g_udev_device_get_sysfs_attr_as_double(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> c_double;
    pub fn g_udev_device_get_sysfs_attr_as_double_uncached(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> c_double;
    pub fn g_udev_device_get_sysfs_attr_as_int(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> c_int;
    pub fn g_udev_device_get_sysfs_attr_as_int_uncached(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> c_int;
    pub fn g_udev_device_get_sysfs_attr_as_strv(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> *const *const c_char;
    pub fn g_udev_device_get_sysfs_attr_as_strv_uncached(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> *const *const c_char;
    pub fn g_udev_device_get_sysfs_attr_as_uint64(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> u64;
    pub fn g_udev_device_get_sysfs_attr_as_uint64_uncached(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> u64;
    pub fn g_udev_device_get_sysfs_attr_keys(device: *mut GUdevDevice) -> *const *const c_char;
    pub fn g_udev_device_get_sysfs_attr_uncached(
        device: *mut GUdevDevice,
        name: *const c_char,
    ) -> *const c_char;
    pub fn g_udev_device_get_sysfs_path(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_tags(device: *mut GUdevDevice) -> *const *const c_char;
    pub fn g_udev_device_get_usec_since_initialized(device: *mut GUdevDevice) -> u64;
    pub fn g_udev_device_has_property(device: *mut GUdevDevice, key: *const c_char) -> gboolean;
    pub fn g_udev_device_has_sysfs_attr(device: *mut GUdevDevice, key: *const c_char) -> gboolean;
    pub fn g_udev_device_has_sysfs_attr_uncached(
        device: *mut GUdevDevice,
        key: *const c_char,
    ) -> gboolean;

    //=========================================================================
    // GUdevEnumerator
    //=========================================================================
    pub fn g_udev_enumerator_get_type() -> GType;
    pub fn g_udev_enumerator_new(client: *mut GUdevClient) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_is_initialized(
        enumerator: *mut GUdevEnumerator,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_name(
        enumerator: *mut GUdevEnumerator,
        name: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_property(
        enumerator: *mut GUdevEnumerator,
        name: *const c_char,
        value: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_subsystem(
        enumerator: *mut GUdevEnumerator,
        subsystem: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_sysfs_attr(
        enumerator: *mut GUdevEnumerator,
        name: *const c_char,
        value: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_tag(
        enumerator: *mut GUdevEnumerator,
        tag: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_nomatch_subsystem(
        enumerator: *mut GUdevEnumerator,
        subsystem: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_nomatch_sysfs_attr(
        enumerator: *mut GUdevEnumerator,
        name: *const c_char,
        value: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_sysfs_path(
        enumerator: *mut GUdevEnumerator,
        sysfs_path: *const c_char,
    ) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_execute(enumerator: *mut GUdevEnumerator) -> *mut glib::GList;

}
