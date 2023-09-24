#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate core_foundation_sys;
extern crate mach2;

pub mod base;
pub mod keys;
pub mod ret;
pub mod types;

pub mod hid;
pub mod ps;
pub mod pwr_mgt;
pub mod serial;
pub mod usb;

use std::os::raw::{c_char, c_int, c_void};

use core_foundation_sys::base::{CFAllocatorRef, CFTypeRef};
use core_foundation_sys::dictionary::{CFDictionaryRef, CFMutableDictionaryRef};
use core_foundation_sys::runloop::CFRunLoopSourceRef;
use core_foundation_sys::string::CFStringRef;
use mach2::boolean::boolean_t;
use mach2::clock_types::mach_timespec_t;
use mach2::kern_return::kern_return_t;
use mach2::mach_types::task_port_t;
use mach2::message::mach_msg_header_t;
use mach2::port::mach_port_t;
use mach2::vm_types::{mach_vm_address_t, mach_vm_size_t};

use base::dispatch_queue_t;
use ret::IOReturn;
use types::{
    io_connect_t, io_iterator_t, io_object_t, io_registry_entry_t, io_service_t, IOOptionBits,
};

// exports from <CoreFoundation/CFString.h>
extern "C" {
    fn __CFStringMakeConstantString(cStr: *const c_char) -> CFStringRef;
}

pub fn CFSTR(cStr: *const c_char) -> CFStringRef {
    unsafe { __CFStringMakeConstantString(cStr) }
}

// exports from <IOKit/IOKitLib.h>
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IONotificationPort {
    _unused: [u8; 0],
}

pub type IONotificationPortRef = *mut IONotificationPort;

pub type IOServiceMatchingCallback =
    unsafe extern "C" fn(refcon: *mut c_void, iterator: io_iterator_t);

pub type IOServiceInterestCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    service: io_service_t,
    messageType: u32,
    messageArgument: *mut c_void,
);

extern "C" {
    #[link_name = "kIOMasterPortDefault"]
    pub static kIOMasterPortDefault: mach_port_t;

    pub fn IOMasterPort(bootstrapPort: mach_port_t, masterPort: *mut mach_port_t) -> kern_return_t;

    pub fn IONotificationPortCreate(masterPort: mach_port_t) -> IONotificationPortRef;

    pub fn IONotificationPortDestroy(notify: IONotificationPortRef);

    pub fn IONotificationPortGetRunLoopSource(notify: IONotificationPortRef) -> CFRunLoopSourceRef;

    pub fn IONotificationPortGetMachPort(notify: IONotificationPortRef) -> mach_port_t;

    pub fn IONotificationPortSetDispatchQueue(
        notify: IONotificationPortRef,
        queue: dispatch_queue_t,
    );

    pub fn IODispatchCalloutFromMessage(
        unused: *mut c_void,
        msg: *mut mach_msg_header_t,
        reference: *mut c_void,
    );

    pub fn IOCreateReceivePort(msgType: u32, recvPort: *mut mach_port_t) -> kern_return_t;
}

// IOObject
extern "C" {
    pub fn IOObjectRelease(object: io_object_t) -> kern_return_t;

    pub fn IOObjectRetain(object: io_object_t) -> kern_return_t;

    pub fn IOObjectGetClass(object: io_object_t, className: *mut c_char) -> kern_return_t;

    pub fn IOObjectCopyClass(object: io_object_t) -> CFStringRef;

    pub fn IOObjectCopySuperclassForClass(classname: CFStringRef) -> CFStringRef;

    pub fn IOObjectCopyBundleIdentifierForClass(classname: CFStringRef) -> CFStringRef;

    pub fn IOObjectConformsTo(object: io_object_t, className: *mut c_char) -> boolean_t;

    pub fn IOObjectIsEqualTo(object: io_object_t, anObject: io_object_t) -> boolean_t;

    pub fn IOObjectGetKernelRetainCount(object: io_object_t) -> u32;

    pub fn IOObjectGetUserRetainCount(object: io_object_t) -> u32;

    pub fn IOObjectGetRetainCount(object: io_object_t) -> u32;
}

// IOIterator, subclass of IOObject
extern "C" {
    pub fn IOIteratorNext(iterator: io_iterator_t) -> io_object_t;

    pub fn IOIteratorReset(iterator: io_iterator_t);

    pub fn IOIteratorIsValid(iterator: io_iterator_t) -> boolean_t;
}

