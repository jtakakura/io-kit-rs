// exports from <IOKit/IOKitKeys.h>

// properties found in the registry root
pub const kIOKitBuildVersionKey: &'static [u8; 18usize] = b"IOKitBuildVersion\x00";
pub const kIOKitDiagnosticsKey: &'static [u8; 17usize] = b"IOKitDiagnostics\x00";
// a dictionary keyed by plane name
pub const kIORegistryPlanesKey: &'static [u8; 17usize] = b"IORegistryPlanes\x00";
pub const kIOCatalogueKey: &'static [u8; 12usize] = b"IOCatalogue\x00";

// registry plane names
pub const kIOServicePlane: &'static [u8; 10usize] = b"IOService\x00";
pub const kIOPowerPlane: &'static [u8; 8usize] = b"IOPower\x00";
pub const kIODeviceTreePlane: &'static [u8; 13usize] = b"IODeviceTree\x00";
pub const kIOAudioPlane: &'static [u8; 8usize] = b"IOAudio\x00";
pub const kIOFireWirePlane: &'static [u8; 11usize] = b"IOFireWire\x00";
pub const kIOUSBPlane: &'static [u8; 6usize] = b"IOUSB\x00";

// registry ID number
pub const kIORegistryEntryIDKey: &'static [u8; 18usize] = b"IORegistryEntryID\x00";

// IOService class name
pub const kIOServiceClass: &'static [u8; 10usize] = b"IOService\x00";

// IOResources class name
pub const kIOResourcesClass: &'static [u8; 12usize] = b"IOResources\x00";

// IOService driver probing property names
pub const kIOClassKey: &'static [u8; 8usize] = b"IOClass\x00";
pub const kIOProbeScoreKey: &'static [u8; 13usize] = b"IOProbeScore\x00";
pub const kIOKitDebugKey: &'static [u8; 11usize] = b"IOKitDebug\x00";

// IOService matching property names
pub const kIOProviderClassKey: &'static [u8; 16usize] = b"IOProviderClass\x00";
pub const kIONameMatchKey: &'static [u8; 12usize] = b"IONameMatch\x00";
pub const kIOPropertyMatchKey: &'static [u8; 16usize] = b"IOPropertyMatch\x00";
pub const kIOPathMatchKey: &'static [u8; 12usize] = b"IOPathMatch\x00";
pub const kIOLocationMatchKey: &'static [u8; 16usize] = b"IOLocationMatch\x00";
pub const kIOParentMatchKey: &'static [u8; 14usize] = b"IOParentMatch\x00";
pub const kIOResourceMatchKey: &'static [u8; 16usize] = b"IOResourceMatch\x00";
pub const kIOMatchedServiceCountKey: &'static [u8; 27usize] = b"IOMatchedServiceCountMatch\x00";

pub const kIONameMatchedKey: &'static [u8; 14usize] = b"IONameMatched\x00";

pub const kIOMatchCategoryKey: &'static [u8; 16usize] = b"IOMatchCategory\x00";
pub const kIODefaultMatchCategoryKey: &'static [u8; 23usize] = b"IODefaultMatchCategory\x00";

// IOService default user client class, for loadable user clients
pub const kIOUserClientClassKey: &'static [u8; 18usize] = b"IOUserClientClass\x00";

// key to find IOMappers
pub const kIOMapperIDKey: &'static [u8; 11usize] = b"IOMapperID\x00";

pub const kIOUserClientCrossEndianKey: &'static [u8; 24usize] = b"IOUserClientCrossEndian\x00";
pub const kIOUserClientCrossEndianCompatibleKey: &'static [u8; 34usize] =
    b"IOUserClientCrossEndianCompatible\x00";
pub const kIOUserClientSharedInstanceKey: &'static [u8; 27usize] =
    b"IOUserClientSharedInstance\x00";
// diagnostic string describing the creating task
pub const kIOUserClientCreatorKey: &'static [u8; 20usize] = b"IOUserClientCreator\x00";

// IOService notification types
pub const kIOPublishNotification: &'static [u8; 17usize] = b"IOServicePublish\x00";
pub const kIOFirstPublishNotification: &'static [u8; 22usize] = b"IOServiceFirstPublish\x00";
pub const kIOMatchedNotification: &'static [u8; 17usize] = b"IOServiceMatched\x00";
pub const kIOFirstMatchNotification: &'static [u8; 20usize] = b"IOServiceFirstMatch\x00";
pub const kIOTerminatedNotification: &'static [u8; 19usize] = b"IOServiceTerminate\x00";

