use core_foundation::base::{CFRelease, TCFType};

use iokit_sys::hid::base::IOHIDValueRef;
use iokit_sys::hid::value::IOHIDValueGetTypeID;

pub struct IOHIDValue(IOHIDValueRef);

impl Drop for IOHIDValue {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDValue, IOHIDValueRef, IOHIDValueGetTypeID);
