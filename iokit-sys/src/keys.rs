// exports from <IOKit/IOKitKeys.h>

// properties found in the registry root
pub const kIOKitBuildVersionKey: *const ::libc::c_char =
    b"IOKitBuildVersion\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kIOKitDiagnosticsKey: *const ::libc::c_char =
    b"IOKitDiagnostics\x00" as *const [u8; 17usize] as *const ::libc::c_char;
// a dictionary keyed by plane name
pub const kIORegistryPlanesKey: *const ::libc::c_char =
    b"IORegistryPlanes\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOCatalogueKey: *const ::libc::c_char =
    b"IOCatalogue\x00" as *const [u8; 12usize] as *const ::libc::c_char;

// registry plane names
pub const kIOServicePlane: *const ::libc::c_char =
    b"IOService\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOPowerPlane: *const ::libc::c_char =
    b"IOPower\x00" as *const [u8; 8usize] as *const ::libc::c_char;
pub const kIODeviceTreePlane: *const ::libc::c_char =
    b"IODeviceTree\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOAudioPlane: *const ::libc::c_char =
    b"IOAudio\x00" as *const [u8; 8usize] as *const ::libc::c_char;
pub const kIOFireWirePlane: *const ::libc::c_char =
    b"IOFireWire\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kIOUSBPlane: *const ::libc::c_char =
    b"IOUSB\x00" as *const [u8; 6usize] as *const ::libc::c_char;

// registry ID number
pub const kIORegistryEntryIDKey: *const ::libc::c_char =
    b"IORegistryEntryID\x00" as *const [u8; 18usize] as *const ::libc::c_char;

// IOService class name
pub const kIOServiceClass: *const ::libc::c_char =
    b"IOService\x00" as *const [u8; 10usize] as *const ::libc::c_char;

// IOResources class name
pub const kIOResourcesClass: *const ::libc::c_char =
    b"IOResources\x00" as *const [u8; 12usize] as *const ::libc::c_char;

// IOService driver probing property names
pub const kIOClassKey: *const ::libc::c_char =
    b"IOClass\x00" as *const [u8; 8usize] as *const ::libc::c_char;
pub const kIOProbeScoreKey: *const ::libc::c_char =
    b"IOProbeScore\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOKitDebugKey: *const ::libc::c_char =
    b"IOKitDebug\x00" as *const [u8; 11usize] as *const ::libc::c_char;

// IOService matching property names
pub const kIOProviderClassKey: *const ::libc::c_char =
    b"IOProviderClass\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kIONameMatchKey: *const ::libc::c_char =
    b"IONameMatch\x00" as *const [u8; 12usize] as *const ::libc::c_char;
pub const kIOPropertyMatchKey: *const ::libc::c_char =
    b"IOPropertyMatch\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kIOPathMatchKey: *const ::libc::c_char =
    b"IOPathMatch\x00" as *const [u8; 12usize] as *const ::libc::c_char;
pub const kIOLocationMatchKey: *const ::libc::c_char =
    b"IOLocationMatch\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kIOParentMatchKey: *const ::libc::c_char =
    b"IOParentMatch\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kIOResourceMatchKey: *const ::libc::c_char =
    b"IOResourceMatch\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kIOMatchedServiceCountKey: *const ::libc::c_char =
    b"IOMatchedServiceCountMatch\x00" as *const [u8; 27usize] as *const ::libc::c_char;

pub const kIONameMatchedKey: *const ::libc::c_char =
    b"IONameMatched\x00" as *const [u8; 14usize] as *const ::libc::c_char;

pub const kIOMatchCategoryKey: *const ::libc::c_char =
    b"IOMatchCategory\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kIODefaultMatchCategoryKey: *const ::libc::c_char =
    b"IODefaultMatchCategory\x00" as *const [u8; 23usize] as *const ::libc::c_char;

