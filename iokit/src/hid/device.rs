use libc::c_char;

use core_foundation::base::{CFRelease, CFType, CFTypeID, TCFType, kCFAllocatorDefault};

pub use iokit_sys::hid::device::*;
use iokit_sys::CFSTR;
use iokit_sys::hid::base::IOHIDDeviceRef;
use iokit_sys::hid::keys::kIOHIDOptionsTypeNone;

use base::{IOService, TIOObject};
use ret::{IOReturn, kIOReturnSuccess};

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
            let result = IOHIDDeviceCreate(kCFAllocatorDefault, service.as_concrete_io_object_t());

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
                Err(IOReturn::from(result))
            }
        }
    }

    pub fn close(&self) -> Result<(), IOReturn> {
        unsafe {
            let result = IOHIDDeviceClose(self.0, kIOHIDOptionsTypeNone);

            if result == kIOReturnSuccess {
                Ok(())
            } else {
                Err(IOReturn::from(result))
            }

        }
    }

    pub fn conforms_to(&self, usage_page: u32, usage: u32) -> bool {
        unsafe { IOHIDDeviceConformsTo(self.0, usage_page, usage) != 0 }
    }

    pub fn get_property(&self, key: *const c_char) -> Option<CFType> {
        unsafe {
            let result = IOHIDDeviceGetProperty(self.0, CFSTR(key));

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }
}

impl_TCFType!(IOHIDDevice, IOHIDDeviceRef, IOHIDDeviceGetTypeID);
