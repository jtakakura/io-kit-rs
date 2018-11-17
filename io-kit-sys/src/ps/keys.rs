// exports from <IOKit/ps/IOPSKeys.h>

// IOPSPowerAdapter Keys
pub const kIOPSPowerAdapterIDKey: *const ::std::os::raw::c_char =
    b"AdapterID\0" as *const [u8; 10usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerAdapterWattsKey: *const ::std::os::raw::c_char =
    b"Watts\0" as *const [u8; 6usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerAdapterRevisionKey: *const ::std::os::raw::c_char =
    b"AdapterRevision\0" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerAdapterSerialNumberKey: *const ::std::os::raw::c_char =
    b"SerialNumber\0" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerAdapterFamilyKey: *const ::std::os::raw::c_char =
    b"FamilyCode\0" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerAdapterCurrentKey: *const ::std::os::raw::c_char =
    b"Current\0" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerAdapterSourceKey: *const ::std::os::raw::c_char =
    b"Source\0" as *const [u8; 7usize] as *const ::std::os::raw::c_char;

// Internal Keys
pub const kIOPSUPSManagementClaimed: *const ::std::os::raw::c_char =
    b"/IOKit/UPSPowerManagementClaimed\0" as *const [u8; 33usize] as *const ::std::os::raw::c_char;
pub const kIOPSLowWarnLevelKey: *const ::std::os::raw::c_char =
    b"Low Warn Level\0" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPSDeadWarnLevelKey: *const ::std::os::raw::c_char =
    b"Shutdown Level\0" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPSDynamicStorePath: *const ::std::os::raw::c_char =
    b"/IOKit/PowerSources\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;

// Power Source Commands (UPS)
pub const kIOPSCommandDelayedRemovePowerKey: *const ::std::os::raw::c_char =
    b"Delayed Remove Power\0" as *const [u8; 21usize] as *const ::std::os::raw::c_char;
pub const kIOPSCommandEnableAudibleAlarmKey: *const ::std::os::raw::c_char =
    b"Enable Audible Alarm\0" as *const [u8; 21usize] as *const ::std::os::raw::c_char;
pub const kIOPSCommandStartupDelayKey: *const ::std::os::raw::c_char =
    b"Startup Delay\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPSCommandSetCurrentLimitKey: *const ::std::os::raw::c_char =
    b"Set Current Limit\0" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kIOPSCommandSetRequiredVoltageKey: *const ::std::os::raw::c_char =
    b"Set Required Voltage\0" as *const [u8; 21usize] as *const ::std::os::raw::c_char;
pub const kIOPSCommandSendCurrentStateOfCharge: *const ::std::os::raw::c_char =
    b"Send Current State of Charge\0" as *const [u8; 29usize] as *const ::std::os::raw::c_char;
pub const kIOPSCommandSendCurrentTemperature: *const ::std::os::raw::c_char =
    b"Send Current Temperature\0" as *const [u8; 25usize] as *const ::std::os::raw::c_char;

// Power Source data keys
pub const kIOPSPowerSourceIDKey: *const ::std::os::raw::c_char =
    b"Power Source ID\0" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPSPowerSourceStateKey: *const ::std::os::raw::c_char =
    b"Power Source State\0" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPSCurrentCapacityKey: *const ::std::os::raw::c_char =
    b"Current Capacity\0" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPSMaxCapacityKey: *const ::std::os::raw::c_char =
    b"Max Capacity\0" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kIOPSDesignCapacityKey: *const ::std::os::raw::c_char =
    b"DesignCapacity\0" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPSNominalCapacityKey: *const ::std::os::raw::c_char =
    b"Nominal Capacity\0" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPSTimeToEmptyKey: *const ::std::os::raw::c_char =
    b"Time to Empty\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPSTimeToFullChargeKey: *const ::std::os::raw::c_char =
    b"Time to Full Charge\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPSIsChargingKey: *const ::std::os::raw::c_char =
    b"Is Charging\0" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPSInternalFailureKey: *const ::std::os::raw::c_char =
    b"Internal Failure\0" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPSIsPresentKey: *const ::std::os::raw::c_char =
    b"Is Present\0" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPSVoltageKey: *const ::std::os::raw::c_char =
    b"Voltage\0" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOPSCurrentKey: *const ::std::os::raw::c_char =
    b"Current\0" as *const [u8; 8usize] as *const ::std::os::raw::c_char;
pub const kIOPSTemperatureKey: *const ::std::os::raw::c_char =
    b"Temperature\0" as *const [u8; 12usize] as *const ::std::os::raw::c_char;
pub const kIOPSNameKey: *const ::std::os::raw::c_char =
    b"Name\0" as *const [u8; 5usize] as *const ::std::os::raw::c_char;
pub const kIOPSTypeKey: *const ::std::os::raw::c_char =
    b"Type\0" as *const [u8; 5usize] as *const ::std::os::raw::c_char;
pub const kIOPSTransportTypeKey: *const ::std::os::raw::c_char =
    b"Transport Type\0" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPSVendorIDKey: *const ::std::os::raw::c_char =
    b"Vendor ID\0" as *const [u8; 10usize] as *const ::std::os::raw::c_char;
pub const kIOPSProductIDKey: *const ::std::os::raw::c_char =
    b"Product ID\0" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPSVendorDataKey: *const ::std::os::raw::c_char =
    b"Vendor Specific Data\0" as *const [u8; 21usize] as *const ::std::os::raw::c_char;
pub const kIOPSBatteryHealthKey: *const ::std::os::raw::c_char =
    b"BatteryHealth\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPSBatteryHealthConditionKey: *const ::std::os::raw::c_char =
    b"BatteryHealthCondition\0" as *const [u8; 23usize] as *const ::std::os::raw::c_char;
pub const kIOPSBatteryFailureModesKey: *const ::std::os::raw::c_char =
    b"BatteryFailureModes\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPSHealthConfidenceKey: *const ::std::os::raw::c_char =
    b"HealthConfidence\0" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPSMaxErrKey: *const ::std::os::raw::c_char =
    b"MaxErr\0" as *const [u8; 7usize] as *const ::std::os::raw::c_char;
pub const kIOPSIsChargedKey: *const ::std::os::raw::c_char =
    b"Is Charged\0" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPSIsFinishingChargeKey: *const ::std::os::raw::c_char =
    b"Is Finishing Charge\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPSHardwareSerialNumberKey: *const ::std::os::raw::c_char =
    b"Hardware Serial Number\0" as *const [u8; 23usize] as *const ::std::os::raw::c_char;

// Transport types
pub const kIOPSSerialTransportType: *const ::std::os::raw::c_char =
    b"Serial\0" as *const [u8; 7usize] as *const ::std::os::raw::c_char;
pub const kIOPSUSBTransportType: *const ::std::os::raw::c_char =
    b"USB\0" as *const [u8; 4usize] as *const ::std::os::raw::c_char;
pub const kIOPSNetworkTransportType: *const ::std::os::raw::c_char =
    b"Ethernet\0" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPSInternalType: *const ::std::os::raw::c_char =
    b"Internal\0" as *const [u8; 9usize] as *const ::std::os::raw::c_char;

// PowerSource Types
pub const kIOPSInternalBatteryType: *const ::std::os::raw::c_char =
    b"InternalBattery\0" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPSUPSType: *const ::std::os::raw::c_char =
    b"UPS\0" as *const [u8; 4usize] as *const ::std::os::raw::c_char;
pub const kIOPSOffLineValue: *const ::std::os::raw::c_char =
    b"Off Line\0" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPSACPowerValue: *const ::std::os::raw::c_char =
    b"AC Power\0" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kIOPSBatteryPowerValue: *const ::std::os::raw::c_char =
    b"Battery Power\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;

// Battery Health values
pub const kIOPSPoorValue: *const ::std::os::raw::c_char =
    b"Poor\0" as *const [u8; 5usize] as *const ::std::os::raw::c_char;
pub const kIOPSFairValue: *const ::std::os::raw::c_char =
    b"Fair\0" as *const [u8; 5usize] as *const ::std::os::raw::c_char;
pub const kIOPSGoodValue: *const ::std::os::raw::c_char =
    b"Good\0" as *const [u8; 5usize] as *const ::std::os::raw::c_char;

// Battery Health Condition values
pub const kIOPSCheckBatteryValue: *const ::std::os::raw::c_char =
    b"Check Battery\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPSPermanentFailureValue: *const ::std::os::raw::c_char =
    b"Permanent Battery Failure\0" as *const [u8; 26usize] as *const ::std::os::raw::c_char;

// Battery Failure Mode values
pub const kIOPSFailureExternalInput: *const ::std::os::raw::c_char =
    b"Externally Indicated Failure\0" as *const [u8; 29usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureSafetyOverVoltage: *const ::std::os::raw::c_char =
    b"Safety Over-Voltage\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureChargeOverTemp: *const ::std::os::raw::c_char =
    b"Charge Over-Temperature\0" as *const [u8; 24usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureDischargeOverTemp: *const ::std::os::raw::c_char =
    b"Discharge Over-Temperature\0" as *const [u8; 27usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureCellImbalance: *const ::std::os::raw::c_char =
    b"Cell Imbalance\0" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureChargeFET: *const ::std::os::raw::c_char =
    b"Charge FET\0" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureDischargeFET: *const ::std::os::raw::c_char =
    b"Discharge FET\0" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureDataFlushFault: *const ::std::os::raw::c_char =
    b"Data Flush Fault\0" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailurePermanentAFEComms: *const ::std::os::raw::c_char =
    b"Permanent AFE Comms\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailurePeriodicAFEComms: *const ::std::os::raw::c_char =
    b"Periodic AFE Comms\0" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureChargeOverCurrent: *const ::std::os::raw::c_char =
    b"Charge Over-Current\0" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureDischargeOverCurrent: *const ::std::os::raw::c_char =
    b"Discharge Over-Current\0" as *const [u8; 23usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureOpenThermistor: *const ::std::os::raw::c_char =
    b"Open Thermistor\0" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kIOPSFailureFuseBlown: *const ::std::os::raw::c_char =
    b"Fuse Blown\0" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
