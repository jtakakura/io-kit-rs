use std::ffi::{CStr, CString};
use std::mem;

use libc::c_char;

use core_foundation::base::TCFType;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;

use iokit_sys::*;
use iokit_sys::base::*;
use iokit_sys::mach_sys::kern_return_t;
use iokit_sys::types::{io_object_t, io_service_t, io_iterator_t};

pub struct IOObject(io_object_t);

impl Drop for IOObject {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl TIOObject<io_object_t> for IOObject {
    #[inline]
    fn as_concrete_io_object_t(&self) -> io_object_t {
        self.0
    }

    #[inline]
    fn as_io_object_t(&self) -> io_object_t {
        self.as_concrete_io_object_t()
    }
}

pub struct IOIterator(io_iterator_t);

impl Drop for IOIterator {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl Iterator for IOIterator {
    type Item = IOObject;

    fn next(&mut self) -> Option<IOObject> {
        unsafe {
            let result = IOIteratorNext(self.as_concrete_io_object_t());

            if result != 0 {
                Some(IOObject(result))
            } else {
                None
            }
        }
    }
}

impl IOIterator {
    pub fn reset(&self) {
        unsafe { IOIteratorReset(self.as_concrete_io_object_t()) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { IOIteratorIsValid(self.as_concrete_io_object_t()) != 0 }
    }
}

impl TIOObject<io_iterator_t> for IOIterator {
    #[inline]
    fn as_concrete_io_object_t(&self) -> io_iterator_t {
        self.0
    }

    #[inline]
    fn as_io_object_t(&self) -> io_object_t {
        self.as_concrete_io_object_t()
    }
}

pub struct IOService(io_service_t);

impl Drop for IOService {
    fn drop(&mut self) {
        self.release().unwrap();
    }
}

impl IOService {
    pub fn get_matching_service(matching: CFDictionary) -> Option<IOService> {
        unsafe {
            let result = IOServiceGetMatchingService(kIOMasterPortDefault,
                                                     matching.as_concrete_TypeRef());

            if result != 0 {
                Some(IOService(result))
            } else {
                None
            }
        }
    }

    pub fn get_matching_services(matching: CFDictionary) -> Result<IOIterator, KernReturn> {
        unsafe {
            let mut io_iterator_t: io_iterator_t = mem::uninitialized();

            let result = IOServiceGetMatchingServices(kIOMasterPortDefault,
                                                      matching.as_concrete_TypeRef(),
                                                      &mut io_iterator_t);

            if result == KERN_SUCCESS {
                Ok(IOIterator(io_iterator_t))
            } else {
                Err(KernReturn::from(result))
            }
        }
    }
}

impl TIOObject<io_service_t> for IOService {
    #[inline]
    fn as_concrete_io_object_t(&self) -> io_service_t {
        self.0
    }

    #[inline]
    fn as_io_object_t(&self) -> io_object_t {
        self.as_concrete_io_object_t()
    }
}

pub trait TIOObject<concrete_io_object_t> {
    /// Returns the object as its concrete `io_object_t`.
    fn as_concrete_io_object_t(&self) -> concrete_io_object_t;

    /// Returns the object as a raw `io_object_t`.
    fn as_io_object_t(&self) -> io_object_t;

    fn release(&self) -> Result<(), KernReturn> {
        unsafe {
            let result = IOObjectRelease(self.as_io_object_t());

            if result == KERN_SUCCESS {
                Ok(())
            } else {
                Err(KernReturn::from(result))
            }
        }
    }

    fn retain(&self) -> Result<(), KernReturn> {
        unsafe {
            let result = IOObjectRetain(self.as_io_object_t());

            if result == KERN_SUCCESS {
                Ok(())
            } else {
                Err(KernReturn::from(result))
            }
        }
    }

    fn get_class(&self) -> Result<String, KernReturn> {
        unsafe {
            let mut buf = Vec::<c_char>::with_capacity(128);

            let result = IOObjectGetClass(self.as_io_object_t(), buf.as_mut_ptr());

            if result == KERN_SUCCESS {
                Ok(String::from(CStr::from_ptr(buf.as_ptr()).to_str().unwrap().to_string()))
            } else {
                Err(KernReturn::from(result))
            }
        }
    }

    fn copy_class(&self) -> Option<CFString> {
        unsafe {
            let result = IOObjectCopyClass(self.as_io_object_t());

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }

    fn copy_superclass_for_class(&self, class_name: CFString) -> Option<CFString> {
        unsafe {
            let result = IOObjectCopySuperclassForClass(class_name.as_concrete_TypeRef());

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }

    fn copy_bundle_identifier_for_class(&self, class_name: CFString) -> Option<CFString> {
        unsafe {
            let result = IOObjectCopyBundleIdentifierForClass(class_name.as_concrete_TypeRef());

            if result.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(result))
            }
        }
    }

    fn conforms_to(&self, class_name: &str) -> bool {
        unsafe {
            let ptr = CString::new(class_name).unwrap().as_ptr() as *mut c_char;

            IOObjectConformsTo(self.as_io_object_t(), ptr) != 0
        }
    }

    fn is_equal_to(&self, object: IOObject) -> bool {
        unsafe { IOObjectIsEqualTo(self.as_io_object_t(), object.as_io_object_t()) != 0 }
    }

    fn get_kernel_retain_count(&self) -> u32 {
        unsafe { IOObjectGetKernelRetainCount(self.as_io_object_t()) }
    }

    fn get_user_retain_count(&self) -> u32 {
        unsafe { IOObjectGetUserRetainCount(self.as_io_object_t()) }
    }

