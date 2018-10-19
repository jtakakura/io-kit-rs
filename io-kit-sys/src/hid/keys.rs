// exports from <IOKit/hid/IOHIDKeys.h>

// This is used to find HID Devices in the IORegistry
pub const kIOHIDDeviceKey: *const ::libc::c_char =
    b"IOHIDDevice\x00" as *const [u8; 12usize] as *const ::libc::c_char;

// HID Device Property Keys
pub const kIOHIDTransportKey: *const ::libc::c_char =
    b"Transport\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOHIDVendorIDKey: *const ::libc::c_char =
    b"VendorID\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kIOHIDVendorIDSourceKey: *const ::libc::c_char =
    b"VendorIDSource\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDProductIDKey: *const ::libc::c_char =
    b"ProductID\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOHIDVersionNumberKey: *const ::libc::c_char =
    b"VersionNumber\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kIOHIDManufacturerKey: *const ::libc::c_char =
    b"Manufacturer\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOHIDProductKey: *const ::libc::c_char =
    b"Product\x00" as *const [u8; 8usize] as *const ::libc::c_char;
pub const kIOHIDSerialNumberKey: *const ::libc::c_char =
    b"SerialNumber\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOHIDCountryCodeKey: *const ::libc::c_char =
    b"CountryCode\x00" as *const [u8; 12usize] as *const ::libc::c_char;
pub const kIOHIDStandardTypeKey: *const ::libc::c_char =
    b"StandardType\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOHIDLocationIDKey: *const ::libc::c_char =
    b"LocationID\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kIOHIDDeviceUsageKey: *const ::libc::c_char =
    b"DeviceUsage\x00" as *const [u8; 12usize] as *const ::libc::c_char;
pub const kIOHIDDeviceUsagePageKey: *const ::libc::c_char =
    b"DeviceUsagePage\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kIOHIDDeviceUsagePairsKey: *const ::libc::c_char =
    b"DeviceUsagePairs\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDPrimaryUsageKey: *const ::libc::c_char =
    b"PrimaryUsage\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOHIDPrimaryUsagePageKey: *const ::libc::c_char =
    b"PrimaryUsagePage\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDMaxInputReportSizeKey: *const ::libc::c_char =
    b"MaxInputReportSize\x00" as *const [u8; 19usize] as *const ::libc::c_char;
pub const kIOHIDMaxOutputReportSizeKey: *const ::libc::c_char =
    b"MaxOutputReportSize\x00" as *const [u8; 20usize] as *const ::libc::c_char;
pub const kIOHIDMaxFeatureReportSizeKey: *const ::libc::c_char =
    b"MaxFeatureReportSize\x00" as *const [u8; 21usize] as *const ::libc::c_char;
pub const kIOHIDReportIntervalKey: *const ::libc::c_char =
    b"ReportInterval\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDSampleIntervalKey: *const ::libc::c_char =
    b"SampleInterval\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDBatchIntervalKey: *const ::libc::c_char =
    b"BatchInterval\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kIOHIDRequestTimeoutKey: *const ::libc::c_char =
    b"RequestTimeout\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDReportDescriptorKey: *const ::libc::c_char =
    b"ReportDescriptor\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDResetKey: *const ::libc::c_char =
    b"Reset\x00" as *const [u8; 6usize] as *const ::libc::c_char;
pub const kIOHIDKeyboardLanguageKey: *const ::libc::c_char =
    b"KeyboardLanguage\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDAltHandlerIdKey: *const ::libc::c_char =
    b"alt_handler_id\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDBuiltInKey: *const ::libc::c_char =
    b"Built-In\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kIOHIDDisplayIntegratedKey: *const ::libc::c_char =
    b"DisplayIntegrated\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kIOHIDProductIDMaskKey: *const ::libc::c_char =
    b"ProductIDMask\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kIOHIDProductIDArrayKey: *const ::libc::c_char =
    b"ProductIDArray\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDPowerOnDelayNSKey: *const ::libc::c_char =
    b"HIDPowerOnDelayNS\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kIOHIDCategoryKey: *const ::libc::c_char =
    b"Category\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kIOHIDMaxResponseLatencyKey: *const ::libc::c_char =
    b"MaxResponseLatency\x00" as *const [u8; 19usize] as *const ::libc::c_char;
