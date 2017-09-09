// exports from <IOKit/hid/IOHIDKeys.h>

// This is used to find HID Devices in the IORegistry
pub const kIOHIDDeviceKey: &'static [u8; 12usize] = b"IOHIDDevice\x00";

// HID Device Property Keys
pub const kIOHIDTransportKey: &'static [u8; 10usize] = b"Transport\x00";
pub const kIOHIDVendorIDKey: &'static [u8; 9usize] = b"VendorID\x00";
pub const kIOHIDVendorIDSourceKey: &'static [u8; 15usize] = b"VendorIDSource\x00";
pub const kIOHIDProductIDKey: &'static [u8; 10usize] = b"ProductID\x00";
pub const kIOHIDVersionNumberKey: &'static [u8; 14usize] = b"VersionNumber\x00";
pub const kIOHIDManufacturerKey: &'static [u8; 13usize] = b"Manufacturer\x00";
pub const kIOHIDProductKey: &'static [u8; 8usize] = b"Product\x00";
pub const kIOHIDSerialNumberKey: &'static [u8; 13usize] = b"SerialNumber\x00";
pub const kIOHIDCountryCodeKey: &'static [u8; 12usize] = b"CountryCode\x00";
pub const kIOHIDStandardTypeKey: &'static [u8; 13usize] = b"StandardType\x00";
pub const kIOHIDLocationIDKey: &'static [u8; 11usize] = b"LocationID\x00";
pub const kIOHIDDeviceUsageKey: &'static [u8; 12usize] = b"DeviceUsage\x00";
pub const kIOHIDDeviceUsagePageKey: &'static [u8; 16usize] = b"DeviceUsagePage\x00";
pub const kIOHIDDeviceUsagePairsKey: &'static [u8; 17usize] = b"DeviceUsagePairs\x00";
pub const kIOHIDPrimaryUsageKey: &'static [u8; 13usize] = b"PrimaryUsage\x00";
pub const kIOHIDPrimaryUsagePageKey: &'static [u8; 17usize] = b"PrimaryUsagePage\x00";
pub const kIOHIDMaxInputReportSizeKey: &'static [u8; 19usize] = b"MaxInputReportSize\x00";
pub const kIOHIDMaxOutputReportSizeKey: &'static [u8; 20usize] = b"MaxOutputReportSize\x00";
pub const kIOHIDMaxFeatureReportSizeKey: &'static [u8; 21usize] = b"MaxFeatureReportSize\x00";
pub const kIOHIDReportIntervalKey: &'static [u8; 15usize] = b"ReportInterval\x00";
pub const kIOHIDSampleIntervalKey: &'static [u8; 15usize] = b"SampleInterval\x00";
pub const kIOHIDBatchIntervalKey: &'static [u8; 14usize] = b"BatchInterval\x00";
pub const kIOHIDRequestTimeoutKey: &'static [u8; 15usize] = b"RequestTimeout\x00";
pub const kIOHIDReportDescriptorKey: &'static [u8; 17usize] = b"ReportDescriptor\x00";
pub const kIOHIDResetKey: &'static [u8; 6usize] = b"Reset\x00";
pub const kIOHIDKeyboardLanguageKey: &'static [u8; 17usize] = b"KeyboardLanguage\x00";
pub const kIOHIDAltHandlerIdKey: &'static [u8; 15usize] = b"alt_handler_id\x00";
pub const kIOHIDBuiltInKey: &'static [u8; 9usize] = b"Built-In\x00";
pub const kIOHIDDisplayIntegratedKey: &'static [u8; 18usize] = b"DisplayIntegrated\x00";
pub const kIOHIDProductIDMaskKey: &'static [u8; 14usize] = b"ProductIDMask\x00";
pub const kIOHIDProductIDArrayKey: &'static [u8; 15usize] = b"ProductIDArray\x00";
pub const kIOHIDPowerOnDelayNSKey: &'static [u8; 18usize] = b"HIDPowerOnDelayNS\x00";
pub const kIOHIDCategoryKey: &'static [u8; 9usize] = b"Category\x00";
pub const kIOHIDMaxResponseLatencyKey: &'static [u8; 19usize] = b"MaxResponseLatency\x00";
pub const kIOHIDUniqueIDKey: &'static [u8; 9usize] = b"UniqueID\x00";
pub const kIOHIDPhysicalDeviceUniqueIDKey: &'static [u8; 23usize] = b"PhysicalDeviceUniqueID\x00";

