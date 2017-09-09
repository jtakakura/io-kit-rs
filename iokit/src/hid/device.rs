use core_foundation::base::{CFRelease, TCFType};

use iokit_sys::ret::IOReturn;
use iokit_sys::hid::base::IOHIDDeviceRef;
use iokit_sys::hid::keys::kIOHIDOptionsTypeNone;
use iokit_sys::hid::device::{IOHIDDeviceGetTypeID, IOHIDDeviceConformsTo, IOHIDDeviceClose,
                             IOHIDDeviceOpen};

pub struct IOHIDDevice(IOHIDDeviceRef);

impl Drop for IOHIDDevice {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl IOHIDDevice {
    pub fn open(&self) -> IOReturn {
        unsafe { IOHIDDeviceOpen(self.0, kIOHIDOptionsTypeNone) }
    }

    pub fn close(&self) -> IOReturn {
        unsafe { IOHIDDeviceClose(self.0, kIOHIDOptionsTypeNone) }
    }

    pub fn conforms_to(&self, usage_page: u32, usage: u32) -> bool {
        unsafe { IOHIDDeviceConformsTo(self.0, usage_page, usage) != 0 }
    }
}

impl_TCFType!(IOHIDDevice, IOHIDDeviceRef, IOHIDDeviceGetTypeID);
