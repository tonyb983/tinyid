// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # `tinyid`
//!
//! A tiny ID type that is definitely not cryptographically secure, but is easier for a user to
//! see and type in, while still being unique enough to create at least 1-10 million instances
//! without collision. It is also very efficiently stored, essentially taking up as much size as
//! a `u64`.
//!
//! ## Example Usage
//! ```
//! use tinyid::TinyId;
//!
//! let mut id = TinyId::random();
//! assert!(id.is_valid());
//! assert!(!id.is_null());
//! id.make_null();
//! assert!(!id.is_valid());
//! assert!(id.is_null());
//! assert_eq!(id, TinyId::null());
//! ```

#![cfg_attr(coverage, feature(no_coverage))]
#![deny(
    clippy::pedantic,
    clippy::all,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rustdoc::all,
    clippy::cargo,
    clippy::cargo_common_metadata
)]

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error type used by [`TinyId`] operations that are fallible.
pub enum TinyIdError {
    /// Error returned when a string has too many characters to be a valid [`TinyId`].
    InvalidLength,
    /// Error returned when a string has invalid characters to be a valid [`TinyId`].
    InvalidCharacters,
    /// A forwarded error message from a built-in conversion.
    Conversion(String),
    /// Error returned when ID generation fails.
    GenerationFailure,
}

impl std::fmt::Display for TinyIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TinyIdError::InvalidLength => write!(f, "Invalid length"),
            TinyIdError::InvalidCharacters => write!(f, "Invalid characters"),
            TinyIdError::Conversion(s) => write!(f, "Conversion error: {s}"),
            TinyIdError::GenerationFailure => write!(f, "TinyId generation failed"),
        }
    }
}

impl From<std::array::TryFromSliceError> for TinyIdError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Self::Conversion(err.to_string())
    }
}

impl std::error::Error for TinyIdError {}

/// A tiny 8-byte ID type that is **NOT** cryptographically secure, but is easy and convenient
/// for tasks that don't require the utmost security or uniqueness. During lightweight testing,
/// between 1 and 10 million IDs can be generated without any collisions, and performance has
/// been pretty good.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TinyId {
    data: [u8; 8],
}

impl TinyId {
    /// The number of letters that make up the potential pool of characters for a [`TinyId`].
    pub const LETTER_COUNT: usize = 64;
    /// The letter pool used during generation of a [`TinyId`].
    pub const LETTERS: [u8; Self::LETTER_COUNT] = [
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o',
        b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'A', b'B', b'C', b'D',
        b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S',
        b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
        b'9', b'0', b'_', b'-',
    ];
    /// The byte used to represent null data / ids.
    pub const NULL_CHAR: u8 = b'\0';
    /// An instance of a fully null byte array, used as the basis for null ids.
    pub const NULL_DATA: [u8; 8] = [Self::NULL_CHAR; 8];

    /// Test whether the given byte is valid for use as one of the 8 bytes in a [`TinyId`].
    ///
    /// This function exists because I'm tired of writing `if LETTERS.contains(&byte)` everywhere.
    /// Hopefully it is also more efficient to do it this way.
    /// Letters (as u8):
    /// [45, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57,
    ///  65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
    ///  76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86,
    ///  87, 88, 89, 90, 95, 97, 98, 99,
    /// 100, 101, 102, 103, 104, 105, 106, 107,
    /// 108, 109, 110, 111, 112, 113, 114, 115,
    /// 116, 117, 118, 119, 120, 121, 122]
    /// aka
    /// [45, 48..=57, 65..=90, 95, 97..=122]
    #[must_use]
    pub const fn is_valid_byte(byte: u8) -> bool {
        if byte == Self::NULL_CHAR {
            return false;
        }

        if byte == 45u8 {
            return true;
        }

        if byte >= 48u8 && byte <= 57u8 {
            return true;
        }

        if byte >= 65u8 && byte <= 90u8 {
            return true;
        }

        if byte == 95 {
            return true;
        }

        if byte >= 97u8 && byte <= 122u8 {
            return true;
        }

        false
    }

    /// Create an instance of the `null` [`TinyId`].
    #[must_use]
    pub fn null() -> Self {
        Self {
            data: Self::NULL_DATA,
        }
    }

    /// Create a new random [`TinyId`].
    #[must_use]
    pub fn random() -> Self {
        Self::random_fastrand2()
    }

    /// Checks whether this [`TinyId`] is null or has any invalid bytes.
    #[must_use]
    pub fn is_valid(self) -> bool {
        !self.is_null() && self.data.iter().all(|&ch| Self::is_valid_byte(ch))
    }