pub const kIOHIDTransportUSBValue: &'static [u8; 4usize] = b"USB\x00";
pub const kIOHIDTransportBluetoothValue: &'static [u8; 10usize] = b"Bluetooth\x00";
pub const kIOHIDTransportBluetoothLowEnergyValue: &'static [u8; 19usize] =
    b"BluetoothLowEnergy\x00";
pub const kIOHIDTransportAIDBValue: &'static [u8; 5usize] = b"AIDB\x00";
pub const kIOHIDTransportI2CValue: &'static [u8; 4usize] = b"I2C\x00";
pub const kIOHIDTransportSPIValue: &'static [u8; 4usize] = b"SPI\x00";
pub const kIOHIDTransportSerialValue: &'static [u8; 7usize] = b"Serial\x00";
pub const kIOHIDTransportIAPValue: &'static [u8; 4usize] = b"IAP\x00";
pub const kIOHIDTransportAirPlayValue: &'static [u8; 8usize] = b"AirPlay\x00";
pub const kIOHIDTransportSPUValue: &'static [u8; 4usize] = b"SPU\x00";

pub const kIOHIDCategoryAutomotiveValue: &'static [u8; 11usize] = b"Automotive\x00";

// HID Element Key
pub const kIOHIDElementKey: &'static [u8; 9usize] = b"Elements\x00";

// HID Element Dictionary Keys
pub const kIOHIDElementCookieKey: &'static [u8; 14usize] = b"ElementCookie\x00";
pub const kIOHIDElementTypeKey: &'static [u8; 5usize] = b"Type\x00";
pub const kIOHIDElementCollectionTypeKey: &'static [u8; 15usize] = b"CollectionType\x00";
pub const kIOHIDElementUsageKey: &'static [u8; 6usize] = b"Usage\x00";
pub const kIOHIDElementUsagePageKey: &'static [u8; 10usize] = b"UsagePage\x00";
pub const kIOHIDElementMinKey: &'static [u8; 4usize] = b"Min\x00";
pub const kIOHIDElementMaxKey: &'static [u8; 4usize] = b"Max\x00";
pub const kIOHIDElementScaledMinKey: &'static [u8; 10usize] = b"ScaledMin\x00";
pub const kIOHIDElementScaledMaxKey: &'static [u8; 10usize] = b"ScaledMax\x00";
pub const kIOHIDElementSizeKey: &'static [u8; 5usize] = b"Size\x00";
pub const kIOHIDElementReportSizeKey: &'static [u8; 11usize] = b"ReportSize\x00";
pub const kIOHIDElementReportCountKey: &'static [u8; 12usize] = b"ReportCount\x00";
pub const kIOHIDElementReportIDKey: &'static [u8; 9usize] = b"ReportID\x00";
pub const kIOHIDElementIsArrayKey: &'static [u8; 8usize] = b"IsArray\x00";
pub const kIOHIDElementIsRelativeKey: &'static [u8; 11usize] = b"IsRelative\x00";
pub const kIOHIDElementIsWrappingKey: &'static [u8; 11usize] = b"IsWrapping\x00";
pub const kIOHIDElementIsNonLinearKey: &'static [u8; 12usize] = b"IsNonLinear\x00";
pub const kIOHIDElementHasPreferredStateKey: &'static [u8; 18usize] = b"HasPreferredState\x00";
pub const kIOHIDElementHasNullStateKey: &'static [u8; 13usize] = b"HasNullState\x00";
pub const kIOHIDElementFlagsKey: &'static [u8; 6usize] = b"Flags\x00";
pub const kIOHIDElementUnitKey: &'static [u8; 5usize] = b"Unit\x00";
pub const kIOHIDElementUnitExponentKey: &'static [u8; 13usize] = b"UnitExponent\x00";
pub const kIOHIDElementNameKey: &'static [u8; 5usize] = b"Name\x00";
pub const kIOHIDElementValueLocationKey: &'static [u8; 14usize] = b"ValueLocation\x00";
pub const kIOHIDElementDuplicateIndexKey: &'static [u8; 15usize] = b"DuplicateIndex\x00";
pub const kIOHIDElementParentCollectionKey: &'static [u8; 17usize] = b"ParentCollection\x00";
pub const kIOHIDElementVendorSpecificKey: &'static [u8; 15usize] = b"VendorSpecific\x00";

