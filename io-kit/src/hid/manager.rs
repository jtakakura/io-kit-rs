use core_foundation::base::{kCFAllocatorDefault, CFRelease, TCFType};

pub use io_kit_sys::hid::manager::*;

pub struct IOHIDManager(IOHIDManagerRef);

impl Drop for IOHIDManager {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl IOHIDManager {
    pub fn new() -> Option<Self> {
        let m = unsafe { IOHIDManagerCreate(kCFAllocatorDefault, kIOHIDManagerOptionNone) };

        if m.is_null() {
            None
        } else {
            Some(IOHIDManager(m))
        }
    }
}

impl_TCFType!(IOHIDManager, IOHIDManagerRef, IOHIDManagerGetTypeID);
