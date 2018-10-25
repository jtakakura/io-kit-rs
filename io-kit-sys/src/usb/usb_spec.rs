// exports from <IOKit/usb/USBSpec.h>

pub const kUSBDeviceClass: *const ::std::os::raw::c_char =
    b"bDeviceClass\x00" as *const [u8; 13usize] as *const ::std::os::raw::c_char;
pub const kUSBDeviceSubClass: *const ::std::os::raw::c_char =
    b"bDeviceSubClass\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kUSBDeviceProtocol: *const ::std::os::raw::c_char =
    b"bDeviceProtocol\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kUSBDeviceMaxPacketSize: *const ::std::os::raw::c_char =
    b"bMaxPacketSize0\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kUSBVendorID: *const ::std::os::raw::c_char =
    b"idVendor\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kUSBVendorName: *const ::std::os::raw::c_char = kUSBVendorID;
pub const kUSBProductID: *const ::std::os::raw::c_char =
    b"idProduct\x00" as *const [u8; 10usize] as *const ::std::os::raw::c_char;
pub const kUSBProductName: *const ::std::os::raw::c_char = kUSBProductID;
pub const kUSBDeviceReleaseNumber: *const ::std::os::raw::c_char =
    b"bcdDevice\x00" as *const [u8; 10usize] as *const ::std::os::raw::c_char;
pub const kUSBManufacturerStringIndex: *const ::std::os::raw::c_char =
    b"iManufacturer\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kUSBProductStringIndex: *const ::std::os::raw::c_char =
    b"iProduct\x00" as *const [u8; 9usize] as *const ::std::os::raw::c_char;
pub const kUSBSerialNumberStringIndex: *const ::std::os::raw::c_char =
    b"iSerialNumber\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kUSBDeviceNumConfigs: *const ::std::os::raw::c_char =
    b"bNumConfigurations\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kUSBInterfaceNumber: *const ::std::os::raw::c_char =
    b"bInterfaceNumber\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kUSBAlternateSetting: *const ::std::os::raw::c_char =
    b"bAlternateSetting\x00" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kUSBNumEndpoints: *const ::std::os::raw::c_char =
    b"bNumEndpoints\x00" as *const [u8; 14usize] as *const ::std::os::raw::c_char;
pub const kUSBInterfaceClass: *const ::std::os::raw::c_char =
    b"bInterfaceClass\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kUSBInterfaceSubClass: *const ::std::os::raw::c_char =
    b"bInterfaceSubClass\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kUSBInterfaceProtocol: *const ::std::os::raw::c_char =
    b"bInterfaceProtocol\x00" as *const [u8; 19usize] as *const ::std::os::raw::c_char;
pub const kUSBInterfaceStringIndex: *const ::std::os::raw::c_char =
    b"iInterface\x00" as *const [u8; 11usize] as *const ::std::os::raw::c_char;
pub const kUSBConfigurationValue: *const ::std::os::raw::c_char =
    b"bConfigurationValue\x00" as *const [u8; 20usize] as *const ::std::os::raw::c_char;
pub const kUSBProductString: *const ::std::os::raw::c_char =
    b"USB Product Name\x00" as *const [u8; 17usize] as *const ::std::os::raw::c_char;
pub const kUSBVendorString: *const ::std::os::raw::c_char =
    b"USB Vendor Name\x00" as *const [u8; 16usize] as *const ::std::os::raw::c_char;
pub const kUSBSerialNumberString: *const ::std::os::raw::c_char =
    b"USB Serial Number\x00" as *const [u8; 18usize] as *const ::std::os::raw::c_char;
pub const kUSB1284DeviceID: *const ::std::os::raw::c_char =
    b"1284 Device ID\x00" as *const [u8; 15usize] as *const ::std::os::raw::c_char;
