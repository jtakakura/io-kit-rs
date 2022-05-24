// exports from <IOKit/hid/IOHIDValue.h>

use core_foundation_sys::base::{CFAllocatorRef, CFIndex, CFTypeID};

use crate::hid::{
    base::{IOHIDElementRef, IOHIDValueRef},
    keys::IOHIDValueScaleType
};

extern "C" {
    pub fn IOHIDValueGetTypeID() -> CFTypeID;

    pub fn IOHIDValueCreateWithIntegerValue(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        timeStamp: u64,
        value: CFIndex,
    ) -> IOHIDValueRef;

    pub fn IOHIDValueCreateWithBytes(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        timeStamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> IOHIDValueRef;

    pub fn IOHIDValueCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        timeStamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> IOHIDValueRef;

    pub fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;

    pub fn IOHIDValueGetTimeStamp(value: IOHIDValueRef) -> u64;

    pub fn IOHIDValueGetLength(value: IOHIDValueRef) -> CFIndex;

    pub fn IOHIDValueGetBytePtr(value: IOHIDValueRef) -> *const u8;

    pub fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> CFIndex;

    pub fn IOHIDValueGetScaledValue(value: IOHIDValueRef, type_: IOHIDValueScaleType) -> f64;
}
