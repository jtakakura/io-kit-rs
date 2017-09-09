// exports from <IOKit/hid/IOHIDBase.h>

use libc::c_void;
use core_foundation::base::{CFRelease, TCFType};
use core_foundation_sys::base::CFIndex;
use core_foundation_sys::dictionary::CFDictionaryRef;

use ret::IOReturn;
use hid::keys::{IOHIDReportType, kIOHIDOptionsTypeNone};
use hid::device::{IOHIDDeviceGetTypeID, IOHIDDeviceConformsTo, IOHIDDeviceClose, IOHIDDeviceOpen};
use hid::element::IOHIDElementGetTypeID;
use hid::value::IOHIDValueGetTypeID;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDDevice {
    _unused: [u8; 0],
}
pub type IOHIDDeviceRef = *mut __IOHIDDevice;

pub struct IOHIDDevice(IOHIDDeviceRef);

impl Drop for IOHIDDevice {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl IOHIDDevice {
    pub fn open(&self) -> IOReturn {
        unsafe { IOHIDDeviceOpen(self.0, kIOHIDOptionsTypeNone) }
    }

    pub fn close(&self) -> IOReturn {
        unsafe { IOHIDDeviceClose(self.0, kIOHIDOptionsTypeNone) }
    }

    pub fn conforms_to(&self, usage_page: u32, usage: u32) -> bool {
        unsafe { IOHIDDeviceConformsTo(self.0, usage_page, usage) != 0 }
    }
}

impl_TCFType!(IOHIDDevice, IOHIDDeviceRef, IOHIDDeviceGetTypeID);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDElement {
    _unused: [u8; 0],
}
pub type IOHIDElementRef = *mut __IOHIDElement;

pub struct IOHIDElement(IOHIDElementRef);

impl Drop for IOHIDElement {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDElement, IOHIDElementRef, IOHIDElementGetTypeID);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __IOHIDValue {
    _unused: [u8; 0],
}
pub type IOHIDValueRef = *mut __IOHIDValue;

pub struct IOHIDValue(IOHIDValueRef);

impl Drop for IOHIDValue {
    fn drop(&mut self) {
        unsafe { CFRelease(self.as_CFTypeRef()) }
    }
}

impl_TCFType!(IOHIDValue, IOHIDValueRef, IOHIDValueGetTypeID);

pub const kIOHIDTransactionDirectionTypeInput: u32 = 0;
pub const kIOHIDTransactionDirectionTypeOutput: u32 = 1;

pub const kIOHIDTransactionOptionDefaultOutputValue: u32 = 0x0001;

pub type IOHIDCallback = Option<unsafe extern "C" fn(context: *mut c_void,
                                                     result: IOReturn,
                                                     sender: *mut c_void)
                                                    >;

pub type IOHIDReportCallback = Option<unsafe extern "C" fn(context: *mut c_void,
                                                           result: IOReturn,
                                                           sender: *mut c_void,
                                                           type_: IOHIDReportType,
                                                           reportID: u32,
                                                           report: *mut u8,
                                                           reportLength: CFIndex)
                                                          >;
pub type IOHIDReportWithTimeStampCallback = Option<unsafe extern "C" fn(context: *mut c_void,
                                                                        result: IOReturn,
                                                                        sender: *mut c_void,
                                                                        type_: IOHIDReportType,
                                                                        reportID: u32,
                                                                        report: *mut u8,
                                                                        reportLength: CFIndex,
                                                                        timeStamp: u64)
                                                                       >;

pub type IOHIDValueCallback = Option<unsafe extern "C" fn(context: *mut c_void,
                                                          result: IOReturn,
                                                          sender: *mut c_void,
                                                          value: IOHIDValueRef)
                                                         >;

pub type IOHIDValueMultipleCallback = Option<unsafe extern "C" fn(context: *mut c_void,
                                                                  result: IOReturn,
                                                                  sender: *mut c_void,
                                                                  multiple: CFDictionaryRef)
                                                                 >;

pub type IOHIDDeviceCallback = Option<unsafe extern "C" fn(context: *mut c_void,
                                                           result: IOReturn,
                                                           sender: *mut c_void,
                                                           device: IOHIDDeviceRef)
                                                          >;