// IOService, subclass of IORegistryEntry
extern "C" {
    pub fn IOServiceGetMatchingService(
        masterPort: mach_port_t,
        matching: CFDictionaryRef,
    ) -> io_service_t;

    pub fn IOServiceGetMatchingServices(
        masterPort: mach_port_t,
        matching: CFDictionaryRef,
        existing: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IOServiceAddNotification(
        masterPort: mach_port_t,
        notificationType: *mut c_char,
        matching: CFDictionaryRef,
        wakePort: mach_port_t,
        reference: usize,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IOServiceAddMatchingNotification(
        notifyPort: IONotificationPortRef,
        notificationType: *mut c_char,
        matching: CFDictionaryRef,
        callback: IOServiceMatchingCallback,
        refCon: *mut c_void,
        notification: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IOServiceAddInterestNotification(
        notifyPort: IONotificationPortRef,
        service: io_service_t,
        interestType: *mut c_char,
        callback: IOServiceInterestCallback,
        refCon: *mut c_void,
        notification: *mut io_object_t,
    ) -> kern_return_t;

    pub fn IOServiceMatchPropertyTable(
        service: io_service_t,
        matching: CFDictionaryRef,
        matches: *mut boolean_t,
    ) -> kern_return_t;

    pub fn IOServiceGetBusyState(service: io_service_t, busyState: *mut u32) -> kern_return_t;

    pub fn IOServiceWaitQuiet(
        service: io_service_t,
        waitTime: *mut mach_timespec_t,
    ) -> kern_return_t;

    pub fn IOKitGetBusyState(masterPort: mach_port_t, busyState: *mut u32) -> kern_return_t;

    pub fn IOKitWaitQuiet(masterPort: mach_port_t, waitTime: *mut mach_timespec_t)
        -> kern_return_t;

    pub fn IOServiceOpen(
        service: io_service_t,
        owningTask: task_port_t,
        type_: u32,
        connect: *mut io_connect_t,
    ) -> kern_return_t;

    pub fn IOServiceRequestProbe(service: io_service_t, options: u32) -> kern_return_t;
}

// options for IOServiceAuthorize()
pub const kIOServiceInteractionAllowed: u32 = 0x00000001;

extern "C" {
    pub fn IOServiceAuthorize(service: io_service_t, options: u32) -> kern_return_t;

    pub fn IOServiceOpenAsFileDescriptor(service: io_service_t, oflag: c_int) -> c_int;
}

// IOService connection
extern "C" {
    pub fn IOServiceClose(connect: io_connect_t) -> kern_return_t;

    pub fn IOConnectAddRef(connect: io_connect_t) -> kern_return_t;

    pub fn IOConnectRelease(connect: io_connect_t) -> kern_return_t;

    pub fn IOConnectGetService(connect: io_connect_t, service: *mut io_service_t) -> kern_return_t;

    pub fn IOConnectSetNotificationPort(
        connect: io_connect_t,
        type_: u32,
        port: mach_port_t,
        reference: usize,
    ) -> kern_return_t;

    pub fn IOConnectMapMemory(
        connect: io_connect_t,
        memoryType: u32,
        intoTask: task_port_t,
        atAddress: *mut mach_vm_address_t,
        ofSize: *mut mach_vm_size_t,
        options: IOOptionBits,
    ) -> kern_return_t;

    pub fn IOConnectMapMemory64(
        connect: io_connect_t,
        memoryType: u32,
        intoTask: task_port_t,
        atAddress: *mut mach_vm_address_t,
        ofSize: *mut mach_vm_size_t,
        options: IOOptionBits,
    ) -> kern_return_t;

    pub fn IOConnectUnmapMemory(
        connect: io_connect_t,
        memoryType: u32,
        fromTask: task_port_t,
        atAddress: mach_vm_address_t,
    ) -> kern_return_t;

    pub fn IOConnectUnmapMemory64(
        connect: io_connect_t,
        memoryType: u32,
        fromTask: task_port_t,
        atAddress: mach_vm_address_t,
    ) -> kern_return_t;

    pub fn IOConnectSetCFProperties(connect: io_connect_t, properties: CFTypeRef) -> kern_return_t;

    pub fn IOConnectSetCFProperty(
        connect: io_connect_t,
        propertyName: CFStringRef,
        property: CFTypeRef,
    ) -> kern_return_t;
}

// Combined LP64 & ILP32 Extended IOUserClient::externalMethod
extern "C" {
    pub fn IOConnectCallMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const u64,
        inputCnt: u32,
        inputStruct: *const c_void,
        inputStructCnt: usize,
        output: *mut u64,
        outputCnt: *mut u32,
        outputStruct: *mut c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;

    pub fn IOConnectCallAsyncMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        referenceCnt: u32,
        input: *const u64,
        inputCnt: u32,
        inputStruct: *const c_void,
        inputStructCnt: usize,
        output: *mut u64,
        outputCnt: *mut u32,
        outputStruct: *mut c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;
}
extern "C" {
    pub fn IOConnectCallStructMethod(
        connection: mach_port_t,
        selector: u32,
        inputStruct: *const c_void,
        inputStructCnt: usize,
        outputStruct: *mut c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;

    pub fn IOConnectCallAsyncStructMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        referenceCnt: u32,
        inputStruct: *const c_void,
        inputStructCnt: usize,
        outputStruct: *mut c_void,
        outputStructCnt: *mut usize,
    ) -> kern_return_t;

    pub fn IOConnectCallScalarMethod(
        connection: mach_port_t,
        selector: u32,
        input: *const u64,
        inputCnt: u32,
        output: *mut u64,
        outputCnt: *mut u32,
    ) -> kern_return_t;

    pub fn IOConnectCallAsyncScalarMethod(
        connection: mach_port_t,
        selector: u32,
        wake_port: mach_port_t,
        reference: *mut u64,
        referenceCnt: u32,
        input: *const u64,
        inputCnt: u32,
        output: *mut u64,
        outputCnt: *mut u32,
    ) -> kern_return_t;
}

extern "C" {
    pub fn IOConnectTrap0(connect: io_connect_t, index: u32) -> kern_return_t;

    pub fn IOConnectTrap1(connect: io_connect_t, index: u32, p1: usize) -> kern_return_t;

    pub fn IOConnectTrap2(connect: io_connect_t, index: u32, p1: usize, p2: usize)
        -> kern_return_t;

    pub fn IOConnectTrap3(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
    ) -> kern_return_t;

    pub fn IOConnectTrap4(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
    ) -> kern_return_t;

    pub fn IOConnectTrap5(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
        p5: usize,
    ) -> kern_return_t;

    pub fn IOConnectTrap6(
        connect: io_connect_t,
        index: u32,
        p1: usize,
        p2: usize,
        p3: usize,
        p4: usize,
        p5: usize,
        p6: usize,
    ) -> kern_return_t;
}

extern "C" {
    pub fn IOConnectAddClient(connect: io_connect_t, client: io_connect_t) -> kern_return_t;

    pub fn IORegistryGetRootEntry(masterPort: mach_port_t) -> io_registry_entry_t;

    pub fn IORegistryEntryFromPath(
        masterPort: mach_port_t,
        path: *mut c_char,
    ) -> io_registry_entry_t;

    pub fn IORegistryEntryCopyFromPath(
        masterPort: mach_port_t,
        path: CFStringRef,
    ) -> io_registry_entry_t;
}

pub const kIORegistryIterateRecursively: u32 = 0x00000001;
pub const kIORegistryIterateParents: u32 = 0x00000002;

extern "C" {
    pub fn IORegistryCreateIterator(
        masterPort: mach_port_t,
        plane: *const c_char,
        options: IOOptionBits,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IORegistryEntryCreateIterator(
        entry: io_registry_entry_t,
        plane: *const c_char,
        options: IOOptionBits,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;
}

// IORegistryIterator, subclass of IOIterator
extern "C" {
    pub fn IORegistryIteratorEnterEntry(iterator: io_iterator_t) -> kern_return_t;

    pub fn IORegistryIteratorExitEntry(iterator: io_iterator_t) -> kern_return_t;
}

// IORegistryEntry, subclass of IOObject
extern "C" {
    pub fn IORegistryEntryGetName(entry: io_registry_entry_t, name: *mut c_char) -> kern_return_t;

    pub fn IORegistryEntryGetNameInPlane(
        entry: io_registry_entry_t,
        plane: *const c_char,
        name: *mut c_char,
    ) -> kern_return_t;

    pub fn IORegistryEntryGetLocationInPlane(
        entry: io_registry_entry_t,
        plane: *const c_char,
        location: *mut c_char,
    ) -> kern_return_t;

    pub fn IORegistryEntryGetPath(
        entry: io_registry_entry_t,
        plane: *const c_char,
        path: *mut c_char,
    ) -> kern_return_t;

    pub fn IORegistryEntryCopyPath(entry: io_registry_entry_t, plane: *const c_char) -> CFStringRef;

    pub fn IORegistryEntryGetRegistryEntryID(
        entry: io_registry_entry_t,
        entryID: *mut u64,
    ) -> kern_return_t;

    pub fn IORegistryEntryCreateCFProperties(
        entry: io_registry_entry_t,
        properties: *mut CFMutableDictionaryRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> kern_return_t;

    pub fn IORegistryEntryCreateCFProperty(
        entry: io_registry_entry_t,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;

    pub fn IORegistryEntrySearchCFProperty(
        entry: io_registry_entry_t,
        plane: *const c_char,
        key: CFStringRef,
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> CFTypeRef;

    #[deprecated(
        since = "0.1.0",
        note = "please use `IORegistryEntryCreateCFProperty` instead"
    )]
    pub fn IORegistryEntryGetProperty(
        entry: io_registry_entry_t,
        propertyName: *mut c_char,
        buffer: *mut c_char,
        size: *mut u32,
    ) -> kern_return_t;

    pub fn IORegistryEntrySetCFProperties(
        entry: io_registry_entry_t,
        properties: CFTypeRef,
    ) -> kern_return_t;

    pub fn IORegistryEntrySetCFProperty(
        entry: io_registry_entry_t,
        propertyName: CFStringRef,
        property: CFTypeRef,
    ) -> kern_return_t;

    pub fn IORegistryEntryGetChildIterator(
        entry: io_registry_entry_t,
        plane: *mut c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IORegistryEntryGetChildEntry(
        entry: io_registry_entry_t,
        plane: *mut c_char,
        child: *mut io_registry_entry_t,
    ) -> kern_return_t;

    pub fn IORegistryEntryGetParentIterator(
        entry: io_registry_entry_t,
        plane: *mut c_char,
        iterator: *mut io_iterator_t,
    ) -> kern_return_t;

    pub fn IORegistryEntryGetParentEntry(
        entry: io_registry_entry_t,
        plane: *const c_char,
        parent: *mut io_registry_entry_t,
    ) -> kern_return_t;

    pub fn IORegistryEntryInPlane(entry: io_registry_entry_t, plane: *mut c_char) -> boolean_t;
}

// Matching dictionary creation helpers
extern "C" {
    pub fn IOServiceMatching(name: *const c_char) -> CFMutableDictionaryRef;

    pub fn IOServiceNameMatching(name: *const c_char) -> CFMutableDictionaryRef;

    pub fn IOBSDNameMatching(
        masterPort: mach_port_t,
        options: u32,
        bsdName: *const c_char,
    ) -> CFMutableDictionaryRef;

    pub fn IOOpenFirmwarePathMatching(
        masterPort: mach_port_t,
        options: u32,
        path: *const c_char,
    ) -> CFMutableDictionaryRef;

    pub fn IORegistryEntryIDMatching(entryID: u64) -> CFMutableDictionaryRef;

    pub fn IOServiceOFPathToBSDName(
        masterPort: mach_port_t,
        openFirmwarePath: *mut c_char,
        bsdName: *mut c_char,
    ) -> kern_return_t;
}

pub type IOAsyncCallback0 = unsafe extern "C" fn(refcon: *mut c_void, result: IOReturn);

pub type IOAsyncCallback1 =
    unsafe extern "C" fn(refcon: *mut c_void, result: IOReturn, arg0: *mut c_void);

pub type IOAsyncCallback2 = unsafe extern "C" fn(
    refcon: *mut c_void,
    result: IOReturn,
    arg0: *mut c_void,
    arg1: *mut c_void,
);

pub type IOAsyncCallback = unsafe extern "C" fn(
    refcon: *mut c_void,
    result: IOReturn,
    args: *mut *mut c_void,
    numArgs: u32,
);