// IOService default user client class, for loadable user clients
pub const kIOUserClientClassKey: *const ::libc::c_char =
    b"IOUserClientClass\x00" as *const [u8; 18usize] as *const ::libc::c_char;

// key to find IOMappers
pub const kIOMapperIDKey: *const ::libc::c_char =
    b"IOMapperID\x00" as *const [u8; 11usize] as *const ::libc::c_char;

pub const kIOUserClientCrossEndianKey: *const ::libc::c_char =
    b"IOUserClientCrossEndian\x00" as *const [u8; 24usize] as *const ::libc::c_char;
pub const kIOUserClientCrossEndianCompatibleKey: *const ::libc::c_char =
    b"IOUserClientCrossEndianCompatible\x00" as *const [u8; 34usize] as *const ::libc::c_char;
pub const kIOUserClientSharedInstanceKey: *const ::libc::c_char =
    b"IOUserClientSharedInstance\x00" as *const [u8; 27usize] as *const ::libc::c_char;
// diagnostic string describing the creating task
pub const kIOUserClientCreatorKey: *const ::libc::c_char =
    b"IOUserClientCreator\x00" as *const [u8; 20usize] as *const ::libc::c_char;

// IOService notification types
pub const kIOPublishNotification: *const ::libc::c_char =
    b"IOServicePublish\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOFirstPublishNotification: *const ::libc::c_char =
    b"IOServiceFirstPublish\x00" as *const [u8; 22usize] as *const ::libc::c_char;
pub const kIOMatchedNotification: *const ::libc::c_char =
    b"IOServiceMatched\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOFirstMatchNotification: *const ::libc::c_char =
    b"IOServiceFirstMatch\x00" as *const [u8; 20usize] as *const ::libc::c_char;
pub const kIOTerminatedNotification: *const ::libc::c_char =
    b"IOServiceTerminate\x00" as *const [u8; 19usize] as *const ::libc::c_char;

// IOService interest notification types
pub const kIOGeneralInterest: *const ::libc::c_char =
    b"IOGeneralInterest\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kIOBusyInterest: *const ::libc::c_char =
    b"IOBusyInterest\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOAppPowerStateInterest: *const ::libc::c_char =
    b"IOAppPowerStateInterest\x00" as *const [u8; 24usize] as *const ::libc::c_char;
pub const kIOPriorityPowerStateInterest: *const ::libc::c_char =
    b"IOPriorityPowerStateInterest\x00" as *const [u8; 29usize] as *const ::libc::c_char;

pub const kIOPlatformDeviceMessageKey: *const ::libc::c_char =
    b"IOPlatformDeviceMessage\x00" as *const [u8; 24usize] as *const ::libc::c_char;

// IOService interest notification types
pub const kIOCFPlugInTypesKey: *const ::libc::c_char =
    b"IOCFPlugInTypes\x00" as *const [u8; 16usize] as *const ::libc::c_char;

// properties found in services that implement command pooling
pub const kIOCommandPoolSizeKey: *const ::libc::c_char =
    b"IOCommandPoolSize\x00" as *const [u8; 18usize] as *const ::libc::c_char;

// properties found in services that implement priority
pub const kIOMaximumPriorityCountKey: *const ::libc::c_char =
    b"IOMaximumPriorityCount\x00" as *const [u8; 23usize] as *const ::libc::c_char;

// properties found in services that have transfer constraints
pub const kIOMaximumBlockCountReadKey: *const ::libc::c_char =
    b"IOMaximumBlockCountRead\x00" as *const [u8; 24usize] as *const ::libc::c_char;
pub const kIOMaximumBlockCountWriteKey: *const ::libc::c_char =
    b"IOMaximumBlockCountWrite\x00" as *const [u8; 25usize] as *const ::libc::c_char;
pub const kIOMaximumByteCountReadKey: *const ::libc::c_char =
    b"IOMaximumByteCountRead\x00" as *const [u8; 23usize] as *const ::libc::c_char;
