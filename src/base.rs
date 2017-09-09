use libc::{c_int, c_uint, c_ulonglong};

// exports from `mach/i386/boolean.h`
#[cfg(target_arch = "x86_64")]
pub type boolean_t = c_uint;

#[cfg(not(target_arch = "x86_64"))]
pub type boolean_t = c_int;

// exports from `mach/clock_types.h`
pub type alarm_type_t = c_int;
pub type sleep_type_t = c_int;
pub type clock_id_t = c_int;
pub type clock_flavor_t = c_int;
pub type clock_attr_t = *mut c_int;
pub type clock_res_t = c_int;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct mach_timespec {
    tv_sec: c_uint,
    tv_nsec: clock_res_t,
}
impl Clone for mach_timespec {
    fn clone(&self) -> Self {
        *self
    }
}

pub type mach_timespec_t = mach_timespec;

// exports from `mach/kern_return.h` and `mach/i386/kern_return.h`
pub type kern_return_t = c_int;

pub const KERN_SUCCESS: kern_return_t = 0;

// Specified address is not currently valid.
pub const KERN_INVALID_ADDRESS: kern_return_t = 1;

// Specified memory is valid, but does not permit the
// required forms of access.
pub const KERN_PROTECTION_FAILURE: kern_return_t = 2;

// The address range specified is already in use, or
// no address range of the size specified could be
// found.
pub const KERN_NO_SPACE: kern_return_t = 3;

// The function requested was not applicable to this
// type of argument, or an argument is invalid
pub const KERN_INVALID_ARGUMENT: kern_return_t = 4;

// The function could not be performed. A catch-all.
pub const KERN_FAILURE: kern_return_t = 5;

// A system resource could not be allocated to fulfill
// this request. This failure may not be permanent.
pub const KERN_RESOURCE_SHORTAGE: kern_return_t = 6;

// The task in question does not hold receive rights
// for the port argument.
pub const KERN_NOT_RECEIVER: kern_return_t = 7;

// Bogus access restriction.
pub const KERN_NO_ACCESS: kern_return_t = 8;

// During a page fault, the target address refers to a
// memory object that has been destroyed. This
// failure is permanent.
pub const KERN_MEMORY_FAILURE: kern_return_t = 9;

// During a page fault, the memory object indicated
// that the data could not be returned. This failure
// may be temporary; future attempts to access this
// same data may succeed, as defined by the memory
// object.
pub const KERN_MEMORY_ERROR: kern_return_t = 10;

// The receive right is already a member of the portset.
pub const KERN_ALREADY_IN_SET: kern_return_t = 11;

// The receive right is not a member of a port set.
pub const KERN_NOT_IN_SET: kern_return_t = 12;

// The name already denotes a right in the task.
pub const KERN_NAME_EXISTS: kern_return_t = 13;

// The operation was aborted. Ipc code will
// catch this and reflect it as a message error.
pub const KERN_ABORTED: kern_return_t = 14;

// The name doesn't denote a right in the task.
pub const KERN_INVALID_NAME: kern_return_t = 15;

// Target task isn't an active task.
pub const KERN_INVALID_TASK: kern_return_t = 16;

// The name denotes a right, but not an appropriate right.
pub const KERN_INVALID_RIGHT: kern_return_t = 17;

// A blatant range error.
pub const KERN_INVALID_VALUE: kern_return_t = 18;

// Operation would overflow limit on user-references.
pub const KERN_UREFS_OVERFLOW: kern_return_t = 19;

// The supplied (port) capability is improper.
pub const KERN_INVALID_CAPABILITY: kern_return_t = 20;

// The task already has send or receive rights
// for the port under another name.
pub const KERN_RIGHT_EXISTS: kern_return_t = 21;

// Target host isn't actually a host.
pub const KERN_INVALID_HOST: kern_return_t = 22;

// An attempt was made to supply "precious" data
// for memory that is already present in a
// memory object.
pub const KERN_MEMORY_PRESENT: kern_return_t = 23;

// A page was requested of a memory manager via
// memory_object_data_request for an object using
// a MEMORY_OBJECT_COPY_CALL strategy, with the
// VM_PROT_WANTS_COPY flag being used to specify
// that the page desired is for a copy of the
// object, and the memory manager has detected
// the page was pushed into a copy of the object
// while the kernel was walking the shadow chain
// from the copy to the object. This error code
// is delivered via memory_object_data_error
// and is handled by the kernel (it forces the
// kernel to restart the fault). It will not be
// seen by users.
pub const KERN_MEMORY_DATA_MOVED: kern_return_t = 24;

// A strategic copy was attempted of an object
// upon which a quicker copy is now possible.
// The caller should retry the copy using
// vm_object_copy_quickly. This error code
// is seen only by the kernel.
pub const KERN_MEMORY_RESTART_COPY: kern_return_t = 25;

