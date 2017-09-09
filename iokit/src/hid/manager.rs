use core_foundation::base::{CFRelease, TCFType};
use core_foundation_sys::base::kCFAllocatorDefault;

use iokit_sys::types::IOOptionBits;
use iokit_sys::hid::manager::{IOHIDManagerCreate, IOHIDManagerGetTypeID, IOHIDManagerOptions,
                              IOHIDManagerRef};

pub struct IOHIDManager(IOHIDManagerRef);

impl Drop for IOHIDManager {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl IOHIDManager {
    pub fn new() -> Option<Self> {
        let m = unsafe {
            IOHIDManagerCreate(kCFAllocatorDefault,
                               IOHIDManagerOptions::kIOHIDManagerOptionNone as IOOptionBits)
        };

        if m.is_null() {
            None
        } else {
            Some(IOHIDManager(m))
        }
    }
}

impl_TCFType!(IOHIDManager, IOHIDManagerRef, IOHIDManagerGetTypeID);
