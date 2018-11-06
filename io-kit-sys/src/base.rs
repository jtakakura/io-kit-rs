use std::os::raw::c_uchar;

// exports from `MacTypes.h`
pub type Boolean = c_uchar;

// exports from `libdispatch`
#[repr(C)]
#[derive(Debug, Copy)]
pub struct dispatch_queue_s {
    _address: u8,
}
impl Clone for dispatch_queue_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub type dispatch_queue_t = *mut dispatch_queue_s;
