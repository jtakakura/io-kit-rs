// exports from <IOKit/serial/IOSerialKeys.h>

use ::std::os::raw::c_char;

pub const kIOSerialBSDServiceValue: *const c_char =
    b"IOSerialBSDClient\0" as *const [u8; 18] as *const c_char;
pub const kIOSerialBSDTypeKey: *const c_char =
    b"IOSerialBSDClientType\0" as *const [u8; 22] as *const c_char;
pub const kIOSerialBSDAllTypes: *const c_char =
    b"IOSerialStream\0" as *const [u8; 15] as *const c_char;
pub const kIOSerialBSDModemType: *const c_char =
    b"IOSerialStream\0" as *const [u8; 15] as *const c_char;
pub const kIOSerialBSDRS232Type: *const c_char =
    b"IOSerialStream\0" as *const [u8; 15] as *const c_char;
pub const kIOTTYDeviceKey: *const c_char =
    b"IOTTYDevice\0" as *const [u8; 12] as *const c_char;
pub const kIOTTYBaseNameKey: *const c_char =
    b"IOTTYBaseName\0" as *const [u8; 14] as *const c_char;
pub const kIOTTYSuffixKey: *const c_char =
    b"IOTTYSuffix\0" as *const [u8; 12] as *const c_char;
pub const kIOCalloutDeviceKey: *const c_char =
    b"IOCalloutDevice\0" as *const [u8; 16] as *const c_char;
pub const kIODialinDeviceKey: *const c_char =
    b"IODialinDevice\0" as *const [u8; 15] as *const c_char;
pub const kIOTTYWaitForIdleKey: *const c_char =
    b"IOTTYWaitForIdle\0" as *const [u8; 17] as *const c_char;
