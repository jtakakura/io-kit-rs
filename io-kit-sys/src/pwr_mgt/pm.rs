// exports from <IOKit/pwr_mgt/IOPM.h>

// IOPMMaxPowerStates
pub const kIOPMMaxPowerStates: u32 = 10;
pub const IOPMMaxPowerStates: u32 = kIOPMMaxPowerStates;

// IOPMPowerFlags
pub type IOPMPowerFlags = ::std::os::raw::c_ulong;
pub const kIOPMPowerOn: u32 = 0x00000002;
pub const kIOPMDeviceUsable: u32 = 0x00008000;
pub const kIOPMLowPower: u32 = 0x00010000;
pub const kIOPMPreventIdleSleep: u32 = 0x00000040;
pub const kIOPMSleepCapability: u32 = 0x00000004;
pub const kIOPMRestartCapability: u32 = 0x00000080;
pub const kIOPMSleep: u32 = 0x00000001;
pub const kIOPMRestart: u32 = 0x00000080;
pub const kIOPMInitialDeviceState: u32 = 0x00000100;
pub const kIOPMRootDomainState: u32 = 0x00000200;

// Private IOPMPowerFlags
pub const kIOPMClockNormal: u32 = 0x0004;
pub const kIOPMClockRunning: u32 = 0x0008;
pub const kIOPMPreventSystemSleep: u32 = 0x0010;
pub const kIOPMDoze: u32 = 0x0400;
pub const kIOPMChildClamp: u32 = 0x0080;
pub const kIOPMChildClamp2: u32 = 0x0200;
pub const kIOPMNotPowerManaged: u32 = 0x0800;

// Deprecated IOPMPowerFlags
pub const kIOPMMaxPerformance: u32 = 0x4000;
pub const kIOPMPassThrough: u32 = 0x0100;
pub const kIOPMAuxPowerOn: u32 = 0x0020;
pub const kIOPMNotAttainable: u32 = 0x0001;
pub const kIOPMContextRetained: u32 = 0x2000;
pub const kIOPMConfigRetained: u32 = 0x1000;
pub const kIOPMStaticPowerValid: u32 = 0x0800;
pub const kIOPMSoftSleep: u32 = 0x0400;
pub const kIOPMCapabilitiesMask: u32 = kIOPMPowerOn
    | kIOPMDeviceUsable
    | kIOPMMaxPerformance
    | kIOPMContextRetained
    | kIOPMConfigRetained
    | kIOPMSleepCapability
    | kIOPMRestartCapability;

// Support for old names of IOPMPowerFlag constants
pub const IOPMNotAttainable: u32 = kIOPMNotAttainable;
pub const IOPMPowerOn: u32 = kIOPMPowerOn;
pub const IOPMClockNormal: u32 = kIOPMClockNormal;
pub const IOPMClockRunning: u32 = kIOPMClockRunning;
pub const IOPMAuxPowerOn: u32 = kIOPMAuxPowerOn;
pub const IOPMDeviceUsable: u32 = kIOPMDeviceUsable;
pub const IOPMMaxPerformance: u32 = kIOPMMaxPerformance;
pub const IOPMContextRetained: u32 = kIOPMContextRetained;
pub const IOPMConfigRetained: u32 = kIOPMConfigRetained;
pub const IOPMNotPowerManaged: u32 = kIOPMNotPowerManaged;
pub const IOPMSoftSleep: u32 = kIOPMSoftSleep;

pub const kIOPMNextHigherState: u32 = 1;
pub const kIOPMHighestState: u32 = 2;
pub const kIOPMNextLowerState: u32 = 3;
pub const kIOPMLowestState: u32 = 4;

pub const IOPMNextHigherState: u32 = kIOPMNextHigherState;
pub const IOPMHighestState: u32 = kIOPMHighestState;
pub const IOPMNextLowerState: u32 = kIOPMNextLowerState;
pub const IOPMLowestState: u32 = kIOPMLowestState;