    /// Checks whether this [`TinyId`] is null.
    #[must_use]
    pub fn is_null(self) -> bool {
        self.data == Self::NULL_DATA
    }

    /// Makes this [`TinyId`] null.
    pub fn make_null(&mut self) {
        self.data = Self::NULL_DATA;
    }

    fn from_str(s: &str) -> std::result::Result<Self, TinyIdError> {
        use std::char::TryFromCharError;
        if s.len() != 8 {
            return Err(TinyIdError::InvalidLength);
        }

        let mut data = Self::NULL_DATA;
        for (i, ch) in s.chars().enumerate() {
            let byte: u8 = ch
                .try_into()
                .map_err(|err: TryFromCharError| TinyIdError::Conversion(err.to_string()))?;
            if !Self::is_valid_byte(byte) {
                return Err(TinyIdError::InvalidCharacters);
            }
            data[i] = byte;
        }
        Ok(Self { data })
    }

    /// Convert from [`&str`] to [`TinyId`], without checking the length or
    /// individual characters of the input.
    #[must_use]
    pub fn from_str_unchecked(s: &str) -> Self {
        let mut data = Self::NULL_DATA;
        for (i, ch) in s.bytes().enumerate() {
            data[i] = ch;
        }
        Self { data }
    }

    /// Convert this [`TinyId`] to an array of 8 bytes.
    #[must_use]
    pub fn to_bytes(self) -> [u8; 8] {
        self.data
    }

    /// Attempt to create a new [`TinyId`] from a u64.
    ///
    /// ## Errors
    /// - [`TinyIdError::InvalidLength`] if the input is not 8 bytes long.
    /// - [`TinyIdError::InvalidCharacters`] if the input contains invalid chars/bytes.
    pub fn from_u64(n: u64) -> Result<Self, TinyIdError> {
        let bytes: [u8; 8] = n.to_be_bytes();
        Self::from_bytes(bytes)
    }

    /// Creates a new [`TinyId`] from the given `u64`, without validating
    /// that the bytes are valid.
    #[must_use]
    pub fn from_u64_unchecked(n: u64) -> Self {
        let data: [u8; 8] = n.to_be_bytes();
        Self { data }
    }

    /// Convert this [`TinyId`] to a u64 representation.
    #[must_use]
    pub fn to_u64(self) -> u64 {
        u64::from_be_bytes(self.data)
    }

    /// Attempt to create a new [`TinyId`] from the given byte array.
    ///
    /// ## Errors
    /// - [`TinyIdError::InvalidCharacters`] if the input contains invalid chars/bytes.
    pub fn from_bytes(bytes: [u8; 8]) -> Result<Self, TinyIdError> {
        let id = Self { data: bytes };
        if id.is_valid() {
            Ok(id)
        } else {
            Err(TinyIdError::InvalidCharacters)
        }
    }

    /// Creates a new [`TinyId`] from the given `[u8; 8]`, without validating
    /// that the bytes are valid.
    #[must_use]
    pub fn from_bytes_unchecked(bytes: [u8; 8]) -> Self {
        Self { data: bytes }
    }

    /// Checks whether this [`TinyId`] starts with the given string. This converts `self` to string so
    /// any associated overhead is incurred.
    #[must_use]
    pub fn starts_with(&self, input: &str) -> bool {
        match input.len() {
            0 => true,
            1..=8 => {
                let s = self.to_string();
                s.starts_with(input)
            }
            _ => false,
        }
    }

    /// Checks whether this [`TinyId`] ends with the given string. This converts `self` to string so
    /// any associated overhead is incurred.
    #[must_use]
    pub fn ends_with(&self, input: &str) -> bool {
        match input.len() {
            0 => true,
            1..=8 => {
                let s = self.to_string();
                s.ends_with(input)
            }
            _ => false,
        }
    }

    /// Create a new random [`TinyId`].
    ///
    /// This method calls [`fastrand::u8`] 8 times. Twice as fast as [`TinyId::random_fastrand2`].
    #[allow(clippy::cast_possible_truncation, unused)]
    #[cfg_attr(coverage, no_coverage)]
    #[must_use]
    pub(crate) fn random_fastrand() -> Self {
        const LETTER_COUNT_U8: u8 = TinyId::LETTER_COUNT as u8;
        let mut data = Self::NULL_DATA;
        for ch in &mut data {
            *ch = Self::LETTERS[fastrand::u8(0..LETTER_COUNT_U8) as usize];
        }
        Self { data }
    }

