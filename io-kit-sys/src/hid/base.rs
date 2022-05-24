// exports from <IOKit/hid/IOHIDBase.h>

use std::os::raw::c_void;

use core_foundation_sys::base::CFIndex;
use core_foundation_sys::dictionary::CFDictionaryRef;

use crate::{hid::keys::IOHIDReportType, ret::IOReturn};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDDevice {
    _unused: [u8; 0],
}
pub type IOHIDDeviceRef = *mut __IOHIDDevice;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDElement {
    _unused: [u8; 0],
}
pub type IOHIDElementRef = *mut __IOHIDElement;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDValue {
    _unused: [u8; 0],
}
pub type IOHIDValueRef = *mut __IOHIDValue;

pub const kIOHIDTransactionDirectionTypeInput: u32 = 0;
pub const kIOHIDTransactionDirectionTypeOutput: u32 = 1;

pub const kIOHIDTransactionOptionDefaultOutputValue: u32 = 0x0001;

pub type IOHIDCallback =
    unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void);

pub type IOHIDReportCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    type_: IOHIDReportType,
    reportID: u32,
    report: *mut u8,
    reportLength: CFIndex,
);
pub type IOHIDReportWithTimeStampCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    type_: IOHIDReportType,
    reportID: u32,
    report: *mut u8,
    reportLength: CFIndex,
    timeStamp: u64,
);

pub type IOHIDValueCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    value: IOHIDValueRef,
);

pub type IOHIDValueMultipleCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    multiple: CFDictionaryRef,
);

pub type IOHIDDeviceCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    device: IOHIDDeviceRef,
);
