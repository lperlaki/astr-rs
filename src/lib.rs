#![cfg_attr(not(feature = "std"), no_std)]

use core::{array::TryFromSliceError, str::Utf8Error};

#[macro_export]
macro_rules! astr {
    ($input:expr) => {
        unsafe {
            const LEN: usize = $input.len();
            $crate::AStr::<LEN>::from_utf8_array_unchecked(&*$input.as_ptr().cast::<[u8; LEN]>())
        }
    };
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AStr<const SIZE: usize>([u8; SIZE]);

#[derive(Debug, Clone)]
enum AStrErrorInner {
    Utf8(Utf8Error),
    Slice(TryFromSliceError),
}


#[derive(Debug, Clone)]
pub struct AStrError(AStrErrorInner);

impl From<Utf8Error> for AStrError {
    fn from(err: Utf8Error) -> Self {
        Self(AStrErrorInner::Utf8(err))
    }
}

impl From<TryFromSliceError> for AStrError {
    fn from(err: TryFromSliceError) -> Self {
        Self(AStrErrorInner::Slice(err))
    }
}

impl core::fmt::Display for AStrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            AStrErrorInner::Utf8(err) => err.fmt(f),
            AStrErrorInner::Slice(err) => err.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AStrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.0 {
            AStrErrorInner::Utf8(ref err) => Some(err),
            AStrErrorInner::Slice(ref err) => Some(err),
        }
    }
}

impl<const SIZE: usize> AStr<SIZE> {
    pub const unsafe fn from_utf8_array_unchecked(arr: &[u8; SIZE]) -> &Self {
        core::mem::transmute(arr)
    }

    pub unsafe fn from_utf8_array_unchecked_mut(arr: &mut [u8; SIZE]) -> &mut Self {
        core::mem::transmute(arr)
    }

    pub fn from_utf8_array(arr: &[u8; SIZE]) -> Result<&Self, Utf8Error> {
        core::str::from_utf8(arr)?;
        Ok(unsafe { Self::from_utf8_array_unchecked(arr) })
    }

    pub fn from_utf8_array_mut(arr: &mut [u8; SIZE]) -> Result<&mut Self, Utf8Error> {
        core::str::from_utf8_mut(arr)?;
        Ok(unsafe { Self::from_utf8_array_unchecked_mut(arr) })
    }

    pub fn from_utf8(slice: &[u8]) -> Result<&Self, AStrError> {
        Ok(Self::from_utf8_array(slice.try_into()?)?)
    }

    pub fn from_utf8_mut(slice: &mut [u8]) -> Result<&mut Self, AStrError> {
        Ok(Self::from_utf8_array_mut(slice.try_into()?)?)
    }

    pub fn from_str_ref(str: &str) -> Result<&Self, TryFromSliceError> {
        str.as_bytes()
            .try_into()
            .map(|arr| unsafe { Self::from_utf8_array_unchecked(arr) })
    }

    pub fn from_str_mut(str: &mut str) -> Result<&mut Self, TryFromSliceError> {
        unsafe {
            str.as_bytes_mut()
                .try_into()
                .map(|arr| Self::from_utf8_array_unchecked_mut(arr))
        }
    }

    pub const fn as_bytes_array(&self) -> &[u8; SIZE] {
        &self.0
    }

    pub unsafe fn as_bytes_array_mut(&mut self) -> &mut [u8; SIZE] {
        &mut self.0
    }

    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub const fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn as_str_mut(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }
}

impl<const SIZE: usize> AsRef<str> for AStr<SIZE> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const SIZE: usize> AsMut<str> for AStr<SIZE> {
    fn as_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl<const SIZE: usize> core::borrow::Borrow<str> for AStr<SIZE> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const SIZE: usize> core::borrow::BorrowMut<str> for AStr<SIZE> {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl<const SIZE: usize> AsRef<[u8]> for AStr<SIZE> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}


// Should be Unsize<str> but that's unstable
impl<const SIZE: usize> core::ops::Deref for AStr<SIZE> {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const SIZE: usize> core::ops::DerefMut for AStr<SIZE> {
    fn deref_mut(&mut self) -> &mut str {
        self.as_str_mut()
    }
}

impl<const SIZE: usize> core::fmt::Debug for AStr<SIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<const SIZE: usize> core::fmt::Display for AStr<SIZE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<'a, const SIZE: usize> TryFrom<&'a str> for &'a AStr<SIZE> {
    type Error = TryFromSliceError;

    fn try_from(str: &'a str) -> Result<Self, Self::Error> {
        AStr::from_str_ref(str)
    }
}

impl<'a, const SIZE: usize> TryFrom<&'a mut str> for &'a mut AStr<SIZE> {
    type Error = TryFromSliceError;

    fn try_from(str: &'a mut str) -> Result<Self, Self::Error> {
        AStr::from_str_mut(str)
    }
}

impl<const SIZE: usize> TryFrom<&'_ str> for AStr<SIZE> {
    type Error = TryFromSliceError;

    fn try_from(str: &'_ str) -> Result<Self, Self::Error> {
        Ok(*AStr::from_str_ref(str)?)
    }
}

impl<const SIZE: usize> core::str::FromStr for AStr<SIZE> {
    type Err = TryFromSliceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AStr::try_from(s)
    }
}

impl<const SIZE: usize> PartialEq<str> for AStr<SIZE> {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl<const SIZE: usize> PartialEq<&'_ str> for AStr<SIZE> {
    fn eq(&self, other: &&'_ str) -> bool {
        self.eq(*other)
    }
}

impl<const SIZE: usize> PartialEq<AStr<SIZE>> for str {
    fn eq(&self, other: &AStr<SIZE>) -> bool {
        self.eq(other.as_str())
    }
}

impl<const SIZE: usize> PartialEq<AStr<SIZE>> for &'_ str {
    fn eq(&self, other: &AStr<SIZE>) -> bool {
        (*self).eq(other)
    }
}

impl<I: core::slice::SliceIndex<str>, const SIZE: usize> core::ops::Index<I> for AStr<SIZE> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl<I: core::slice::SliceIndex<str>, const SIZE: usize> core::ops::IndexMut<I> for AStr<SIZE> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_str_mut().index_mut(index)
    }
}

#[cfg(feature = "std")]
impl<const SIZE: usize> AsRef<std::ffi::OsStr> for AStr<SIZE> {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

#[cfg(feature = "std")]
impl<const SIZE: usize> AsRef<std::path::Path> for AStr<SIZE> {
    fn as_ref(&self) -> &std::path::Path {
        self.as_str().as_ref()
    }
}

#[cfg(feature = "std")]
impl<const SIZE: usize> From<AStr<SIZE>> for String {
    fn from(s: AStr<SIZE>) -> Self {
        s.as_str().into()
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

    impl<const SIZE: usize> Serialize for AStr<SIZE> {
        fn serialize<S: Serializer>(&'_ self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(self.as_str())
        }
    }

    struct AStrVisitor<const SIZE: usize>;

    impl<'de, const SIZE: usize> Visitor<'de> for AStrVisitor<SIZE> {
        type Value = AStr<SIZE>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a string of length {}", SIZE)
        }

        #[inline]
        fn visit_str<E: de::Error>(self, s: &'_ str) -> Result<Self::Value, E> {
            AStr::try_from(s)
                .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
        }
    }

    impl<'de, const SIZE: usize> Deserialize<'de> for AStr<SIZE> {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_str(AStrVisitor::<SIZE>)
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
}
