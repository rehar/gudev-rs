// This file was generated by gir (https://github.com/gtk-rs/gir @ 60cbef0)
// from gir-files (https://github.com/gtk-rs/gir-files @ 25c8aaf)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gudev_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use DeviceNumber;
use DeviceType;

glib_wrapper! {
    pub struct Client(Object<gudev_sys::GUdevClient, gudev_sys::GUdevClientClass, ClientClass>);

    match fn {
        get_type => || gudev_sys::g_udev_client_get_type(),
    }
}

impl Client {
    pub fn new(subsystems: &[&str]) -> Client {
        unsafe {
            from_glib_full(gudev_sys::g_udev_client_new(subsystems.to_glib_none().0))
        }
    }
}

pub const NONE_CLIENT: Option<&Client> = None;

pub trait ClientExt: 'static {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device>;

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device>;

    fn query_by_subsystem(&self, subsystem: Option<&str>) -> Vec<Device>;

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device>;

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device>;

    fn get_property_subsystems(&self) -> Vec<GString>;

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Client>> ClientExt for O {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device> {
        unsafe {
            from_glib_full(gudev_sys::g_udev_client_query_by_device_file(self.as_ref().to_glib_none().0, device_file.to_glib_none().0))
        }
    }

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device> {
        unsafe {
            from_glib_full(gudev_sys::g_udev_client_query_by_device_number(self.as_ref().to_glib_none().0, type_.to_glib(), number))
        }
    }

    fn query_by_subsystem(&self, subsystem: Option<&str>) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(gudev_sys::g_udev_client_query_by_subsystem(self.as_ref().to_glib_none().0, subsystem.to_glib_none().0))
        }
    }

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device> {
        unsafe {
            from_glib_full(gudev_sys::g_udev_client_query_by_subsystem_and_name(self.as_ref().to_glib_none().0, subsystem.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device> {
        unsafe {
            from_glib_full(gudev_sys::g_udev_client_query_by_sysfs_path(self.as_ref().to_glib_none().0, sysfs_path.to_glib_none().0))
        }
    }

    fn get_property_subsystems(&self) -> Vec<GString> {
        unsafe {
            let mut value = Value::from_type(<Vec<GString> as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"subsystems\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `subsystems` getter").unwrap()
        }
    }

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn uevent_trampoline<P, F: Fn(&P, &str, &Device) + 'static>(this: *mut gudev_sys::GUdevClient, action: *mut libc::c_char, device: *mut gudev_sys::GUdevDevice, f: glib_sys::gpointer)
            where P: IsA<Client>
        {
            let f: &F = &*(f as *const F);
            f(&Client::from_glib_borrow(this).unsafe_cast_ref(), &GString::from_glib_borrow(action), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"uevent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(uevent_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Client")
    }
}
