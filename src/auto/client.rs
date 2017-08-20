// This file was generated by gir (9bd51ed) from gir-files (25c8aaf)
// DO NOT EDIT

use Device;
use DeviceNumber;
use DeviceType;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Client(Object<ffi::GUdevClient>);

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

pub trait ClientExt {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device>;

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device>;

    fn query_by_subsystem<'a, P: Into<Option<&'a str>>>(&self, subsystem: P) -> Vec<Device>;

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device>;

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device>;

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> u64;

    fn connect_property_subsystems_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Client> + IsA<glib::object::Object>> ClientExt for O {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_file(self.to_glib_none().0, device_file.to_glib_none().0))
        }
    }

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_number(self.to_glib_none().0, type_.to_glib(), number))
        }
    }

    fn query_by_subsystem<'a, P: Into<Option<&'a str>>>(&self, subsystem: P) -> Vec<Device> {
        let subsystem = subsystem.into();
        let subsystem = subsystem.to_glib_none();
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_udev_client_query_by_subsystem(self.to_glib_none().0, subsystem.0))
        }
    }

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_subsystem_and_name(self.to_glib_none().0, subsystem.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_sysfs_path(self.to_glib_none().0, sysfs_path.to_glib_none().0))
        }
    }

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "uevent",
                transmute(uevent_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_subsystems_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::subsystems",
                transmute(notify_subsystems_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn uevent_trampoline<P>(this: *mut ffi::GUdevClient, action: *mut libc::c_char, device: *mut ffi::GUdevDevice, f: glib_ffi::gpointer)
where P: IsA<Client> {
    callback_guard!();
    let f: &&(Fn(&P, &str, &Device) + 'static) = transmute(f);
    f(&Client::from_glib_none(this).downcast_unchecked(), &String::from_glib_none(action), &from_glib_none(device))
}

unsafe extern "C" fn notify_subsystems_trampoline<P>(this: *mut ffi::GUdevClient, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Client> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Client::from_glib_none(this).downcast_unchecked())
}
