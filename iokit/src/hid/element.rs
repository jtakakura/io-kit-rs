use core_foundation::base::{CFRelease, TCFType};

pub use iokit_sys::hid::element::*;
use iokit_sys::hid::base::IOHIDElementRef;

pub struct IOHIDElement(IOHIDElementRef);

impl Drop for IOHIDElement {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDElement, IOHIDElementRef, IOHIDElementGetTypeID);
