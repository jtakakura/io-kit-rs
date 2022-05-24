// exports from <IOKit/hid/IOHIDManager.h>

use std::os::raw::c_void;

use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{CFAllocatorRef, CFTypeID, CFTypeRef};
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::set::CFSetRef;
use core_foundation_sys::string::CFStringRef;

use crate::{
    base::Boolean,
    hid::base::{IOHIDDeviceCallback, IOHIDReportCallback, IOHIDValueCallback},
    ret::IOReturn,
    types::IOOptionBits,
};

pub type IOHIDManagerOptions = IOOptionBits;
pub const kIOHIDManagerOptionNone: IOHIDManagerOptions = 0x0;
pub const kIOHIDManagerOptionUsePersistentProperties: IOHIDManagerOptions = 0x1;
pub const kIOHIDManagerOptionDoNotLoadProperties: IOHIDManagerOptions = 0x2;
pub const kIOHIDManagerOptionDoNotSaveProperties: IOHIDManagerOptions = 0x4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDManager {
    _unused: [u8; 0],
}
pub type IOHIDManagerRef = *mut __IOHIDManager;

extern "C" {
    pub fn IOHIDManagerGetTypeID() -> CFTypeID;

    pub fn IOHIDManagerCreate(
        allocator: CFAllocatorRef,
        options: IOHIDManagerOptions,
    ) -> IOHIDManagerRef;

    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOHIDManagerOptions) -> IOReturn;

    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOHIDManagerOptions) -> IOReturn;

    pub fn IOHIDManagerGetProperty(manager: IOHIDManagerRef, key: CFStringRef) -> CFTypeRef;

    pub fn IOHIDManagerSetProperty(
        manager: IOHIDManagerRef,
        key: CFStringRef,
        value: CFTypeRef,
    ) -> Boolean;

    pub fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    pub fn IOHIDManagerUnscheduleFromRunLoop(
        manager: IOHIDManagerRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);

    pub fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDManagerRef, multiple: CFArrayRef);

    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;

    pub fn IOHIDManagerRegisterDeviceMatchingCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDDeviceCallback,
        context: *mut c_void,
    );

    pub fn IOHIDManagerRegisterDeviceRemovalCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDDeviceCallback,
        context: *mut c_void,
    );

    pub fn IOHIDManagerRegisterInputReportCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    );

    pub fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDValueCallback,
        context: *mut c_void,
    );

    pub fn IOHIDManagerSetInputValueMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);

    pub fn IOHIDManagerSetInputValueMatchingMultiple(
        manager: IOHIDManagerRef,
        multiple: CFArrayRef,
    );

    pub fn IOHIDManagerSaveToPropertyDomain(
        manager: IOHIDManagerRef,
        applicationID: CFStringRef,
        userName: CFStringRef,
        hostName: CFStringRef,
        options: IOOptionBits,
    );
}
