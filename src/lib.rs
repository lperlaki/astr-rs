#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

use core::{array::TryFromSliceError, str::Utf8Error};

/// # astr
/// Build an AStr from a string literal.
///
/// ```rust
/// use astr::astr;
///
/// let s = astr!("iam a string");
/// assert_eq!(s, "iam a string");
/// ```
///
#[macro_export]
macro_rules! astr {
    ($input:expr) => {
        unsafe {
            const LEN: usize = $input.len();
            // this is safa because we know that the bytes are valid utf8
            $crate::AStr::<LEN>::from_utf8_array_unchecked_ref(&*$input.as_ptr().cast::<[u8; LEN]>())
        }
    };
}

/// A str with a copiletime length.
/// 
/// This is a wrapper around an array of bytes representing an utf-8 string.
/// 
/// use the `astr!` macro to create an AStr from a string literal.
/// 
/// ```rust
/// use astr::astr;
/// 
/// let s = astr!("iam a string");
/// assert_eq!(s, "iam a string");
/// ```
/// 
/// if you want to create an AStr from a string or string reference, use the `AStr::try_from`
/// 
/// ```rust
/// use astr::AStr;
/// 
/// let s = AStr::<11>::try_from("Hallo World").unwrap();
/// assert_eq!(s, "Hallo World");
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AStr<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> std::hash::Hash for AStr<LEN> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum AStrError {
    Utf8(Utf8Error),
    Slice(TryFromSliceError),
}

impl From<Utf8Error> for AStrError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<TryFromSliceError> for AStrError {
    fn from(err: TryFromSliceError) -> Self {
        Self::Slice(err)
    }
}

impl core::fmt::Display for AStrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Utf8(err) => err.fmt(f),
            Self::Slice(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Utf8(ref err) => Some(err),
            Self::Slice(ref err) => Some(err),
        }
    }
}

impl<const LEN: usize> AStr<LEN> {
    /// Create a new AStr from an array of bytes.
    /// # Safety
    /// The slice must be valid UTF-8.
    pub const unsafe fn from_utf8_array_unchecked(arr: [u8; LEN]) -> Self {
        *Self::from_utf8_array_unchecked_ref(&arr)
    }

    /// Create a new AStr from an array of bytes.
    /// # Safety
    /// The slice must be valid UTF-8.
    pub const unsafe fn from_utf8_array_unchecked_ref(arr: &[u8; LEN]) -> &Self {
        core::mem::transmute(arr)
    }

    /// Create a new AStr from an array of bytes.
    /// # Safety
    /// The slice must be valid UTF-8.
    pub unsafe fn from_utf8_array_unchecked_mut(arr: &mut [u8; LEN]) -> &mut Self {
        core::mem::transmute(arr)
    }

    /// Create a new AStr from a slice of bytes.
    pub fn from_utf8_array_ref(arr: &[u8; LEN]) -> Result<&Self, AStrError> {
        core::str::from_utf8(arr)?;
        Ok(unsafe { Self::from_utf8_array_unchecked_ref(arr) })
    }

    /// Create a new AStr from a slice of bytes.
    pub fn from_utf8_array_mut(arr: &mut [u8; LEN]) -> Result<&mut Self, AStrError> {
        core::str::from_utf8_mut(arr)?;
        Ok(unsafe { Self::from_utf8_array_unchecked_mut(arr) })
    }

    /// Create a new AStr from a slice of bytes.
    pub fn from_utf8(slice: &[u8]) -> Result<&Self, AStrError> {
        Ok(Self::from_utf8_array_ref(slice.try_into()?)?)
    }

    /// Create a new AStr from a slice of bytes.
    pub fn from_utf8_mut(slice: &mut [u8]) -> Result<&mut Self, AStrError> {
        Ok(Self::from_utf8_array_mut(slice.try_into()?)?)
    }

    /// Create a new AStr from a str
    pub fn from_str_ref(str: &str) -> Result<&Self, AStrError> {
        let arr = str.as_bytes().try_into()?;
        Ok(unsafe { Self::from_utf8_array_unchecked_ref(arr) })
    }

    /// Create a new AStr from a str
    pub fn from_str_mut(str: &mut str) -> Result<&mut Self, AStrError> {
        Ok(unsafe {
            let arr = str.as_bytes_mut().try_into()?;
            Self::from_utf8_array_unchecked_mut(arr)
        })
    }

    /// get byte representation of the AStr
    pub const fn as_bytes_array(&self) -> &[u8; LEN] {
        &self.0
    }

    /// get mutable byte representation of the AStr
    /// # Safety
    /// Invariant: the byte array must be valid UTF-8.
    pub unsafe fn as_bytes_array_mut(&mut self) -> &mut [u8; LEN] {
        &mut self.0
    }

    /// get byte representation of the AStr
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// get mutable byte representation of the AStr
    /// # Safety
    /// Invariant: the byte array must be valid UTF-8.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// get str representation of the AStr
    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// get mutable str representation of the AStr
    pub fn as_str_mut(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }
}