    fn get_retain_count(&self) -> u32 {
        unsafe { IOObjectGetRetainCount(self.as_io_object_t()) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KernReturn {
    Success,
    InvalidAddress,
    ProtectionFailure,
    NoSpace,
    InvalidArgument,
    Failure,
    ResourceShortage,
    NotReceiver,
    NoAccess,
    MemoryFailure,
    MemoryError,
    AlreadyInSet,
    NotInSet,
    NameExists,
    Aborted,
    InvalidName,
    InvalidTask,
    InvalidRight,
    InvalidValue,
    UrefsOverflow,
    InvalidCapability,
    RightExists,
    InvalidHost,
    MemoryPresent,
    MemoryDataMoved,
    MemoryRestartCopy,
    InvalidProcessorSet,
    PolicyLimit,
    InvalidPolicy,
    InvalidObject,
    AlreadyWaiting,
    DefaultSet,
    ExceptionProtected,
    InvalidLedger,
    InvalidMemoryControl,
    InvalidSecurity,
    NotDepressed,
    Terminated,
    LockSetDestroyed,
    LockUnstable,
    LockOwned,
    LockOwnedSelf,
    SemaphoreDestroyed,
    RpcServerTerminated,
    RpcTerminateOrphan,
    RpcContinueOrphan,
    NotSupported,
    NodeDown,
    NotWaiting,
    OperationTimedOut,
    ReturnMax,
    Unknown(kern_return_t),
}

impl From<kern_return_t> for KernReturn {
    fn from(code: kern_return_t) -> KernReturn {
        match code {
            KERN_SUCCESS => KernReturn::Success,
            KERN_INVALID_ADDRESS => KernReturn::InvalidAddress,
            KERN_PROTECTION_FAILURE => KernReturn::ProtectionFailure,
            KERN_NO_SPACE => KernReturn::NoSpace,
            KERN_INVALID_ARGUMENT => KernReturn::InvalidArgument,
            KERN_FAILURE => KernReturn::Failure,
            KERN_RESOURCE_SHORTAGE => KernReturn::ResourceShortage,
            KERN_NOT_RECEIVER => KernReturn::NotReceiver,
            KERN_NO_ACCESS => KernReturn::NoAccess,
            KERN_MEMORY_FAILURE => KernReturn::MemoryFailure,
            KERN_MEMORY_ERROR => KernReturn::MemoryError,
            KERN_ALREADY_IN_SET => KernReturn::AlreadyInSet,
            KERN_NOT_IN_SET => KernReturn::NotInSet,
            KERN_NAME_EXISTS => KernReturn::NameExists,
            KERN_ABORTED => KernReturn::Aborted,
            KERN_INVALID_NAME => KernReturn::InvalidName,
            KERN_INVALID_TASK => KernReturn::InvalidTask,
            KERN_INVALID_RIGHT => KernReturn::InvalidRight,
            KERN_INVALID_VALUE => KernReturn::InvalidValue,
            KERN_UREFS_OVERFLOW => KernReturn::UrefsOverflow,
            KERN_INVALID_CAPABILITY => KernReturn::InvalidCapability,
            KERN_RIGHT_EXISTS => KernReturn::RightExists,
            KERN_INVALID_HOST => KernReturn::InvalidHost,
            KERN_MEMORY_PRESENT => KernReturn::MemoryPresent,
            KERN_MEMORY_DATA_MOVED => KernReturn::MemoryDataMoved,
            KERN_MEMORY_RESTART_COPY => KernReturn::MemoryRestartCopy,
            KERN_INVALID_PROCESSOR_SET => KernReturn::InvalidProcessorSet,
            KERN_POLICY_LIMIT => KernReturn::PolicyLimit,
            KERN_INVALID_POLICY => KernReturn::InvalidPolicy,
            KERN_INVALID_OBJECT => KernReturn::InvalidObject,
            KERN_ALREADY_WAITING => KernReturn::AlreadyWaiting,
            KERN_DEFAULT_SET => KernReturn::DefaultSet,
            KERN_EXCEPTION_PROTECTED => KernReturn::ExceptionProtected,
            KERN_INVALID_LEDGER => KernReturn::InvalidLedger,
            KERN_INVALID_MEMORY_CONTROL => KernReturn::InvalidMemoryControl,
            KERN_INVALID_SECURITY => KernReturn::InvalidSecurity,
            KERN_NOT_DEPRESSED => KernReturn::NotDepressed,
            KERN_TERMINATED => KernReturn::Terminated,
            KERN_LOCK_SET_DESTROYED => KernReturn::LockSetDestroyed,
            KERN_LOCK_UNSTABLE => KernReturn::LockUnstable,
            KERN_LOCK_OWNED => KernReturn::LockOwned,
            KERN_LOCK_OWNED_SELF => KernReturn::LockOwnedSelf,
            KERN_SEMAPHORE_DESTROYED => KernReturn::SemaphoreDestroyed,
            KERN_RPC_SERVER_TERMINATED => KernReturn::RpcServerTerminated,
            KERN_RPC_TERMINATE_ORPHAN => KernReturn::RpcTerminateOrphan,
            KERN_RPC_CONTINUE_ORPHAN => KernReturn::RpcContinueOrphan,
            KERN_NOT_SUPPORTED => KernReturn::NotSupported,
            KERN_NODE_DOWN => KernReturn::NodeDown,
            KERN_NOT_WAITING => KernReturn::NotWaiting,
            KERN_OPERATION_TIMED_OUT => KernReturn::OperationTimedOut,
            KERN_RETURN_MAX => KernReturn::ReturnMax,
            code => KernReturn::Unknown(code),
        }
    }
}
