use core_foundation::base::{CFRelease, TCFType};

use io_kit_sys::hid::base::IOHIDValueRef;
pub use io_kit_sys::hid::value::*;

pub struct IOHIDValue(IOHIDValueRef);

impl Drop for IOHIDValue {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDValue, IOHIDValueRef, IOHIDValueGetTypeID);