// Internal commands used by power managment command queue
pub const kIOPMBroadcastAggressiveness: u32 = 1;
pub const kIOPMUnidleDevice: u32 = 2;

// Power consumption unknown value
pub const kIOPMUnknown: u32 = 0xFFFF;

// AppleClamshellState
pub const kAppleClamshellStateKey: *const ::std::os::raw::c_char =
    b"AppleClamshellState\x00" as *const [u8; 20usize] as *const ::std::os::raw::c_char;

// AppleClamshellCausesSleep
pub const kAppleClamshellCausesSleepKey: *const ::std::os::raw::c_char =
    b"AppleClamshellCausesSleep\x00" as *const [u8; 26usize] as *const ::std::os::raw::c_char;

// kIOPMSleepWakeUUIDKey
pub const kIOPMSleepWakeUUIDKey: *const ::std::os::raw::c_char =
    b"SleepWakeUUID\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;

// kIOPMBootSessionUUIDKey
pub const kIOPMBootSessionUUIDKey: *const ::std::os::raw::c_char =
    b"BootSessionUUID\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;

// kIOPMDeepSleepEnabledKey
pub const kIOPMDeepSleepEnabledKey: *const ::std::os::raw::c_char =
    b"Standby Enabled\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;

// kIOPMDeepSleepDelayKey
pub const kIOPMDeepSleepDelayKey: *const ::std::os::raw::c_char =
    b"Standby Delay\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;

// kIOPMDeepSleepDelayHighKey
pub const kIOPMDeepSleepDelayHighKey: *const ::std::os::raw::c_char =
    b"High Standby Delay\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;

// kIOPMLowBatteryThresholdKey
pub const kIOPMStandbyBatteryThresholdKey: *const ::std::os::raw::c_char =
    b"Standby Battery Threshold\x00" as *const [u8; 26usize] as *const ::std::os::raw::c_char;

// kIOPMDestroyFVKeyOnStandbyKey
pub const kIOPMDestroyFVKeyOnStandbyKey: *const ::std::os::raw::c_char =
    b"DestroyFVKeyOnStandby\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;

// kIOPMResetPowerStateOnWakeKey
pub const kIOPMResetPowerStateOnWakeKey: *const ::std::os::raw::c_char =
    b"IOPMResetPowerStateOnWake\x00" as *const [u8; 26usize] as *const ::std::os::raw::c_char;

// Driver Assertion bitfield description
pub const kIOPMDriverAssertionCPUBit: u32 = 0x01;
pub const kIOPMDriverAssertionUSBExternalDeviceBit: u32 = 0x04;
pub const kIOPMDriverAssertionBluetoothHIDDevicePairedBit: u32 = 0x08;
pub const kIOPMDriverAssertionExternalMediaMountedBit: u32 = 0x10;
pub const kIOPMDriverAssertionReservedBit5: u32 = 0x20;
pub const kIOPMDriverAssertionPreventDisplaySleepBit: u32 = 0x40;
pub const kIOPMDriverAssertionReservedBit7: u32 = 0x80;
pub const kIOPMDriverAssertionMagicPacketWakeEnabledBit: u32 = 0x100;
pub const kIOPMDriverAssertionNetworkKeepAliveActiveBit: u32 = 0x200;

// kIOPMAssertionsDriverKey
pub const kIOPMAssertionsDriverKey: *const ::std::os::raw::c_char =
    b"DriverPMAssertions\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPMAssertionsDriverDetailedKey: *const ::std::os::raw::c_char =
    b"DriverPMAssertionsDetailed\x00" as *const [u8; 27usize] as *const ::std::os::raw::c_char;

