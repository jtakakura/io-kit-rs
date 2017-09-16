use core_foundation::base::{CFRelease, TCFType};

pub use iokit_sys::hid::value::*;
use iokit_sys::hid::base::IOHIDValueRef;

pub struct IOHIDValue(IOHIDValueRef);

impl Drop for IOHIDValue {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDValue, IOHIDValueRef, IOHIDValueGetTypeID);