pub const kIOMaximumByteCountWriteKey: *const ::libc::c_char =
    b"IOMaximumByteCountWrite\x00" as *const [u8; 24usize] as *const ::libc::c_char;
pub const kIOMaximumSegmentCountReadKey: *const ::libc::c_char =
    b"IOMaximumSegmentCountRead\x00" as *const [u8; 26usize] as *const ::libc::c_char;
pub const kIOMaximumSegmentCountWriteKey: *const ::libc::c_char =
    b"IOMaximumSegmentCountWrite\x00" as *const [u8; 27usize] as *const ::libc::c_char;
pub const kIOMaximumSegmentByteCountReadKey: *const ::libc::c_char =
    b"IOMaximumSegmentByteCountRead\x00" as *const [u8; 30usize] as *const ::libc::c_char;
pub const kIOMaximumSegmentByteCountWriteKey: *const ::libc::c_char =
    b"IOMaximumSegmentByteCountWrite\x00" as *const [u8; 31usize] as *const ::libc::c_char;
pub const kIOMinimumSegmentAlignmentByteCountKey: *const ::libc::c_char =
    b"IOMinimumSegmentAlignmentByteCount\x00" as *const [u8; 35usize] as *const ::libc::c_char;
pub const kIOMaximumSegmentAddressableBitCountKey: *const ::libc::c_char =
    b"IOMaximumSegmentAddressableBitCount\x00" as *const [u8; 36usize] as *const ::libc::c_char;

// properties found in services that wish to describe an icon
pub const kIOIconKey: *const ::libc::c_char =
    b"IOIcon\x00" as *const [u8; 7usize] as *const ::libc::c_char;
pub const kIOBundleResourceFileKey: *const ::libc::c_char =
    b"IOBundleResourceFile\x00" as *const [u8; 21usize] as *const ::libc::c_char;
pub const kIOBusBadgeKey: *const ::libc::c_char =
    b"IOBusBadge\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kIODeviceIconKey: *const ::libc::c_char =
    b"IODeviceIcon\x00" as *const [u8; 13usize] as *const ::libc::c_char;

// property of root that describes the machine's serial number as a string
pub const kIOPlatformSerialNumberKey: *const ::libc::c_char =
    b"IOPlatformSerialNumber\x00" as *const [u8; 23usize] as *const ::libc::c_char;

// property of root that describes the machine's UUID as a string
pub const kIOPlatformUUIDKey: *const ::libc::c_char =
    b"IOPlatformUUID\x00" as *const [u8; 15usize] as *const ::libc::c_char;

// IODTNVRAM property keys
pub const kIONVRAMDeletePropertyKey: *const ::libc::c_char =
    b"IONVRAM-DELETE-PROPERTY\x00" as *const [u8; 24usize] as *const ::libc::c_char;
pub const kIONVRAMSyncNowPropertyKey: *const ::libc::c_char =
    b"IONVRAM-SYNCNOW-PROPERTY\x00" as *const [u8; 25usize] as *const ::libc::c_char;
pub const kIONVRAMActivateCSRConfigPropertyKey: *const ::libc::c_char =
    b"IONVRAM-ARMCSR-PROPERTY\x00" as *const [u8; 24usize] as *const ::libc::c_char;
pub const kIODTNVRAMPanicInfoKey: *const ::libc::c_char =
    b"aapl,panic-info\x00" as *const [u8; 16usize] as *const ::libc::c_char;

// keys for complex boot information
pub const kIOBootDeviceKey: *const ::libc::c_char =
    b"IOBootDevice\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOBootDevicePathKey: *const ::libc::c_char =
    b"IOBootDevicePath\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOBootDeviceSizeKey: *const ::libc::c_char =
    b"IOBootDeviceSize\x00" as *const [u8; 17usize] as *const ::libc::c_char;

// keys for OS Version information
pub const kOSBuildVersionKey: *const ::libc::c_char =
    b"OS Build Version\x00" as *const [u8; 17usize] as *const ::libc::c_char;
