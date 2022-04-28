//! Bindings to the Windows SChannel APIs.
#![cfg(windows)]
#![warn(missing_docs)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate lazy_static;
extern crate windows;

use std::ffi::c_void;
use std::os::raw::c_ulong;
use std::ptr;

use windows::Win32::Security::Authentication::Identity;

macro_rules! inner {
    ($t:path, $raw:ty) => {
        impl crate::Inner<$raw> for $t {
            unsafe fn from_inner(t: $raw) -> Self {
                $t(t)
            }

            fn as_inner(&self) -> $raw {
                self.0
            }

            fn get_mut(&mut self) -> &mut $raw {
                &mut self.0
            }
        }

        impl crate::RawPointer for $t {
            unsafe fn from_ptr(t: *mut ::std::os::raw::c_void) -> $t {
                $t(t as _)
            }

            unsafe fn as_ptr(&self) -> *mut ::std::os::raw::c_void {
                self.0 as *mut _
            }
        }
    };
}

macro_rules! inner_newtype {
    ($t:path, $raw:path) => {
        impl crate::Inner<$raw> for $t {
            unsafe fn from_inner(t: $raw) -> Self {
                $t(t)
            }

            fn as_inner(&self) -> $raw {
                self.0
            }

            fn get_mut(&mut self) -> &mut $raw {
                &mut self.0
            }
        }

        impl crate::RawPointer for $t {
            unsafe fn from_ptr(t: *mut ::std::os::raw::c_void) -> $t {
                $t($raw(t as _))
            }

            unsafe fn as_ptr(&self) -> *mut ::std::os::raw::c_void {
                self.0 .0 as *mut _
            }
        }
    };
}

/// Allows access to the underlying schannel API representation of a wrapped data type
///
/// Performing actions with internal handles might lead to the violation of internal assumptions
/// and therefore is inherently unsafe.
pub trait RawPointer {
    /// Constructs an instance of this type from its handle / pointer.
    unsafe fn from_ptr(t: *mut ::std::os::raw::c_void) -> Self;

    /// Get a raw pointer from the underlying handle / pointer.
    unsafe fn as_ptr(&self) -> *mut ::std::os::raw::c_void;
}

pub mod cert_chain;
pub mod cert_context;
pub mod cert_store;
pub mod crypt_key;
pub mod crypt_prov;
/* pub */ mod ctl_context;
pub mod key_handle;
pub mod ncrypt_key;
pub mod schannel_cred;
pub mod tls_stream;

mod alpn_list;
mod context_buffer;
mod security_context;

#[cfg(test)]
mod test;

const ACCEPT_REQUESTS: c_ulong = Identity::ASC_REQ_ALLOCATE_MEMORY.0
    | Identity::ASC_REQ_CONFIDENTIALITY
    | Identity::ASC_REQ_SEQUENCE_DETECT.0
    | Identity::ASC_REQ_STREAM.0
    | Identity::ASC_REQ_REPLAY_DETECT.0;

const INIT_REQUESTS: c_ulong = Identity::ISC_REQ_CONFIDENTIALITY
    | Identity::ISC_REQ_INTEGRITY
    | Identity::ISC_REQ_REPLAY_DETECT
    | Identity::ISC_REQ_SEQUENCE_DETECT
    | Identity::ISC_REQ_MANUAL_CRED_VALIDATION
    | Identity::ISC_REQ_ALLOCATE_MEMORY
    | Identity::ISC_REQ_STREAM
    | Identity::ISC_REQ_USE_SUPPLIED_CREDS;

trait Inner<T> {
    unsafe fn from_inner(t: T) -> Self;

    fn as_inner(&self) -> T;

    fn get_mut(&mut self) -> &mut T;
}

unsafe fn secbuf(buftype: c_ulong, bytes: Option<&mut [u8]>) -> Identity::SecBuffer {
    let (ptr, len) = match bytes {
        Some(bytes) => (bytes.as_mut_ptr(), bytes.len() as c_ulong),
        None => (ptr::null_mut(), 0),
    };
    Identity::SecBuffer {
        BufferType: buftype,
        cbBuffer: len,
        pvBuffer: ptr as *mut c_void,
    }
}

unsafe fn secbuf_desc(bufs: &mut [Identity::SecBuffer]) -> Identity::SecBufferDesc {
    Identity::SecBufferDesc {
        ulVersion: Identity::SECBUFFER_VERSION,
        cBuffers: bufs.len() as c_ulong,
        pBuffers: bufs.as_mut_ptr(),
    }
}