impl<const LEN: usize> AsRef<str> for AStr<LEN> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const LEN: usize> AsMut<str> for AStr<LEN> {
    fn as_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl<const LEN: usize> core::borrow::Borrow<str> for AStr<LEN> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const LEN: usize> core::borrow::BorrowMut<str> for AStr<LEN> {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl<const LEN: usize> AsRef<[u8]> for AStr<LEN> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

// Should be Unsize<str> but that's unstable
impl<const LEN: usize> core::ops::Deref for AStr<LEN> {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const LEN: usize> core::ops::DerefMut for AStr<LEN> {
    fn deref_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl<const LEN: usize> core::fmt::Debug for AStr<LEN> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<const LEN: usize> core::fmt::Display for AStr<LEN> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a, const LEN: usize> TryFrom<&'a str> for &'a AStr<LEN> {
    type Error = AStrError;

    fn try_from(str: &'a str) -> Result<Self, Self::Error> {
        AStr::from_str_ref(str)
    }
}

impl<'a, const LEN: usize> TryFrom<&'a mut str> for &'a mut AStr<LEN> {
    type Error = AStrError;

    fn try_from(str: &'a mut str) -> Result<Self, Self::Error> {
        AStr::from_str_mut(str)
    }
}

impl<const LEN: usize> TryFrom<&'_ str> for AStr<LEN> {
    type Error = AStrError;

    fn try_from(str: &'_ str) -> Result<Self, Self::Error> {
        Ok(*AStr::from_str_ref(str)?)
    }
}

impl<const LEN: usize> core::str::FromStr for AStr<LEN> {
    type Err = AStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AStr::try_from(s)
    }
}

impl<const LEN: usize> PartialEq<str> for AStr<LEN> {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl<const LEN: usize> PartialEq<AStr<LEN>> for &AStr<LEN> {
    fn eq(&self, other: &AStr<LEN>) -> bool {
        AStr::<LEN>::eq(self, other)
    }
}

impl<const LEN: usize> PartialEq<&'_ str> for AStr<LEN> {
    fn eq(&self, other: &&'_ str) -> bool {
        self.eq(*other)
    }
}

impl<const LEN: usize> PartialEq<AStr<LEN>> for str {
    fn eq(&self, other: &AStr<LEN>) -> bool {
        self.eq(other.as_str())
    }
}

impl<const LEN: usize> PartialEq<AStr<LEN>> for &'_ str {
    fn eq(&self, other: &AStr<LEN>) -> bool {
        (*self).eq(other)
    }
}

impl<I: core::slice::SliceIndex<str>, const LEN: usize> core::ops::Index<I> for AStr<LEN> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl<I: core::slice::SliceIndex<str>, const LEN: usize> core::ops::IndexMut<I> for AStr<LEN> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_str_mut().index_mut(index)
    }
}

#[cfg(feature = "std")]
impl<const LEN: usize> AsRef<std::ffi::OsStr> for AStr<LEN> {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

#[cfg(feature = "std")]
impl<const LEN: usize> AsRef<std::path::Path> for AStr<LEN> {
    fn as_ref(&self) -> &std::path::Path {
        self.as_str().as_ref()
    }
}

#[cfg(feature = "std")]
impl<const LEN: usize> From<AStr<LEN>> for String {
    fn from(s: AStr<LEN>) -> Self {
        s.as_str().into()
    }
}


#[cfg(feature = "std")]
impl<const LEN: usize> TryFrom<String> for AStr<LEN> {
    type Error = AStrError;

    fn try_from(str: String) -> Result<Self, Self::Error> {
        Ok(*AStr::from_str_ref(&str)?)
    }
}


impl Default for AStr<0> {
    fn default() -> Self {
        AStr([])
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::AStr;
    use serde::{
        de::{self, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    impl<const LEN: usize> Serialize for AStr<LEN> {
        fn serialize<S: Serializer>(&'_ self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(self.as_str())
        }
    }

    struct AStrVisitor<const LEN: usize>;

    impl<'de, const LEN: usize> Visitor<'de> for AStrVisitor<LEN> {
        type Value = AStr<LEN>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a string of length {}", LEN)
        }

        #[inline]
        fn visit_str<E: de::Error>(self, s: &'_ str) -> Result<Self::Value, E> {
            AStr::try_from(s).map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
        }
    }

    impl<'de, const LEN: usize> Deserialize<'de> for AStr<LEN> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_str(AStrVisitor::<LEN>)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{astr, AStr};

    #[test]
    fn test_const() {
        const TEST_STR: AStr<4> = *astr!("test");
        assert_eq!(TEST_STR.as_str(), "test");
    }

    #[test]
    fn test_a() {
        let s = astr!("hello");

        assert_eq!(s.len(), 5);
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_index() {
        let s = astr!("hello world");

        assert_eq!(&s[0..5], "hello");
    }

    #[test]
    fn test_to_string() {
        let s = *astr!("hello");

        assert_eq!(s.to_string(), "hello");
    }

    #[test]
    fn test_from_string() {
        const S: &str = "hello";
        let s = *astr!(S);

        assert_eq!(s.to_string(), "hello");
    }

    #[test]
    fn test_cstr() {
        let s = *astr!("hello\0");
        let str = s.as_str();
        let cstr = std::ffi::CStr::from_bytes_with_nul(str.as_bytes()).unwrap();
        assert_eq!(cstr.to_str().unwrap(), "hello");
    }
}
