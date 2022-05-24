// exports from <IOKit/hid/IOHIDDevice.h>

use std::os::raw::c_void;

use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::{CFAllocatorRef, CFIndex, CFTypeID, CFTypeRef};
use core_foundation_sys::date::CFTimeInterval;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::runloop::CFRunLoopRef;
use core_foundation_sys::string::CFStringRef;

use crate::{
    base::Boolean,
    hid::{
        base::{
            IOHIDCallback, IOHIDDeviceRef, IOHIDElementRef, IOHIDReportCallback,
            IOHIDReportWithTimeStampCallback, IOHIDValueCallback, IOHIDValueMultipleCallback,
            IOHIDValueRef,
        },
        keys::IOHIDReportType,
    },
    ret::IOReturn,
    types::{io_service_t, IOOptionBits},
};

extern "C" {
    pub fn IOHIDDeviceGetTypeID() -> CFTypeID;

    pub fn IOHIDDeviceCreate(allocator: CFAllocatorRef, service: io_service_t) -> IOHIDDeviceRef;

    pub fn IOHIDDeviceGetService(device: IOHIDDeviceRef) -> io_service_t;

    pub fn IOHIDDeviceOpen(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;

    pub fn IOHIDDeviceClose(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;

    pub fn IOHIDDeviceConformsTo(device: IOHIDDeviceRef, usagePage: u32, usage: u32) -> Boolean;

    pub fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;

    pub fn IOHIDDeviceSetProperty(
        device: IOHIDDeviceRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> Boolean;

    pub fn IOHIDDeviceCopyMatchingElements(
        device: IOHIDDeviceRef,
        matching: CFDictionaryRef,
        options: IOOptionBits,
    ) -> CFArrayRef;

    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    pub fn IOHIDDeviceUnscheduleFromRunLoop(
        device: IOHIDDeviceRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    pub fn IOHIDDeviceRegisterRemovalCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDCallback,
        context: *mut c_void,
    );

    pub fn IOHIDDeviceRegisterInputValueCallback(
        device: IOHIDDeviceRef,
        callback: IOHIDValueCallback,
        context: *mut c_void,
    );

    pub fn IOHIDDeviceRegisterInputReportCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        reportLength: CFIndex,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    );

    pub fn IOHIDDeviceRegisterInputReportWithTimeStampCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        reportLength: CFIndex,
        callback: IOHIDReportWithTimeStampCallback,
        context: *mut c_void,
    );

    pub fn IOHIDDeviceSetInputValueMatching(device: IOHIDDeviceRef, matching: CFDictionaryRef);

    pub fn IOHIDDeviceSetInputValueMatchingMultiple(device: IOHIDDeviceRef, multiple: CFArrayRef);

    pub fn IOHIDDeviceSetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
    ) -> IOReturn;

    pub fn IOHIDDeviceSetValueMultiple(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
    ) -> IOReturn;

    pub fn IOHIDDeviceSetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueCallback,
        context: *mut c_void,
    ) -> IOReturn;

    pub fn IOHIDDeviceSetValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueMultipleCallback,
        context: *mut c_void,
    ) -> IOReturn;

    pub fn IOHIDDeviceGetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
    ) -> IOReturn;

    pub fn IOHIDDeviceCopyValueMultiple(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        pMultiple: *mut CFDictionaryRef,
    ) -> IOReturn;

    pub fn IOHIDDeviceGetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        pValue: *mut IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueCallback,
        context: *mut c_void,
    ) -> IOReturn;

    pub fn IOHIDDeviceCopyValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        pMultiple: *mut CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: IOHIDValueMultipleCallback,
        context: *mut c_void,
    ) -> IOReturn;

    pub fn IOHIDDeviceSetReport(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *const u8,
        reportLength: CFIndex,
    ) -> IOReturn;

    pub fn IOHIDDeviceSetReportWithCallback(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *const u8,
        reportLength: CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    ) -> IOReturn;

    pub fn IOHIDDeviceGetReport(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *mut u8,
        pReportLength: *mut CFIndex,
    ) -> IOReturn;

    pub fn IOHIDDeviceGetReportWithCallback(
        device: IOHIDDeviceRef,
        reportType: IOHIDReportType,
        reportID: CFIndex,
        report: *mut u8,
        pReportLength: *mut CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    ) -> IOReturn;
}