// HID Element Match Keys
pub const kIOHIDElementCookieMinKey: &'static [u8; 17usize] = b"ElementCookieMin\x00";
pub const kIOHIDElementCookieMaxKey: &'static [u8; 17usize] = b"ElementCookieMax\x00";
pub const kIOHIDElementUsageMinKey: &'static [u8; 9usize] = b"UsageMin\x00";
pub const kIOHIDElementUsageMaxKey: &'static [u8; 9usize] = b"UsageMax\x00";

// HID Element Calibration Keys
pub const kIOHIDElementCalibrationMinKey: &'static [u8; 15usize] = b"CalibrationMin\x00";
pub const kIOHIDElementCalibrationMaxKey: &'static [u8; 15usize] = b"CalibrationMax\x00";
pub const kIOHIDElementCalibrationSaturationMinKey: &'static [u8; 25usize] =
    b"CalibrationSaturationMin\x00";
pub const kIOHIDElementCalibrationSaturationMaxKey: &'static [u8; 25usize] =
    b"CalibrationSaturationMax\x00";
pub const kIOHIDElementCalibrationDeadZoneMinKey: &'static [u8; 23usize] =
    b"CalibrationDeadZoneMin\x00";
pub const kIOHIDElementCalibrationDeadZoneMaxKey: &'static [u8; 23usize] =
    b"CalibrationDeadZoneMax\x00";
pub const kIOHIDElementCalibrationGranularityKey: &'static [u8; 23usize] =
    b"CalibrationGranularity\x00";

pub type IOHIDElementCookie = u32;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IOHIDElementType {
    kIOHIDElementTypeInput_Misc = 1,
    kIOHIDElementTypeInput_Button = 2,
    kIOHIDElementTypeInput_Axis = 3,
    kIOHIDElementTypeInput_ScanCodes = 4,
    kIOHIDElementTypeOutput = 129,
    kIOHIDElementTypeFeature = 257,
    kIOHIDElementTypeCollection = 513,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IOHIDElementCollectionType {
    kIOHIDElementCollectionTypePhysical = 0x00,
    kIOHIDElementCollectionTypeApplication = 0x01,
    kIOHIDElementCollectionTypeLogical = 0x02,
    kIOHIDElementCollectionTypeReport = 0x03,
    kIOHIDElementCollectionTypeNamedArray = 0x04,
    kIOHIDElementCollectionTypeUsageSwitch = 0x05,
    kIOHIDElementCollectionTypeUsageModifier = 0x06,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IOHIDReportType {
    kIOHIDReportTypeInput = 0,
    kIOHIDReportTypeOutput = 1,
    kIOHIDReportTypeFeature = 2,
    kIOHIDReportTypeCount = 3,
}

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

pub const kIOHIDDigitizerGestureCharacterStateKey: &'static [u8; 31usize] =
    b"DigitizerCharacterGestureState\x00";
