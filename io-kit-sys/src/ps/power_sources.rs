// exports from <IOKit/ps/IOPowerSources.h>

use core_foundation_sys::array::CFArrayRef;
use core_foundation_sys::base::CFTypeRef;
use core_foundation_sys::dictionary::CFDictionaryRef;
use core_foundation_sys::runloop::CFRunLoopSourceRef;
use core_foundation_sys::string::CFStringRef;

// Low Power Warnings
pub const kIOPSNotifyLowBattery: *const ::std::os::raw::c_char =
        b"com.apple.system.powersources.lowbattery\0" as *const [u8; 41usize]
                as *const ::std::os::raw::c_char;

pub type IOPSLowBatteryWarningLevel = u32;
pub const kIOPSLowBatteryWarningNone: IOPSLowBatteryWarningLevel = 1;
pub const kIOPSLowBatteryWarningEarly: IOPSLowBatteryWarningLevel = 2;
pub const kIOPSLowBatteryWarningFinal: IOPSLowBatteryWarningLevel = 3;

pub const kIOPSNotifyTimeRemaining: *const ::std::os::raw::c_char =
        b"com.apple.system.powersources.timeremaining\0" as *const [u8; 44usize]
                as *const ::std::os::raw::c_char;
pub const kIOPSTimeRemainingNotificationKey: *const ::std::os::raw::c_char =
        kIOPSNotifyTimeRemaining as *const [u8; 44usize] as *const ::std::os::raw::c_char;
pub const kIOPSNotifyPowerSource: *const ::std::os::raw::c_char =
        b"com.apple.system.powersources.source\0" as *const [u8; 37usize]
                as *const ::std::os::raw::c_char;
pub const kIOPSNotifyAttach: *const ::std::os::raw::c_char =
        b"com.apple.system.powersources.attach\0" as *const [u8; 37usize]
                as *const ::std::os::raw::c_char;
pub const kIOPSNotifyAnyPowerSource: *const ::std::os::raw::c_char =
        b"com.apple.system.powersources\0" as *const [u8; 30usize] as *const ::std::os::raw::c_char;
pub const kIOPMUPSPowerKey: *const ::std::os::raw::c_char =
        b"UPS Power\0" as *const [u8; 10usize] as *const ::std::os::raw::c_char;
pub const kIOPMBatteryPowerKey: *const ::std::os::raw::c_char =
        b"Battery Power\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPMACPowerKey: *const ::std::os::raw::c_char =
        b"AC Power\0" as *const [u8; 9usize] as *const ::std::os::raw::c_char;

pub type IOPowerSourceCallbackType = unsafe extern "C" fn(context: *mut ::std::os::raw::c_void);

extern "C" {
        pub fn IOPSGetBatteryWarningLevel() -> IOPSLowBatteryWarningLevel;
        pub fn IOPSCopyPowerSourcesInfo() -> CFTypeRef;
        pub fn IOPSCopyPowerSourcesList(blob: CFTypeRef) -> CFArrayRef;
        pub fn IOPSGetPowerSourceDescription(blob: CFTypeRef, ps: CFTypeRef) -> CFDictionaryRef;
        pub fn IOPSGetProvidingPowerSourceType(snapshot: CFTypeRef) -> CFStringRef;
        pub fn IOPSNotificationCreateRunLoopSource(
                callback: IOPowerSourceCallbackType,
                context: *mut ::std::os::raw::c_void,
        ) -> CFRunLoopSourceRef;
        pub fn IOPSCreateLimitedPowerNotification(
                callback: IOPowerSourceCallbackType,
                context: *mut ::std::os::raw::c_void,
        ) -> CFRunLoopSourceRef;
        pub fn IOPSCopyExternalPowerAdapterDetails() -> CFDictionaryRef;
}
