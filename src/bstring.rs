use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;

#[cfg(unix)]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct BString(Vec<u8>);

impl BString {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        BString(vec)
    }

    pub fn from_bytes(bs: &[u8]) -> Self {
        BString(bs.into())
    }

    pub fn from_str(s: &str) -> Self {
        BString(s.as_bytes().into())
    }

    pub fn from_string(s: String) -> Self {
        BString(s.as_bytes().into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn starts_with(&self, pfx: &[u8]) -> bool {
        self.0.starts_with(pfx)
    }

    pub fn strip_prefix(&self, prefix: &BString) -> Option<&[u8]> {
        self.0.strip_prefix(prefix.0.as_slice())
    }

    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(self.to_os_str())
    }
}

impl<'a> IntoIterator for &'a BString {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl fmt::Display for BString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match std::str::from_utf8(self.0.as_ref()) {
            Ok(s) => write!(f, "{}", s),
            Err(..) => write!(f, "{}", String::from_utf8_lossy(self.0.as_ref())),
        }
    }
}

impl fmt::Debug for BString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_os_str())
    }
}

#[cfg(unix)]
impl BString {
    pub fn from_os_str(s: &OsStr) -> Self {
        Self(s.as_bytes().into())
    }

    pub fn to_os_str(&self) -> OsString {
        OsStr::from_bytes(self.0.as_ref()).into()
    }
}

#[cfg(not(unix))]
impl BString {
    pub fn from_os_str(s: &OsStr) -> Self {
        BString::from_str(&s.to_string_lossy().to_string())
    }

    pub fn to_os_str(&self) -> OsString {
        String::from_utf8_lossy(self.0.as_ref()).to_string().into()
    }
}