pub const kIOHIDUniqueIDKey: *const ::libc::c_char =
    b"UniqueID\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kIOHIDPhysicalDeviceUniqueIDKey: *const ::libc::c_char =
    b"PhysicalDeviceUniqueID\x00" as *const [u8; 23usize] as *const ::libc::c_char;

pub const kIOHIDTransportUSBValue: *const ::libc::c_char =
    b"USB\x00" as *const [u8; 4usize] as *const ::libc::c_char;
pub const kIOHIDTransportBluetoothValue: *const ::libc::c_char =
    b"Bluetooth\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOHIDTransportBluetoothLowEnergyValue: *const ::libc::c_char =
    b"BluetoothLowEnergy\x00" as *const [u8; 19usize] as *const ::libc::c_char;
pub const kIOHIDTransportAIDBValue: *const ::libc::c_char =
    b"AIDB\x00" as *const [u8; 5usize] as *const ::libc::c_char;
pub const kIOHIDTransportI2CValue: *const ::libc::c_char =
    b"I2C\x00" as *const [u8; 4usize] as *const ::libc::c_char;
pub const kIOHIDTransportSPIValue: *const ::libc::c_char =
    b"SPI\x00" as *const [u8; 4usize] as *const ::libc::c_char;
pub const kIOHIDTransportSerialValue: *const ::libc::c_char =
    b"Serial\x00" as *const [u8; 7usize] as *const ::libc::c_char;
pub const kIOHIDTransportIAPValue: *const ::libc::c_char =
    b"IAP\x00" as *const [u8; 4usize] as *const ::libc::c_char;
pub const kIOHIDTransportAirPlayValue: *const ::libc::c_char =
    b"AirPlay\x00" as *const [u8; 8usize] as *const ::libc::c_char;
pub const kIOHIDTransportSPUValue: *const ::libc::c_char =
    b"SPU\x00" as *const [u8; 4usize] as *const ::libc::c_char;

pub const kIOHIDCategoryAutomotiveValue: *const ::libc::c_char =
    b"Automotive\x00" as *const [u8; 11usize] as *const ::libc::c_char;

// HID Element Key
pub const kIOHIDElementKey: *const ::libc::c_char =
    b"Elements\x00" as *const [u8; 9usize] as *const ::libc::c_char;

// HID Element Dictionary Keys
pub const kIOHIDElementCookieKey: *const ::libc::c_char =
    b"ElementCookie\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kIOHIDElementTypeKey: *const ::libc::c_char =
    b"Type\x00" as *const [u8; 5usize] as *const ::libc::c_char;
pub const kIOHIDElementCollectionTypeKey: *const ::libc::c_char =
    b"CollectionType\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDElementUsageKey: *const ::libc::c_char =
    b"Usage\x00" as *const [u8; 6usize] as *const ::libc::c_char;
pub const kIOHIDElementUsagePageKey: *const ::libc::c_char =
    b"UsagePage\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOHIDElementMinKey: *const ::libc::c_char =
    b"Min\x00" as *const [u8; 4usize] as *const ::libc::c_char;
pub const kIOHIDElementMaxKey: *const ::libc::c_char =
    b"Max\x00" as *const [u8; 4usize] as *const ::libc::c_char;
pub const kIOHIDElementScaledMinKey: *const ::libc::c_char =
    b"ScaledMin\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOHIDElementScaledMaxKey: *const ::libc::c_char =
    b"ScaledMax\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kIOHIDElementSizeKey: *const ::libc::c_char =
    b"Size\x00" as *const [u8; 5usize] as *const ::libc::c_char;
pub const kIOHIDElementReportSizeKey: *const ::libc::c_char =
    b"ReportSize\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kIOHIDElementReportCountKey: *const ::libc::c_char =
    b"ReportCount\x00" as *const [u8; 12usize] as *const ::libc::c_char;
