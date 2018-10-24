// exports from <IOKit/usb/USBSpec.h>

pub const kUSBDeviceClass: *const ::libc::c_char =
    b"bDeviceClass\x00" as *const [u8; 13usize] as *const ::libc::c_char;
pub const kUSBDeviceSubClass: *const ::libc::c_char =
    b"bDeviceSubClass\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kUSBDeviceProtocol: *const ::libc::c_char =
    b"bDeviceProtocol\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kUSBDeviceMaxPacketSize: *const ::libc::c_char =
    b"bMaxPacketSize0\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kUSBVendorID: *const ::libc::c_char =
    b"idVendor\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kUSBVendorName: *const ::libc::c_char = kUSBVendorID;
pub const kUSBProductID: *const ::libc::c_char =
    b"idProduct\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kUSBProductName: *const ::libc::c_char = kUSBProductID;
pub const kUSBDeviceReleaseNumber: *const ::libc::c_char =
    b"bcdDevice\x00" as *const [u8; 10usize] as *const ::libc::c_char;
pub const kUSBManufacturerStringIndex: *const ::libc::c_char =
    b"iManufacturer\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kUSBProductStringIndex: *const ::libc::c_char =
    b"iProduct\x00" as *const [u8; 9usize] as *const ::libc::c_char;
pub const kUSBSerialNumberStringIndex: *const ::libc::c_char =
    b"iSerialNumber\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kUSBDeviceNumConfigs: *const ::libc::c_char =
    b"bNumConfigurations\x00" as *const [u8; 19usize] as *const ::libc::c_char;
pub const kUSBInterfaceNumber: *const ::libc::c_char =
    b"bInterfaceNumber\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kUSBAlternateSetting: *const ::libc::c_char =
    b"bAlternateSetting\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kUSBNumEndpoints: *const ::libc::c_char =
    b"bNumEndpoints\x00" as *const [u8; 14usize] as *const ::libc::c_char;
pub const kUSBInterfaceClass: *const ::libc::c_char =
    b"bInterfaceClass\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kUSBInterfaceSubClass: *const ::libc::c_char =
    b"bInterfaceSubClass\x00" as *const [u8; 19usize] as *const ::libc::c_char;
pub const kUSBInterfaceProtocol: *const ::libc::c_char =
    b"bInterfaceProtocol\x00" as *const [u8; 19usize] as *const ::libc::c_char;
pub const kUSBInterfaceStringIndex: *const ::libc::c_char =
    b"iInterface\x00" as *const [u8; 11usize] as *const ::libc::c_char;
pub const kUSBConfigurationValue: *const ::libc::c_char =
    b"bConfigurationValue\x00" as *const [u8; 20usize] as *const ::libc::c_char;
pub const kUSBProductString: *const ::libc::c_char =
    b"USB Product Name\x00" as *const [u8; 17usize] as *const ::libc::c_char;
pub const kUSBVendorString: *const ::libc::c_char =
    b"USB Vendor Name\x00" as *const [u8; 16usize] as *const ::libc::c_char;
pub const kUSBSerialNumberString: *const ::libc::c_char =
    b"USB Serial Number\x00" as *const [u8; 18usize] as *const ::libc::c_char;
pub const kUSB1284DeviceID: *const ::libc::c_char =
    b"1284 Device ID\x00" as *const [u8; 15usize] as *const ::libc::c_char;