// IOService interest notification types
pub const kIOGeneralInterest: &'static [u8; 18usize] = b"IOGeneralInterest\x00";
pub const kIOBusyInterest: &'static [u8; 15usize] = b"IOBusyInterest\x00";
pub const kIOAppPowerStateInterest: &'static [u8; 24usize] = b"IOAppPowerStateInterest\x00";
pub const kIOPriorityPowerStateInterest: &'static [u8; 29usize] =
    b"IOPriorityPowerStateInterest\x00";

pub const kIOPlatformDeviceMessageKey: &'static [u8; 24usize] = b"IOPlatformDeviceMessage\x00";

// IOService interest notification types
pub const kIOCFPlugInTypesKey: &'static [u8; 16usize] = b"IOCFPlugInTypes\x00";

// properties found in services that implement command pooling
pub const kIOCommandPoolSizeKey: &'static [u8; 18usize] = b"IOCommandPoolSize\x00";

// properties found in services that implement priority
pub const kIOMaximumPriorityCountKey: &'static [u8; 23usize] = b"IOMaximumPriorityCount\x00";

// properties found in services that have transfer constraints
pub const kIOMaximumBlockCountReadKey: &'static [u8; 24usize] = b"IOMaximumBlockCountRead\x00";
pub const kIOMaximumBlockCountWriteKey: &'static [u8; 25usize] = b"IOMaximumBlockCountWrite\x00";
pub const kIOMaximumByteCountReadKey: &'static [u8; 23usize] = b"IOMaximumByteCountRead\x00";
pub const kIOMaximumByteCountWriteKey: &'static [u8; 24usize] = b"IOMaximumByteCountWrite\x00";
pub const kIOMaximumSegmentCountReadKey: &'static [u8; 26usize] = b"IOMaximumSegmentCountRead\x00";
pub const kIOMaximumSegmentCountWriteKey: &'static [u8; 27usize] =
    b"IOMaximumSegmentCountWrite\x00";
pub const kIOMaximumSegmentByteCountReadKey: &'static [u8; 30usize] =
    b"IOMaximumSegmentByteCountRead\x00";
pub const kIOMaximumSegmentByteCountWriteKey: &'static [u8; 31usize] =
    b"IOMaximumSegmentByteCountWrite\x00";
pub const kIOMinimumSegmentAlignmentByteCountKey: &'static [u8; 35usize] =
    b"IOMinimumSegmentAlignmentByteCount\x00";
pub const kIOMaximumSegmentAddressableBitCountKey: &'static [u8; 36usize] =
    b"IOMaximumSegmentAddressableBitCount\x00";

// properties found in services that wish to describe an icon
pub const kIOIconKey: &'static [u8; 7usize] = b"IOIcon\x00";
pub const kIOBundleResourceFileKey: &'static [u8; 21usize] = b"IOBundleResourceFile\x00";
pub const kIOBusBadgeKey: &'static [u8; 11usize] = b"IOBusBadge\x00";
pub const kIODeviceIconKey: &'static [u8; 13usize] = b"IODeviceIcon\x00";

// property of root that describes the machine's serial number as a string
pub const kIOPlatformSerialNumberKey: &'static [u8; 23usize] = b"IOPlatformSerialNumber\x00";

// property of root that describes the machine's UUID as a string
pub const kIOPlatformUUIDKey: &'static [u8; 15usize] = b"IOPlatformUUID\x00";

// IODTNVRAM property keys
pub const kIONVRAMDeletePropertyKey: &'static [u8; 24usize] = b"IONVRAM-DELETE-PROPERTY\x00";
pub const kIONVRAMSyncNowPropertyKey: &'static [u8; 25usize] = b"IONVRAM-SYNCNOW-PROPERTY\x00";
pub const kIONVRAMActivateCSRConfigPropertyKey: &'static [u8; 24usize] =
    b"IONVRAM-ARMCSR-PROPERTY\x00";
pub const kIODTNVRAMPanicInfoKey: &'static [u8; 16usize] = b"aapl,panic-info\x00";

// keys for complex boot information
pub const kIOBootDeviceKey: &'static [u8; 13usize] = b"IOBootDevice\x00";
pub const kIOBootDevicePathKey: &'static [u8; 17usize] = b"IOBootDevicePath\x00";
pub const kIOBootDeviceSizeKey: &'static [u8; 17usize] = b"IOBootDeviceSize\x00";

// keys for OS Version information
pub const kOSBuildVersionKey: &'static [u8; 17usize] = b"OS Build Version\x00";
