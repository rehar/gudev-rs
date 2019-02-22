// This file was generated by gir (https://github.com/gtk-rs/gir @ 2723216)
// from gir-files (https://github.com/gtk-rs/gir-files @ 25c8aaf)
// DO NOT EDIT

use Device;
use DeviceNumber;
use DeviceType;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Client(Object<ffi::GUdevClient, ffi::GUdevClientClass, ClientClass>);

    match fn {
        get_type => || ffi::g_udev_client_get_type(),
    }
}

impl Client {
    pub fn new(subsystems: &[&str]) -> Client {
        unsafe {
            from_glib_full(ffi::g_udev_client_new(subsystems.to_glib_none().0))
        }
    }
}

pub const NONE_CLIENT: Option<&Client> = None;

pub trait ClientExt: 'static {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device>;

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device>;

    fn query_by_subsystem<'a, P: Into<Option<&'a str>>>(&self, subsystem: P) -> Vec<Device>;

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device>;

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device>;

    fn get_property_subsystems(&self) -> Vec<GString>;

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Client>> ClientExt for O {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_file(self.as_ref().to_glib_none().0, device_file.to_glib_none().0))
        }
    }

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_number(self.as_ref().to_glib_none().0, type_.to_glib(), number))
        }
    }

    fn query_by_subsystem<'a, P: Into<Option<&'a str>>>(&self, subsystem: P) -> Vec<Device> {
        let subsystem = subsystem.into();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_udev_client_query_by_subsystem(self.as_ref().to_glib_none().0, subsystem.to_glib_none().0))
        }
    }

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_subsystem_and_name(self.as_ref().to_glib_none().0, subsystem.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_sysfs_path(self.as_ref().to_glib_none().0, sysfs_path.to_glib_none().0))
        }
    }

    fn get_property_subsystems(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"subsystems\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"uevent\0".as_ptr() as *const _,
                Some(transmute(uevent_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn uevent_trampoline<P, F: Fn(&P, &str, &Device) + 'static>(this: *mut ffi::GUdevClient, action: *mut libc::c_char, device: *mut ffi::GUdevDevice, f: glib_ffi::gpointer)
where P: IsA<Client> {
    let f: &F = transmute(f);
    f(&Client::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(action), &from_glib_borrow(device))
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}