// An argument applied to assert processor set privilege
// was not a processor set control port.
pub const KERN_INVALID_PROCESSOR_SET: kern_return_t = 26;

// The specified scheduling attributes exceed the thread's
// limits.
pub const KERN_POLICY_LIMIT: kern_return_t = 27;

// The specified scheduling policy is not currently
// enabled for the processor set.
pub const KERN_INVALID_POLICY: kern_return_t = 28;

// The external memory manager failed to initialize the
// memory object.
pub const KERN_INVALID_OBJECT: kern_return_t = 29;

// A thread is attempting to wait for an event for which
// there is already a waiting thread.
pub const KERN_ALREADY_WAITING: kern_return_t = 30;

// An attempt was made to destroy the default processor
// set.
pub const KERN_DEFAULT_SET: kern_return_t = 31;

// An attempt was made to fetch an exception port that is
// protected, or to abort a thread while processing a
// protected exception.
pub const KERN_EXCEPTION_PROTECTED: kern_return_t = 32;

// A ledger was required but not supplied.
pub const KERN_INVALID_LEDGER: kern_return_t = 33;

// The port was not a memory cache control port.
pub const KERN_INVALID_MEMORY_CONTROL: kern_return_t = 34;

// An argument supplied to assert security privilege
// was not a host security port.
pub const KERN_INVALID_SECURITY: kern_return_t = 35;

// thread_depress_abort was called on a thread which
// was not currently depressed.
pub const KERN_NOT_DEPRESSED: kern_return_t = 36;

// Object has been terminated and is no longer available
pub const KERN_TERMINATED: kern_return_t = 37;

// Lock set has been destroyed and is no longer available.
pub const KERN_LOCK_SET_DESTROYED: kern_return_t = 38;

// The thread holding the lock terminated before releasing
// the lock
pub const KERN_LOCK_UNSTABLE: kern_return_t = 39;

// The lock is already owned by another thread
pub const KERN_LOCK_OWNED: kern_return_t = 40;

// The lock is already owned by the calling thread
pub const KERN_LOCK_OWNED_SELF: kern_return_t = 41;

// Semaphore has been destroyed and is no longer available.
pub const KERN_SEMAPHORE_DESTROYED: kern_return_t = 42;

// Return from RPC indicating the target server was
// terminated before it successfully replied
pub const KERN_RPC_SERVER_TERMINATED: kern_return_t = 43;

// Terminate an orphaned activation.
pub const KERN_RPC_TERMINATE_ORPHAN: kern_return_t = 44;

// Allow an orphaned activation to continue executing.
pub const KERN_RPC_CONTINUE_ORPHAN: kern_return_t = 45;

// Empty thread activation (No thread linked to it)
pub const KERN_NOT_SUPPORTED: kern_return_t = 46;

// Remote node down or inaccessible.
pub const KERN_NODE_DOWN: kern_return_t = 47;

// A signalled thread was not actually waiting.
pub const KERN_NOT_WAITING: kern_return_t = 48;

// Some thread-oriented operation (semaphore_wait) timed out
pub const KERN_OPERATION_TIMED_OUT: kern_return_t = 49;

// Maximum return value allowable
pub const KERN_RETURN_MAX: kern_return_t = 0x100;

// exports from `mach/i386/vm_types.h`
pub type natural_t = c_uint;

pub type vm_offset_t = natural_t;
pub type vm_size_t = natural_t;

pub type mach_vm_address_t = c_uint;
pub type mach_vm_offset_t = c_uint;
pub type mach_vm_size_t = c_uint;

// exports from `mach/message.h`
pub type mach_msg_bits_t = c_uint;
pub type mach_msg_size_t = natural_t;
pub type mach_msg_id_t = c_int;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct mach_msg_header_t {
    pub msgh_bits: mach_msg_bits_t,
    pub msgh_size: mach_msg_size_t,
    pub msgh_remote_port: mach_port_t,
    pub msgh_local_port: mach_port_t,
    pub msgh_voucher_port: mach_port_name_t,
    pub msgh_id: mach_msg_id_t,
}
impl Clone for mach_msg_header_t {
    fn clone(&self) -> Self {
        *self
    }
}

// exports from `mach/port.h`
pub type mach_port_t = c_uint;
pub type mach_port_name_t = natural_t;

// exports from `mach/mach_types.h`
pub type task_t = mach_port_t;
pub type task_port_t = task_t;

// exports from `mach/vm_types.h`
pub type pointer_t = vm_offset_t;
pub type vm_address_t = vm_offset_t;
pub type vm_object_offset_t = c_ulonglong;

// exports from `libdispatch`
#[repr(C)]
#[derive(Debug, Copy)]
pub struct dispatch_queue_s {
    _address: u8,
}
impl Clone for dispatch_queue_s {
    fn clone(&self) -> Self {
        *self
    }
}
pub type dispatch_queue_t = *mut dispatch_queue_s;
