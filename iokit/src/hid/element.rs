use core_foundation::base::{CFRelease, TCFType};

use iokit_sys::hid::base::IOHIDElementRef;
use iokit_sys::hid::element::IOHIDElementGetTypeID;

pub struct IOHIDElement(IOHIDElementRef);

impl Drop for IOHIDElement {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDElement, IOHIDElementRef, IOHIDElementGetTypeID);
