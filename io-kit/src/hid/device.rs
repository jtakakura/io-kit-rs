use std::os::raw::c_char;

use core_foundation::base::{kCFAllocatorDefault, CFRelease, CFType, CFTypeID, TCFType};

pub use io_kit_sys::hid::base::IOHIDDeviceRef;
pub use io_kit_sys::hid::device::*;
use io_kit_sys::hid::keys::kIOHIDOptionsTypeNone;
use io_kit_sys::CFSTR;

use crate::{
    base::{IOService, TIOObject},
    ret::{kIOReturnSuccess, IOReturn},
};

pub struct IOHIDDevice(IOHIDDeviceRef);

impl Drop for IOHIDDevice {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl IOHIDDevice {
    pub fn get_type_id() -> CFTypeID {
        unsafe { IOHIDDeviceGetTypeID() }
    }

    pub fn create(service: IOService) -> Option<IOHIDDevice> {
        unsafe {
            let result = IOHIDDeviceCreate(kCFAllocatorDefault, service.as_io_object_t());

            if result.is_null() {
                None
            } else {
                Some(IOHIDDevice(result))
            }
        }
    }

    pub fn open(&self) -> Result<(), IOReturn> {
        unsafe {
            let result = IOHIDDeviceOpen(self.0, kIOHIDOptionsTypeNone);

            if result == kIOReturnSuccess {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn close(&self) -> Result<(), IOReturn> {
        unsafe {
            let result = IOHIDDeviceClose(self.0, kIOHIDOptionsTypeNone);

            if result == kIOReturnSuccess {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn conforms_to(&self, usage_page: u32, usage: u32) -> bool {
        unsafe { IOHIDDeviceConformsTo(self.0, usage_page, usage) != 0 }
    }

    /// Gets a property from the device for the given key.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `key` is a valid, null-terminated C string.
    pub unsafe fn get_property(&self, key: *const c_char) -> Option<CFType> {
        let result = IOHIDDeviceGetProperty(self.0, CFSTR(key));

        if result.is_null() {
            None
        } else {
            Some(TCFType::wrap_under_get_rule(result))
        }
    }
}

impl_TCFType!(IOHIDDevice, IOHIDDeviceRef, IOHIDDeviceGetTypeID);