pub const kIOHIDElementReportIDKey: *const ::libc::c_char =
    b"ReportID\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kIOHIDElementIsArrayKey: *const ::libc::c_char =
    b"IsArray\x00" as *const [u8; 8usize] as *const ::libc::c_char;
pub const kIOHIDElementIsRelativeKey: *const ::libc::c_char =
    b"IsRelative\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kIOHIDElementIsWrappingKey: *const ::libc::c_char =
    b"IsWrapping\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kIOHIDElementIsNonLinearKey: *const ::libc::c_char =
    b"IsNonLinear\x00" as *const [u8; 12usize] as *const ::libc::c_char;
pub const kIOHIDElementHasPreferredStateKey: *const ::libc::c_char =
    b"HasPreferredState\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kIOHIDElementHasNullStateKey: *const ::libc::c_char =
    b"HasNullState\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOHIDElementFlagsKey: *const ::libc::c_char =
    b"Flags\x00" as *const [u8; 6usize] as *const ::libc::c_char;
pub const kIOHIDElementUnitKey: *const ::libc::c_char =
    b"Unit\x00" as *const [u8; 5usize] as *const ::libc::c_char;
pub const kIOHIDElementUnitExponentKey: *const ::libc::c_char =
    b"UnitExponent\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kIOHIDElementNameKey: *const ::libc::c_char =
    b"Name\x00" as *const [u8; 5usize] as *const ::libc::c_char;
pub const kIOHIDElementValueLocationKey: *const ::libc::c_char =
    b"ValueLocation\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kIOHIDElementDuplicateIndexKey: *const ::libc::c_char =
    b"DuplicateIndex\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDElementParentCollectionKey: *const ::libc::c_char =
    b"ParentCollection\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDElementVendorSpecificKey: *const ::libc::c_char =
    b"VendorSpecific\x00" as *const [u8; 15usize] as *const ::libc::c_char;

// HID Element Match Keys
pub const kIOHIDElementCookieMinKey: *const ::libc::c_char =
    b"ElementCookieMin\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDElementCookieMaxKey: *const ::libc::c_char =
    b"ElementCookieMax\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kIOHIDElementUsageMinKey: *const ::libc::c_char =
    b"UsageMin\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kIOHIDElementUsageMaxKey: *const ::libc::c_char =
    b"UsageMax\x00" as *const [u8; 9usize] as *const ::libc::c_char;

// HID Element Calibration Keys
pub const kIOHIDElementCalibrationMinKey: *const ::libc::c_char =
    b"CalibrationMin\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDElementCalibrationMaxKey: *const ::libc::c_char =
    b"CalibrationMax\x00" as *const [u8; 15usize] as *const ::libc::c_char;
pub const kIOHIDElementCalibrationSaturationMinKey: *const ::libc::c_char =
    b"CalibrationSaturationMin\x00" as *const [u8; 25usize] as *const ::libc::c_char;
pub const kIOHIDElementCalibrationSaturationMaxKey: *const ::libc::c_char =
    b"CalibrationSaturationMax\x00" as *const [u8; 25usize] as *const ::libc::c_char;
pub const kIOHIDElementCalibrationDeadZoneMinKey: *const ::libc::c_char =
    b"CalibrationDeadZoneMin\x00" as *const [u8; 23usize] as *const ::libc::c_char;
pub const kIOHIDElementCalibrationDeadZoneMaxKey: *const ::libc::c_char =
    b"CalibrationDeadZoneMax\x00" as *const [u8; 23usize] as *const ::libc::c_char;
pub const kIOHIDElementCalibrationGranularityKey: *const ::libc::c_char =
    b"CalibrationGranularity\x00" as *const [u8; 23usize] as *const ::libc::c_char;

pub type IOHIDElementCookie = u32;

