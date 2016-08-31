// This file was generated by gir (dc86d20) from gir-files (25c8aaf)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType, Volatile};

pub type GUdevDeviceNumber = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum GUdevDeviceType {
    None = 0,
    Block = 98,
    Char = 99,
}
pub const G_UDEV_DEVICE_TYPE_NONE: GUdevDeviceType = GUdevDeviceType::None;
pub const G_UDEV_DEVICE_TYPE_BLOCK: GUdevDeviceType = GUdevDeviceType::Block;
pub const G_UDEV_DEVICE_TYPE_CHAR: GUdevDeviceType = GUdevDeviceType::Char;






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

#[repr(C)]
pub struct GUdevClientPrivate(c_void);

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

#[repr(C)]
pub struct GUdevDevicePrivate(c_void);

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

#[repr(C)]
pub struct GUdevEnumeratorPrivate(c_void);


#[repr(C)]
pub struct GUdevClient(c_void);
#[repr(C)]
pub struct GUdevDevice(c_void);
#[repr(C)]
pub struct GUdevEnumerator(c_void);


extern "C" {

    //=========================================================================
    // GUdevClient
    //=========================================================================
    pub fn g_udev_client_get_type() -> GType;
    pub fn g_udev_client_new(subsystems: *mut *mut c_char) -> *mut GUdevClient;
    pub fn g_udev_client_query_by_device_file(client: *mut GUdevClient, device_file: *const c_char) -> *mut GUdevDevice;
    pub fn g_udev_client_query_by_device_number(client: *mut GUdevClient, type_: GUdevDeviceType, number: GUdevDeviceNumber) -> *mut GUdevDevice;
    pub fn g_udev_client_query_by_subsystem(client: *mut GUdevClient, subsystem: *const c_char) -> *mut glib::GList;
    pub fn g_udev_client_query_by_subsystem_and_name(client: *mut GUdevClient, subsystem: *const c_char, name: *const c_char) -> *mut GUdevDevice;
    pub fn g_udev_client_query_by_sysfs_path(client: *mut GUdevClient, sysfs_path: *const c_char) -> *mut GUdevDevice;

    //=========================================================================
    // GUdevDevice
    //=========================================================================
    pub fn g_udev_device_get_type() -> GType;
    pub fn g_udev_device_get_action(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_device_file(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_device_file_symlinks(device: *mut GUdevDevice) -> *mut *mut c_char;
    pub fn g_udev_device_get_device_number(device: *mut GUdevDevice) -> GUdevDeviceNumber;
    pub fn g_udev_device_get_device_type(device: *mut GUdevDevice) -> GUdevDeviceType;
    pub fn g_udev_device_get_devtype(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_driver(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_is_initialized(device: *mut GUdevDevice) -> gboolean;
    pub fn g_udev_device_get_name(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_number(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_parent(device: *mut GUdevDevice) -> *mut GUdevDevice;
    pub fn g_udev_device_get_parent_with_subsystem(device: *mut GUdevDevice, subsystem: *const c_char, devtype: *const c_char) -> *mut GUdevDevice;
    pub fn g_udev_device_get_property(device: *mut GUdevDevice, key: *const c_char) -> *const c_char;
    pub fn g_udev_device_get_property_as_boolean(device: *mut GUdevDevice, key: *const c_char) -> gboolean;
    pub fn g_udev_device_get_property_as_double(device: *mut GUdevDevice, key: *const c_char) -> c_double;
    pub fn g_udev_device_get_property_as_int(device: *mut GUdevDevice, key: *const c_char) -> c_int;
    pub fn g_udev_device_get_property_as_strv(device: *mut GUdevDevice, key: *const c_char) -> *mut *mut c_char;
    pub fn g_udev_device_get_property_as_uint64(device: *mut GUdevDevice, key: *const c_char) -> u64;
    pub fn g_udev_device_get_property_keys(device: *mut GUdevDevice) -> *mut *mut c_char;
    pub fn g_udev_device_get_seqnum(device: *mut GUdevDevice) -> u64;
    pub fn g_udev_device_get_subsystem(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_sysfs_attr(device: *mut GUdevDevice, name: *const c_char) -> *const c_char;
    pub fn g_udev_device_get_sysfs_attr_as_boolean(device: *mut GUdevDevice, name: *const c_char) -> gboolean;
    pub fn g_udev_device_get_sysfs_attr_as_double(device: *mut GUdevDevice, name: *const c_char) -> c_double;
    pub fn g_udev_device_get_sysfs_attr_as_int(device: *mut GUdevDevice, name: *const c_char) -> c_int;
    pub fn g_udev_device_get_sysfs_attr_as_strv(device: *mut GUdevDevice, name: *const c_char) -> *mut *mut c_char;
    pub fn g_udev_device_get_sysfs_attr_as_uint64(device: *mut GUdevDevice, name: *const c_char) -> u64;
    pub fn g_udev_device_get_sysfs_attr_keys(device: *mut GUdevDevice) -> *mut *mut c_char;
    pub fn g_udev_device_get_sysfs_path(device: *mut GUdevDevice) -> *const c_char;
    pub fn g_udev_device_get_tags(device: *mut GUdevDevice) -> *mut *mut c_char;
    pub fn g_udev_device_get_usec_since_initialized(device: *mut GUdevDevice) -> u64;
    pub fn g_udev_device_has_property(device: *mut GUdevDevice, key: *const c_char) -> gboolean;
    pub fn g_udev_device_has_sysfs_attr(device: *mut GUdevDevice, key: *const c_char) -> gboolean;

    //=========================================================================
    // GUdevEnumerator
    //=========================================================================
    pub fn g_udev_enumerator_get_type() -> GType;
    pub fn g_udev_enumerator_new(client: *mut GUdevClient) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_is_initialized(enumerator: *mut GUdevEnumerator) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_name(enumerator: *mut GUdevEnumerator, name: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_property(enumerator: *mut GUdevEnumerator, name: *const c_char, value: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_subsystem(enumerator: *mut GUdevEnumerator, subsystem: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_sysfs_attr(enumerator: *mut GUdevEnumerator, name: *const c_char, value: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_match_tag(enumerator: *mut GUdevEnumerator, tag: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_nomatch_subsystem(enumerator: *mut GUdevEnumerator, subsystem: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_nomatch_sysfs_attr(enumerator: *mut GUdevEnumerator, name: *const c_char, value: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_add_sysfs_path(enumerator: *mut GUdevEnumerator, sysfs_path: *const c_char) -> *mut GUdevEnumerator;
    pub fn g_udev_enumerator_execute(enumerator: *mut GUdevEnumerator) -> *mut glib::GList;

}