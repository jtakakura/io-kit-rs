// exports from <IOKit/hid/IOHIDManager.h>

use libc::c_void;
use core_foundation::base::{CFRelease, TCFType};
use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{CFAllocatorRef, CFTypeID, CFTypeRef, kCFAllocatorDefault};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::set::CFSetRef;
use core_foundation_sys::string::CFStringRef;

use base::Boolean;
use types::IOOptionBits;
use ret::IOReturn;
use hid::base::{IOHIDDeviceCallback, IOHIDReportCallback, IOHIDValueCallback};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IOHIDManagerOptions {
    kIOHIDManagerOptionNone = 0x0,
    kIOHIDManagerOptionUsePersistentProperties = 0x1,
    kIOHIDManagerOptionDoNotLoadProperties = 0x2,
    kIOHIDManagerOptionDoNotSaveProperties = 0x4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDManager {
    _unused: [u8; 0],
}
pub type IOHIDManagerRef = *mut __IOHIDManager;

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

extern "C" {
    pub fn IOHIDManagerGetTypeID() -> CFTypeID;

    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDManagerRef;

    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;

    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;

    pub fn IOHIDManagerGetProperty(manager: IOHIDManagerRef, key: CFStringRef) -> CFTypeRef;

    pub fn IOHIDManagerSetProperty(manager: IOHIDManagerRef,
                                   key: CFStringRef,
                                   value: CFTypeRef)
                                   -> Boolean;

    pub fn IOHIDManagerScheduleWithRunLoop(manager: IOHIDManagerRef,
                                           runLoop: CFRunLoopRef,
                                           runLoopMode: CFStringRef);

    pub fn IOHIDManagerUnscheduleFromRunLoop(manager: IOHIDManagerRef,
                                             runLoop: CFRunLoopRef,
                                             runLoopMode: CFStringRef);

    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);

    pub fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDManagerRef, multiple: CFArrayRef);

    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;

    pub fn IOHIDManagerRegisterDeviceMatchingCallback(manager: IOHIDManagerRef,
                                                      callback: IOHIDDeviceCallback,
                                                      context: *mut c_void);

    pub fn IOHIDManagerRegisterDeviceRemovalCallback(manager: IOHIDManagerRef,
                                                     callback: IOHIDDeviceCallback,
                                                     context: *mut c_void);

    pub fn IOHIDManagerRegisterInputReportCallback(manager: IOHIDManagerRef,
                                                   callback: IOHIDReportCallback,
                                                   context: *mut c_void);

    pub fn IOHIDManagerRegisterInputValueCallback(manager: IOHIDManagerRef,
                                                  callback: IOHIDValueCallback,
                                                  context: *mut c_void);

    pub fn IOHIDManagerSetInputValueMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);

    pub fn IOHIDManagerSetInputValueMatchingMultiple(manager: IOHIDManagerRef,
                                                     multiple: CFArrayRef);

    pub fn IOHIDManagerSaveToPropertyDomain(manager: IOHIDManagerRef,
                                            applicationID: CFStringRef,
                                            userName: CFStringRef,
                                            hostName: CFStringRef,
                                            options: IOOptionBits);
}