    /// Create a new random [`TinyId`].
    ///
    /// This method uses a single call to [`fastrand::u64`], splits it into bytes, and uses
    /// them to index the letter array.
    #[must_use]
    pub(crate) fn random_fastrand2() -> Self {
        let seed = fastrand::u64(..);
        let mut data: [u8; 8] = seed.to_be_bytes();
        for b in &mut data {
            *b = Self::LETTERS[*b as usize % Self::LETTER_COUNT];
        }
        Self { data }
    }
}

impl TryFrom<[u8; 8]> for TinyId {
    type Error = TinyIdError;

    fn try_from(value: [u8; 8]) -> std::result::Result<Self, Self::Error> {
        let data = value;
        if data.iter().any(|&ch| !Self::is_valid_byte(ch)) {
            Err(TinyIdError::InvalidCharacters)
        } else {
            Ok(Self { data })
        }
    }
}

impl TryFrom<&[u8; 8]> for TinyId {
    type Error = TinyIdError;

    fn try_from(value: &[u8; 8]) -> std::result::Result<Self, Self::Error> {
        let data = *value;
        if data.iter().any(|&ch| !Self::is_valid_byte(ch)) {
            Err(TinyIdError::InvalidCharacters)
        } else {
            Ok(Self { data })
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for TinyId {
    type Error = TinyIdError;

    fn try_from(value: &'a [u8]) -> std::result::Result<Self, Self::Error> {
        let data = <[u8; 8]>::try_from(value)?;
        if data.iter().any(|&ch| !Self::is_valid_byte(ch)) {
            return Err(TinyIdError::InvalidCharacters);
        }
        Ok(Self { data })
    }
}

impl TryFrom<u64> for TinyId {
    type Error = TinyIdError;

    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        Self::from_u64(value)
    }
}

impl std::str::FromStr for TinyId {
    type Err = TinyIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl std::fmt::Display for TinyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ch in &self.data {
            write!(f, "{}", *ch as char)?;
        }
        Ok(())
    }
}

impl Default for TinyId {
    fn default() -> Self {
        Self::null()
    }
}

impl PartialEq<TinyId> for [u8; 8] {
    fn eq(&self, other: &TinyId) -> bool {
        self == &other.data
    }
}
impl PartialEq<[u8; 8]> for TinyId {
    fn eq(&self, other: &[u8; 8]) -> bool {
        self.data == *other
    }
}
impl PartialEq<[u8; 8]> for &TinyId {
    fn eq(&self, other: &[u8; 8]) -> bool {
        self.data == *other
    }
}
impl PartialEq<TinyId> for &[u8; 8] {
    fn eq(&self, other: &TinyId) -> bool {
        **self == other.data
    }
}
impl PartialEq<&[u8; 8]> for TinyId {
    fn eq(&self, other: &&[u8; 8]) -> bool {
        self.data == **other
    }
}
impl PartialEq<TinyId> for &[u8] {
    fn eq(&self, other: &TinyId) -> bool {
        *self == other.data.as_slice()
    }
}
impl PartialEq<&[u8]> for TinyId {
    fn eq(&self, other: &&[u8]) -> bool {
        self.data == *other
    }
}
impl PartialEq<TinyId> for Vec<u8> {
    fn eq(&self, other: &TinyId) -> bool {
        self == other.data.as_slice()
    }
}
impl PartialEq<Vec<u8>> for TinyId {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.data == *other.as_slice()
    }
}
impl PartialEq<TinyId> for &Vec<u8> {
    fn eq(&self, other: &TinyId) -> bool {
        self.as_slice() == other.data.as_slice()
    }
}
impl PartialEq<&Vec<u8>> for TinyId {
    fn eq(&self, other: &&Vec<u8>) -> bool {
        self.data == *other.as_slice()
    }
}
impl PartialEq<&TinyId> for TinyId {
    fn eq(&self, other: &&TinyId) -> bool {
        self.data == other.data
    }
}
impl PartialEq<TinyId> for &TinyId {
    fn eq(&self, other: &TinyId) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn error() {
        assert_eq!(TinyIdError::InvalidLength.to_string(), "Invalid length");
        assert_eq!(
            TinyIdError::InvalidCharacters.to_string(),
            "Invalid characters"
        );
        assert_eq!(
            TinyIdError::Conversion("Hello".to_string()).to_string(),
            "Conversion error: Hello"
        );
        assert_eq!(
            TinyIdError::GenerationFailure.to_string(),
            "TinyId generation failed"
        );
    }

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn letters() {
        for letter in &TinyId::LETTERS {
            assert!(
                TinyId::is_valid_byte(*letter),
                "{} (letter {}) failed",
                *letter,
                *letter as char
            );
        }
        assert!(!TinyId::is_valid_byte(TinyId::NULL_CHAR));
    }

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn basic_usage() {
        let id = TinyId::random();
        // println!("Created id {} ({:?})", id, id);
        assert!(id.is_valid());
        assert!(!id.is_null());
        let num = id.to_u64();
        let back = TinyId::from_u64(num).expect("Unable to convert back to u64");
        assert_eq!(id, back);
        assert_eq!(num, back.to_u64());
        let bytes = id.to_bytes();
        let back = TinyId::from_bytes(bytes).expect("Unable to convert back to bytes");
        assert_eq!(id, back);
        assert_eq!(bytes, back.to_bytes());
        let bad_id = TinyId::null();
        assert!(!bad_id.is_valid());
        assert!(bad_id.is_null());
    }

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn collision_test_one_million() {
        use std::collections::HashSet;
        let mut ids = HashSet::new();
        for _ in 0..1_000_000 {
            let id = TinyId::random();
            assert!(ids.insert(id));
        }
    }

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn froms() {
        let result = TinyId::from_bytes([b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        // println!("ID {} = u64 {}", id, id.to_u64());
        let result = TinyId::from_bytes([b'!', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(result.is_err());

        // ID 'abcdefgh' = 7_017_280_452_245_743_464_u64
        let result = TinyId::from_u64(7_017_280_452_245_743_464_u64);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        let result = TinyId::from_u64(u64::MAX);
        assert!(result.is_err());
        let result = TinyId::try_from(7_017_280_452_245_743_464_u64);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        let result = TinyId::try_from(u64::MAX);
        assert!(result.is_err());

        let result = <TinyId as std::str::FromStr>::from_str("abcdefgh");
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        let result = <TinyId as std::str::FromStr>::from_str("abcdefghijklmnopqrstuvwxyz");
        assert!(result.is_err());
        let result = <TinyId as std::str::FromStr>::from_str("!@#$%^&*");
        assert!(result.is_err());

        let result = TinyId::try_from([b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        let result = TinyId::try_from([b'!', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(result.is_err());

        let result = TinyId::try_from(&[b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        let result = TinyId::try_from(&[b'!', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(result.is_err());

        let result = TinyId::try_from(&[b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as &[u8]);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id.to_string(), "abcdefgh");
        let result = TinyId::try_from(&[b'!', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as &[u8]);
        assert!(result.is_err());
        let result = TinyId::try_from(&[b'!', b'b', b'c', b'd', b'e', b'f', b'g'] as &[u8]);
        assert!(result.is_err());
    }

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    fn bad_froms() {
        let id = TinyId::from_bytes_unchecked([b'!', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        assert!(!id.is_valid());
        let id = TinyId::from_u64_unchecked(u64::MAX);
        assert!(!id.is_valid());
        let id = TinyId::from_str_unchecked("abcdefg!");
        assert!(!id.is_valid());
    }

    #[test]
    #[should_panic]
    #[cfg_attr(coverage, no_coverage)]
    fn bad_froms_panic1() {
        let _id = TinyId::from_str_unchecked("oopsie poopsie!");
    }

    #[test]
    #[cfg_attr(coverage, no_coverage)]
    #[allow(clippy::op_ref)]
    fn eqs() {
        let mut id = TinyId::from_bytes_unchecked([b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
        let id2 = TinyId::from_u64_unchecked(id.to_u64());
        let id3 = TinyId::from_str_unchecked("abcdefgh");
        assert!(id.is_valid());
        assert!(id2.is_valid());
        assert!(id3.is_valid());
        assert!(id == id2);
        assert!(&id == id);
        assert!(id == &id2);
        assert!(&id2 == &id3);
        assert!(id2 == &id3.data);
        assert!(id == id2.data);
        assert!(id == id3.data.as_slice());
        assert!(id2 == id3.data.as_ref());
        assert!(id2 == id3.data.to_vec());
        assert!(id2 == &id.data.to_vec());
        assert!(id3 == id.data);
        assert!(id3 == [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as [u8; 8]);
        assert!(id == &[b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as &[u8; 8]);
        assert!(id2 == &[b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as &[u8]);
        assert!(&id3 == [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as [u8; 8]);
        assert!(&id == &[b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'] as &[u8; 8]);
        let bytes: [u8; 8] = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
        assert!(id == bytes);

        id.make_null();
        assert!(!id.is_valid());
        assert!(id.is_null());
        assert!(id.data == TinyId::NULL_DATA);
        assert!(id == TinyId::default());
    }
}
