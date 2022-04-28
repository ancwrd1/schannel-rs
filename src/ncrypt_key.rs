//! CNG private keys.

use windows::Win32::Security::Cryptography;

/// A CNG handle to a key.
pub struct NcryptKey(Cryptography::NCRYPT_KEY_HANDLE);

impl Drop for NcryptKey {
    fn drop(&mut self) {
        unsafe {
            let _ = Cryptography::NCryptFreeObject(Cryptography::NCRYPT_HANDLE(self.0 .0));
        }
    }
}

inner_newtype!(NcryptKey, Cryptography::NCRYPT_KEY_HANDLE);