pub type IOHIDElementType = u32;
pub const kIOHIDElementTypeInput_Misc: IOHIDElementType = 1;
pub const kIOHIDElementTypeInput_Button: IOHIDElementType = 2;
pub const kIOHIDElementTypeInput_Axis: IOHIDElementType = 3;
pub const kIOHIDElementTypeInput_ScanCodes: IOHIDElementType = 4;
pub const kIOHIDElementTypeOutput: IOHIDElementType = 129;
pub const kIOHIDElementTypeFeature: IOHIDElementType = 257;
pub const kIOHIDElementTypeCollection: IOHIDElementType = 513;

pub type IOHIDElementCollectionType = u32;
pub const kIOHIDElementCollectionTypePhysical: IOHIDElementCollectionType = 0x00;
pub const kIOHIDElementCollectionTypeApplication: IOHIDElementCollectionType = 0x01;
pub const kIOHIDElementCollectionTypeLogical: IOHIDElementCollectionType = 0x02;
pub const kIOHIDElementCollectionTypeReport: IOHIDElementCollectionType = 0x03;
pub const kIOHIDElementCollectionTypeNamedArray: IOHIDElementCollectionType = 0x04;
pub const kIOHIDElementCollectionTypeUsageSwitch: IOHIDElementCollectionType = 0x05;
pub const kIOHIDElementCollectionTypeUsageModifier: IOHIDElementCollectionType = 0x06;

pub type IOHIDReportType = u32;
pub const kIOHIDReportTypeInput: IOHIDReportType = 0;
pub const kIOHIDReportTypeOutput: IOHIDReportType = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount: IOHIDReportType = 3;

pub type IOHIDOptionsType = u32;
pub const kIOHIDOptionsTypeNone: IOHIDOptionsType = 0x00;
pub const kIOHIDOptionsTypeSeizeDevice: IOHIDOptionsType = 0x01;

pub type IOHIDQueueOptionsType = u32;
pub const kIOHIDQueueOptionsTypeNone: IOHIDQueueOptionsType = 0x00;
pub const kIOHIDQueueOptionsTypeEnqueueAll: IOHIDQueueOptionsType = 0x01;

pub type IOHIDElementFlags = u32;
pub const kIOHIDElementFlagsConstantMask: IOHIDElementFlags = 0x0001;
pub const kIOHIDElementFlagsVariableMask: IOHIDElementFlags = 0x0002;
pub const kIOHIDElementFlagsRelativeMask: IOHIDElementFlags = 0x0004;
pub const kIOHIDElementFlagsWrapMask: IOHIDElementFlags = 0x0008;
pub const kIOHIDElementFlagsNonLinearMask: IOHIDElementFlags = 0x0010;
pub const kIOHIDElementFlagsNoPreferredMask: IOHIDElementFlags = 0x0020;
pub const kIOHIDElementFlagsNullStateMask: IOHIDElementFlags = 0x0040;
pub const kIOHIDElementFlagsVolativeMask: IOHIDElementFlags = 0x0080;
pub const kIOHIDElementFlagsBufferedByteMask: IOHIDElementFlags = 0x0100;

pub type IOHIDStandardType = u32;
pub const kIOHIDStandardTypeANSI: IOHIDStandardType = 0;
pub const kIOHIDStandardTypeISO: IOHIDStandardType = 1;
pub const kIOHIDStandardTypeJIS: IOHIDStandardType = 2;

pub type IOHIDValueScaleType = u32;
pub const kIOHIDValueScaleTypeCalibrated: IOHIDValueScaleType = 0;
pub const kIOHIDValueScaleTypePhysical: IOHIDValueScaleType = 1;

pub type IOHIDValueOptions = u32;
pub const kIOHIDValueOptionsFlagRelativeSimple: IOHIDValueOptions = (1 << 0);
pub const kIOHIDValueOptionsFlagPrevious: IOHIDValueOptions = (1 << 1);

pub const kIOHIDDigitizerGestureCharacterStateKey: *const ::libc::c_char =
    b"DigitizerCharacterGestureState\x00" as *const [u8; 31usize] as *const ::libc::c_char;
