// exports from <IOKit/IOTypes.h>

use std::os::raw::{c_char, c_int, c_uint, c_ulonglong};

use mach2::port::mach_port_t;
use mach2::vm_types::mach_vm_address_t;

pub type IOOptionBits = c_uint;
pub type IOFixed = c_int;
pub type IOVersion = c_uint;
pub type IOItemCount = c_uint;
pub type IOCacheMode = c_uint;

pub type IOByteCount32 = c_uint;
pub type IOByteCount64 = c_ulonglong;

pub type IOPhysicalAddress32 = c_uint;
pub type IOPhysicalAddress64 = c_ulonglong;
pub type IOPhysicalLength32 = c_uint;
pub type IOPhysicalLength64 = c_ulonglong;

#[cfg(all(not(target_arch = "arm"), not(target_arch = "x86")))]
pub type IOVirtualAddress = mach_vm_address_t;
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
pub type IOVirtualAddress = vm_address_t;

#[cfg(all(
    not(target_arch = "arm"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64")
))]
pub type IOByteCount = IOByteCount64;
#[cfg(any(target_arch = "arm", target_arch = "x86", target_arch = "x86_64"))]
pub type IOByteCount = IOByteCount32;

pub type IOLogicalAddress = IOVirtualAddress;

#[cfg(all(
    not(target_arch = "arm"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64")
))]
pub type IOPhysicalAddress = IOPhysicalAddress64;
#[cfg(all(
    not(target_arch = "arm"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64")
))]
pub type IOPhysicalLength = IOPhysicalLength64;
#[cfg(all(
    not(target_arch = "arm"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64")
))]
pub const IOPhysSize: c_int = 64;

#[cfg(any(target_arch = "arm", target_arch = "x86", target_arch = "x86_64"))]
pub type IOPhysicalAddress = IOPhysicalAddress32;
#[cfg(any(target_arch = "arm", target_arch = "x86", target_arch = "x86_64"))]
pub type IOPhysicalLength = IOPhysicalAddress32;
#[cfg(any(target_arch = "arm", target_arch = "x86", target_arch = "x86_64"))]
pub const IOPhysSize: c_int = 32;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct IOPhysicalRange {
    address: IOPhysicalAddress,
    length: IOByteCount,
}
impl Clone for IOPhysicalRange {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct IOVirtualRange {
    address: IOVirtualAddress,
    length: IOByteCount,
}
impl Clone for IOVirtualRange {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(all(not(target_arch = "arm"), not(target_arch = "x86")))]
pub type IOAddressRange = IOVirtualRange;
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
#[repr(C)]
#[derive(Debug, Copy)]
pub struct IOAddressRange {
    address: mach_vm_address_t,
    length: mach_vm_size_t,
}
#[cfg(any(target_arch = "arm", target_arch = "x86"))]
impl Clone for IOAddressRange {
    fn clone(&self) -> Self {
        *self
    }
}

// Map between #defined or enum'd constants and text description.
#[repr(C)]
#[derive(Debug, Copy)]
pub struct IONamedValue {
    value: c_int,
    name: *const c_char,
}
impl Clone for IONamedValue {
    fn clone(&self) -> Self {
        *self
    }
}

// Memory alignment -- specified as a power of two.
pub type IOAlignment = c_uint;

pub type io_object_t = mach_port_t;

pub type io_connect_t = io_object_t;
pub type io_enumerator_t = io_object_t;
pub type io_iterator_t = io_object_t;
pub type io_registry_entry_t = io_object_t;
pub type io_service_t = io_object_t;

pub const IO_OBJECT_NULL: io_object_t = 0;

// IOConnectMapMemory memoryTypes
pub const kIODefaultMemoryType: c_int = 0;

pub const kIODefaultCache: c_int = 0;
pub const kIOInhibitCache: c_int = 1;
pub const kIOWriteThruCache: c_int = 2;
pub const kIOCopybackCache: c_int = 3;
pub const kIOWriteCombineCache: c_int = 4;
pub const kIOCopybackInnerCache: c_int = 5;

// IOMemory mapping options
pub const kIOMapAnywhere: c_int = 0x00000001;

pub const kIOMapCacheMask: c_int = 0x00000700;
pub const kIOMapCacheShift: c_int = 8;
pub const kIOMapDefaultCache: c_int = kIODefaultCache << kIOMapCacheShift;
pub const kIOMapInhibitCache: c_int = kIOInhibitCache << kIOMapCacheShift;
pub const kIOMapWriteThruCache: c_int = kIOWriteThruCache << kIOMapCacheShift;
pub const kIOMapCopybackCache: c_int = kIOCopybackCache << kIOMapCacheShift;
pub const kIOMapWriteCombineCache: c_int = kIOWriteCombineCache << kIOMapCacheShift;
pub const kIOMapCopybackInnerCache: c_int = kIOCopybackInnerCache << kIOMapCacheShift;

pub const kIOMapUserOptionsMask: c_int = 0x00000fff;

pub const kIOMapReadOnly: c_int = 0x00001000;

pub const kIOMapStatic: c_int = 0x01000000;
pub const kIOMapReference: c_int = 0x02000000;
pub const kIOMapUnique: c_int = 0x04000000;
pub const kIOMapPrefault: c_int = 0x10000000;
pub const kIOMapOverwrite: c_int = 0x20000000;

// Scale Factors
pub const kNanosecondScale: c_int = 1;
pub const kMicrosecondScale: c_int = 1000;
pub const kMillisecondScale: c_int = 1000 * 1000;
pub const kSecondScale: c_int = 1000 * 1000 * 1000;
pub const kTickScale: c_int = kSecondScale / 100;

pub const kIOConnectMethodVarOutputSize: c_int = -3;

// compatibility types
pub type IODeviceNumber = c_uint;