// Kernel Driver assertion detailed dictionary keys
pub const kIOPMDriverAssertionIDKey: *const ::std::os::raw::c_char =
    b"ID\x00" as *const [u8; 3usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionCreatedTimeKey: *const ::std::os::raw::c_char =
    b"CreatedTime\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionModifiedTimeKey: *const ::std::os::raw::c_char =
    b"ModifiedTime\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionOwnerStringKey: *const ::std::os::raw::c_char =
    b"Owner\x00" as *const [u8; 6usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionOwnerServiceKey: *const ::std::os::raw::c_char =
    b"ServicePtr\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionRegistryEntryIDKey: *const ::std::os::raw::c_char =
    b"RegistryEntryID\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionLevelKey: *const ::std::os::raw::c_char =
    b"Level\x00" as *const [u8; 6usize] as *const ::std::os::raw::c_char;
pub const kIOPMDriverAssertionAssertedKey: *const ::std::os::raw::c_char =
    b"Assertions\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;

// Root Domain general interest messages
pub const kClamshellStateBit: u32 = 1 << 0;
pub const kClamshellSleepBit: u32 = 1 << 1;
pub const kInflowForciblyEnabledBit: u32 = 1 << 0;

// Power commands issued to root domain
pub const kIOPMSleepNow: u32 = 1 << 0;
pub const kIOPMAllowSleep: u32 = 1 << 1;
pub const kIOPMPreventSleep: u32 = 1 << 2;
pub const kIOPMPowerButton: u32 = 1 << 3;
pub const kIOPMClamshellClosed: u32 = 1 << 4;
pub const kIOPMPowerEmergency: u32 = 1 << 5;
pub const kIOPMDisableClamshell: u32 = 1 << 6;
pub const kIOPMEnableClamshell: u32 = 1 << 7;
pub const kIOPMProcessorSpeedChange: u32 = 1 << 8;
pub const kIOPMOverTemp: u32 = 1 << 9;
pub const kIOPMClamshellOpened: u32 = 1 << 10;
pub const kIOPMDWOverTemp: u32 = 1 << 11;

// Power Management Return Codes
pub const kIOPMNoErr: u32 = 0;
pub const kIOPMAckImplied: u32 = 0;
pub const kIOPMWillAckLater: u32 = 1;
pub const kIOPMBadSpecification: u32 = 4;
pub const kIOPMNoSuchState: u32 = 5;
pub const kIOPMCannotRaisePower: u32 = 6;
pub const kIOPMParameterError: u32 = 7;
pub const kIOPMNotYetInitialized: u32 = 8;
// And the old constants; deprecated
pub const IOPMNoErr: u32 = kIOPMNoErr;
pub const IOPMAckImplied: u32 = kIOPMAckImplied;
pub const IOPMWillAckLater: u32 = kIOPMWillAckLater;
pub const IOPMBadSpecification: u32 = kIOPMBadSpecification;
pub const IOPMNoSuchState: u32 = kIOPMNoSuchState;
pub const IOPMCannotRaisePower: u32 = kIOPMCannotRaisePower;
pub const IOPMParameterError: u32 = kIOPMParameterError;
pub const IOPMNotYetInitialized: u32 = kIOPMNotYetInitialized;

// IOPMPowerSource class descriptive strings
pub const kIOPMPSExternalConnectedKey: *const ::std::os::raw::c_char =
    b"ExternalConnected\x00" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSExternalChargeCapableKey: *const ::std::os::raw::c_char =
    b"ExternalChargeCapable\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSBatteryInstalledKey: *const ::std::os::raw::c_char =
    b"BatteryInstalled\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSIsChargingKey: *const ::std::os::raw::c_char =
    b"IsCharging\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPMFullyChargedKey: *const ::std::os::raw::c_char =
    b"FullyCharged\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAtWarnLevelKey: *const ::std::os::raw::c_char =
    b"AtWarnLevel\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAtCriticalLevelKey: *const ::std::os::raw::c_char =
    b"AtCriticalLevel\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSCurrentCapacityKey: *const ::std::os::raw::c_char =
    b"CurrentCapacity\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSMaxCapacityKey: *const ::std::os::raw::c_char =
    b"MaxCapacity\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSDesignCapacityKey: *const ::std::os::raw::c_char =
    b"DesignCapacity\x00" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSTimeRemainingKey: *const ::std::os::raw::c_char =
    b"TimeRemaining\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAmperageKey: *const ::std::os::raw::c_char =
    b"Amperage\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSVoltageKey: *const ::std::os::raw::c_char =
    b"Voltage\x00" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSCycleCountKey: *const ::std::os::raw::c_char =
    b"CycleCount\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSMaxErrKey: *const ::std::os::raw::c_char =
    b"MaxErr\x00" as *const [u8; 7usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterInfoKey: *const ::std::os::raw::c_char =
    b"AdapterInfo\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSLocationKey: *const ::std::os::raw::c_char =
    b"Location\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSErrorConditionKey: *const ::std::os::raw::c_char =
    b"ErrorCondition\x00" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSManufacturerKey: *const ::std::os::raw::c_char =
    b"Manufacturer\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSManufactureDateKey: *const ::std::os::raw::c_char =
    b"ManufactureDate\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSModelKey: *const ::std::os::raw::c_char =
    b"Model\x00" as *const [u8; 6usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSSerialKey: *const ::std::os::raw::c_char =
    b"Serial\x00" as *const [u8; 7usize] as *const ::std::os::raw::c_char;
pub const kIOPMDeviceNameKey: *const ::std::os::raw::c_char =
    b"DeviceName\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSLegacyBatteryInfoKey: *const ::std::os::raw::c_char =
    b"LegacyBatteryInfo\x00" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSBatteryHealthKey: *const ::std::os::raw::c_char =
    b"BatteryHealth\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSHealthConfidenceKey: *const ::std::os::raw::c_char =
    b"HealthConfidence\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSCapacityEstimatedKey: *const ::std::os::raw::c_char =
    b"CapacityEstimated\x00" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSBatteryChargeStatusKey: *const ::std::os::raw::c_char =
    b"ChargeStatus\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSBatteryTemperatureKey: *const ::std::os::raw::c_char =
    b"Temperature\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsKey: *const ::std::os::raw::c_char =
    b"AdapterDetails\x00" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSChargerConfigurationKey: *const ::std::os::raw::c_char =
    b"ChargerConfiguration\x00" as *const [u8; 21usize] as *const ::std::os::raw::c_char;

// kIOPMPSBatteryChargeStatusKey may have one of the following values
pub const kIOPMBatteryChargeStatusTooHot: *const ::std::os::raw::c_char =
    b"HighTemperature\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMBatteryChargeStatusTooCold: *const ::std::os::raw::c_char =
    b"LowTemperature\x00" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPMBatteryChargeStatusTooHotOrCold: *const ::std::os::raw::c_char =
    b"HighOrLowTemperature\x00" as *const [u8; 21usize] as *const ::std::os::raw::c_char;
pub const kIOPMBatteryChargeStatusGradient: *const ::std::os::raw::c_char =
    b"BatteryTemperatureGradient\x00" as *const [u8; 27usize] as *const ::std::os::raw::c_char;

// Definitions for battery location, in case of multiple batteries.
pub const kIOPMPSLocationLeft: u32 = 1001;
pub const kIOPMPSLocationRight: u32 = 1002;

// Battery quality health types, specified by BatteryHealth and HealthConfidence
pub const kIOPMUndefinedValue: u32 = 0;
pub const kIOPMPoorValue: u32 = 1;
pub const kIOPMFairValue: u32 = 2;
pub const kIOPMGoodValue: u32 = 3;

// Keys for kIOPMPSAdapterDetailsKey dictionary
pub const kIOPMPSAdapterDetailsIDKey: *const ::std::os::raw::c_char =
    b"AdapterID\x00" as *const [u8; 10usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsWattsKey: *const ::std::os::raw::c_char =
    b"Watts\x00" as *const [u8; 6usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsRevisionKey: *const ::std::os::raw::c_char =
    b"AdapterRevision\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsSerialNumberKey: *const ::std::os::raw::c_char =
    b"SerialNumber\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsFamilyKey: *const ::std::os::raw::c_char =
    b"FamilyCode\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsAmperageKey: *const ::std::os::raw::c_char =
    b"Amperage\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsDescriptionKey: *const ::std::os::raw::c_char =
    b"Description\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsPMUConfigurationKey: *const ::std::os::raw::c_char =
    b"PMUConfiguration\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsVoltage: *const ::std::os::raw::c_char =
    b"AdapterVoltage\x00" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsSourceIDKey: *const ::std::os::raw::c_char =
    b"SourceID\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsErrorFlagsKey: *const ::std::os::raw::c_char =
    b"ErrorFlags\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsSharedSourceKey: *const ::std::os::raw::c_char =
    b"SharedSource\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSAdapterDetailsCloakedKey: *const ::std::os::raw::c_char =
    b"CloakedSource\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;

// values for kIOPSPowerAdapterFamilyKey
pub const kIOPSFamilyCodeDisconnected: i32 = 0;
pub const kIOPSFamilyCodeUnsupported: i32 = -536870201;
pub const kIOPSFamilyCodeFirewire: i32 = -536838144;
pub const kIOPSFamilyCodeUSBHost: i32 = -536854528;
pub const kIOPSFamilyCodeUSBHostSuspended: i32 = -536854527;
pub const kIOPSFamilyCodeUSBDevice: i32 = -536854526;
pub const kIOPSFamilyCodeUSBAdapter: i32 = -536854525;
pub const kIOPSFamilyCodeUSBChargingPortDedicated: i32 = -536854524;
pub const kIOPSFamilyCodeUSBChargingPortDownstream: i32 = -536854523;
pub const kIOPSFamilyCodeUSBChargingPort: i32 = -536854522;
pub const kIOPSFamilyCodeUSBUnknown: i32 = -536854521;
pub const kIOPSFamilyCodeUSBCBrick: i32 = -536854520;
pub const kIOPSFamilyCodeUSBCTypeC: i32 = -536854519;
pub const kIOPSFamilyCodeUSBCPD: i32 = -536854518;
pub const kIOPSFamilyCodeAC: i32 = -536723456;
pub const kIOPSFamilyCodeExternal: i32 = -536723455;
pub const kIOPSFamilyCodeExternal2: i32 = -536723454;
pub const kIOPSFamilyCodeExternal3: i32 = -536723453;
pub const kIOPSFamilyCodeExternal4: i32 = -536723452;
pub const kIOPSFamilyCodeExternal5: i32 = -536723451;

// values for kIOPMPSAdapterDetailsErrorFlagsKey
pub const kIOPSAdapterErrorFlagNoErrors: u32 = 0;
pub const kIOPSAdapterErrorFlagInsufficientAvailablePower: u32 = 2;
pub const kIOPSAdapterErrorFlagForeignObjectDetected: u32 = 4;
pub const kIOPSAdapterErrorFlagDeviceNeedsToBeRepositioned: u32 = 8;

pub const kIOPMPSInvalidWakeSecondsKey: *const ::std::os::raw::c_char =
    b"BatteryInvalidWakeSeconds\x00" as *const [u8; 26usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSPostChargeWaitSecondsKey: *const ::std::os::raw::c_char =
    b"PostChargeWaitSeconds\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;
pub const kIOPMPSPostDishargeWaitSecondsKey: *const ::std::os::raw::c_char =
    b"PostDischargeWaitSeconds\x00" as *const [u8; 25usize] as *const ::std::os::raw::c_char;

// CPU Power Management status keys
pub const kIOPMGraphicsPowerLimitsKey: *const ::std::os::raw::c_char =
    b"Graphics_Power_Limits\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;
pub const kIOPMGraphicsPowerLimitPerformanceKey: *const ::std::os::raw::c_char =
    b"Graphics_Power_Performance\x00" as *const [u8; 27usize] as *const ::std::os::raw::c_char;
pub const kIOPMCPUPowerLimitsKey: *const ::std::os::raw::c_char =
    b"CPU_Power_Limits\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPMCPUPowerLimitProcessorSpeedKey: *const ::std::os::raw::c_char =
    b"CPU_Speed_Limit\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPMCPUPowerLimitProcessorCountKey: *const ::std::os::raw::c_char =
    b"CPU_Available_CPUs\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPMCPUPowerLimitSchedulerTimeKey: *const ::std::os::raw::c_char =
    b"CPU_Scheduler_Limit\x00" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPMThermalLevelWarningKey: *const ::std::os::raw::c_char =
    b"Thermal_Level_Warning\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;

// Thermal Warning Level values
pub const kIOPMThermalLevelNormal: u32 = 0;
pub const kIOPMThermalLevelDanger: u32 = 5;
pub const kIOPMThermalLevelCritical: u32 = 10;
pub const kIOPMThermalLevelWarning: u32 = 100;
pub const kIOPMThermalLevelTrap: u32 = 110;
pub const kIOPMThermalLevelUnknown: u32 = 255;

// PM Settings Controller setting types
pub const kIOPMSettingWakeOnRingKey: *const ::std::os::raw::c_char =
    b"Wake On Modem Ring\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingRestartOnPowerLossKey: *const ::std::os::raw::c_char =
    b"Automatic Restart On Power Loss\x00" as *const [u8; 32usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingWakeOnACChangeKey: *const ::std::os::raw::c_char =
    b"Wake On AC Change\x00" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingSleepOnPowerButtonKey: *const ::std::os::raw::c_char =
    b"Sleep On Power Button\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingWakeOnClamshellKey: *const ::std::os::raw::c_char =
    b"Wake On Clamshell Open\x00" as *const [u8; 23usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingReduceBrightnessKey: *const ::std::os::raw::c_char =
    b"ReduceBrightness\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingDisplaySleepUsesDimKey: *const ::std::os::raw::c_char =
    b"Display Sleep Uses Dim\x00" as *const [u8; 23usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingTimeZoneOffsetKey: *const ::std::os::raw::c_char =
    b"TimeZoneOffsetSeconds\x00" as *const [u8; 22usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingMobileMotionModuleKey: *const ::std::os::raw::c_char =
    b"MobileMotionModule\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingGraphicsSwitchKey: *const ::std::os::raw::c_char =
    b"GPUSwitch\x00" as *const [u8; 10usize] as *const ::std::os::raw::c_char;

// Setting controlling drivers can register to receive scheduled wake data
pub const kIOPMSettingAutoWakeSecondsKey: *const ::std::os::raw::c_char =
    b"wake\x00" as *const [u8; 5usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingAutoWakeCalendarKey: *const ::std::os::raw::c_char =
    b"WakeByCalendarDate\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingAutoPowerSecondsKey: *const ::std::os::raw::c_char =
    b"poweron\x00" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingAutoPowerCalendarKey: *const ::std::os::raw::c_char =
    b"PowerByCalendarDate\x00" as *const [u8; 20usize] as *const ::std::os::raw::c_char;

// Debug seconds auto wake
pub const kIOPMSettingDebugWakeRelativeKey: *const ::std::os::raw::c_char =
    b"WakeRelativeToSleep\x00" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPMSettingDebugPowerRelativeKey: *const ::std::os::raw::c_char =
    b"PowerRelativeToShutdown\x00" as *const [u8; 24usize] as *const ::std::os::raw::c_char;

// Maintenance wake calendar.
pub const kIOPMSettingMaintenanceWakeCalendarKey: *const ::std::os::raw::c_char =
    b"MaintenanceWakeCalendarDate\x00" as *const [u8; 28usize] as *const ::std::os::raw::c_char;

// SetAggressiveness types
pub const kPMGeneralAggressiveness: u32 = 0;
pub const kPMMinutesToDim: u32 = 1;
pub const kPMMinutesToSpinDown: u32 = 2;
pub const kPMMinutesToSleep: u32 = 3;
pub const kPMEthernetWakeOnLANSettings: u32 = 4;
pub const kPMSetProcessorSpeed: u32 = 5;
pub const kPMPowerSource: u32 = 6;
pub const kPMMotionSensor: u32 = 7;
pub const kPMLastAggressivenessType: u32 = 8;

// SetAggressiveness values for the kPMPowerSource aggressiveness type
pub const kIOPMInternalPower: u32 = 1;
pub const kIOPMExternalPower: u32 = 2;

pub const kIOREMSleepEnabledKey: *const ::std::os::raw::c_char =
    b"REMSleepEnabled\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;

// Strings for deciphering the dictionary returned from IOPMCopyBatteryInfo
pub const kIOBatteryInfoKey: *const ::std::os::raw::c_char =
    b"IOBatteryInfo\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOBatteryCurrentChargeKey: *const ::std::os::raw::c_char =
    b"Current\x00" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOBatteryCapacityKey: *const ::std::os::raw::c_char =
    b"Capacity\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOBatteryFlagsKey: *const ::std::os::raw::c_char =
    b"Flags\x00" as *const [u8; 6usize] as *const ::std::os::raw::c_char;
pub const kIOBatteryVoltageKey: *const ::std::os::raw::c_char =
    b"Voltage\x00" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOBatteryAmperageKey: *const ::std::os::raw::c_char =
    b"Amperage\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOBatteryCycleCountKey: *const ::std::os::raw::c_char =
    b"Cycle Count\x00" as *const [u8; 12usize] as *const ::std::os::raw::c_char;

pub const kIOBatteryInstalled: u32 = 1 << 2;
pub const kIOBatteryCharge: u32 = 1 << 1;
pub const kIOBatteryChargerConnect: u32 = 1 << 0;

pub const IOPM_POWER_SOURCE_REV: u32 = 2;

// For use with IOPMPowerSource bFlags
pub const kIOPMACInstalled: u32 = kIOBatteryChargerConnect;
pub const kIOPMBatteryCharging: u32 = kIOBatteryCharge;
pub const kIOPMBatteryInstalled: u32 = kIOBatteryInstalled;
pub const kIOPMUPSInstalled: u32 = 1 << 3;
pub const kIOPMBatteryAtWarn: u32 = 1 << 4;
pub const kIOPMBatteryDepleted: u32 = 1 << 5;
pub const kIOPMACnoChargeCapability: u32 = 1 << 6;
pub const kIOPMRawLowBattery: u32 = 1 << 7;
pub const kIOPMForceLowSpeed: u32 = 1 << 8;
pub const kIOPMClosedClamshell: u32 = 1 << 9;
pub const kIOPMClamshellStateOnWake: u32 = 1 << 10;

// IOPMSystemCapabilityChangeFlags
pub const kIOPMSystemCapabilityWillChange: u32 = 0x01;
pub const kIOPMSystemCapabilityDidChange: u32 = 0x02;

pub const kIOPMSystemCapabilityCPU: u32 = 0x01;
pub const kIOPMSystemCapabilityGraphics: u32 = 0x02;
pub const kIOPMSystemCapabilityAudio: u32 = 0x04;
pub const kIOPMSystemCapabilityNetwork: u32 = 0x08;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOPMCalendarStruct {
    pub year: ::std::os::raw::c_uint,
    pub month: ::std::os::raw::c_uchar,
    pub day: ::std::os::raw::c_uchar,
    pub hour: ::std::os::raw::c_uchar,
    pub minute: ::std::os::raw::c_uchar,
    pub second: ::std::os::raw::c_uchar,
    pub selector: ::std::os::raw::c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOPowerStateChangeNotification {
    pub powerRef: *mut ::std::os::raw::c_void,
    pub returnValue: ::std::os::raw::c_ulong,
    pub stateNumber: ::std::os::raw::c_ulong,
    pub stateFlags: IOPMPowerFlags,
}
pub type sleepWakeNote = IOPowerStateChangeNotification;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOPMSystemCapabilityChangeParameters {
    pub notifyRef: u32,
    pub maxWaitForReply: u32,
    pub changeFlags: u32,
    pub __reserved1: u32,
    pub fromCapabilities: u32,
    pub toCapabilities: u32,
    pub __reserved2: [u32; 4usize],
}
