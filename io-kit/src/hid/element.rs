use core_foundation::base::{CFRelease, TCFType};

pub use io_kit_sys::hid::base::IOHIDElementRef;
pub use io_kit_sys::hid::element::*;

pub struct IOHIDElement(IOHIDElementRef);

impl Drop for IOHIDElement {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDElement, IOHIDElementRef, IOHIDElementGetTypeID);
