use std::borrow::{Borrow, Cow};
use std::ops::Deref;
use std::str::Utf8Error;

/// OsStr, but specifically for Linux (since we aren't always processing native dumps).
#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
pub struct LinuxOsStr([u8]);

/// OsString, but specifically for Linux (since we aren't always processing native dumps).
#[derive(Default, Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
pub struct LinuxOsString(Vec<u8>);

impl LinuxOsStr {
    pub fn new() -> &'static Self {
        Self::from_bytes(b"")
    }

    pub fn from_bytes(inner: &[u8]) -> &Self {
        // This is the idiom std uses for creating a type that wraps a slice.
        // Yes, there really isn't a way to do this without unsafe. No, it's
        // not at all a safety concern.
        unsafe { &*(inner as *const [u8] as *const LinuxOsStr) }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Tries to interpret the LinuxOsStr as a utf8 str.
    ///
    /// While linux OsStrs are "arbitrary bytes" in general, there are often
    /// parts that are known to be utf8 (ascii even).
    ///
    /// For instance, when parsing /proc/self/mem, most of the line is ascii
    /// like "r-xp" or "1a23-4fe2". However the "path" at the end of each line
    /// is a true LinuxOsStr and may not be proper utf8.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self)
    }

    /// Converts to a utf8 string lossily (uses the usual std lossy algorithm).
    pub fn to_string_lossy(&self) -> Cow<str> {
        // Ok so this is the end of the line for dancing around and acting
        // like we can just be fine with Linux OS strings being arbitrary bags
        // of bytes. We need some way to print this value in a reasonable way,
        // and the best precedent I can find for that is std::Path::display.
        // This wraps the a Path (which is just an OsStr) and provides a
        // Display impl.
        //
        // What does this Display impl do..?
        //
        // It just calls from_utf8_lossy.
        //
        // Whelp. Ok.
        //
        // (Strictly speaking it wraps it up in the internal/unstable
        // Utf8Lossy iterator so it avoids the allocation, but we don't
        // have that luxury, so we might as well make the allocation/conversion
        // explicit.)
        String::from_utf8_lossy(self.as_bytes())
    }

    // ~Copies of a bunch of string APIs since [u8] doesn't have them (reasonably)
    pub fn split_once(&self, separator: u8) -> Option<(&LinuxOsStr, &LinuxOsStr)> {
        self.iter().position(|&b| b == separator).map(|idx| {
            (
                Self::from_bytes(&self[..idx]),
                Self::from_bytes(&self[idx + 1..]),
            )
        })
    }
    pub fn rsplit_once(&self, separator: u8) -> Option<(&LinuxOsStr, &LinuxOsStr)> {
        self.iter().rposition(|&b| b == separator).map(|idx| {
            (
                Self::from_bytes(&self[..idx]),
                Self::from_bytes(&self[idx + 1..]),
            )
        })
    }

    pub fn split(&self, separator: u8) -> impl Iterator<Item = &LinuxOsStr> {
        self.as_bytes()
            .split(move |&b| b == separator)
            .map(LinuxOsStr::from_bytes)
    }

    pub fn split_ascii_whitespace(&self) -> impl Iterator<Item = &LinuxOsStr> {
        // Quick and dirty impl: just split on every individual whitespace
        // char but discard all the empty substrings.
        self.as_bytes()
            .split(|b| b.is_ascii_whitespace())
            .filter(|slice| !slice.is_empty())
            .map(LinuxOsStr::from_bytes)
    }

    pub fn lines(&self) -> impl Iterator<Item = &LinuxOsStr> {
        // Intentionally doesn't mess around with stuff like \r
        // since we're processing files generated by the OS, but maybe
        // this will be a problem later?
        self.split(b'\n')
    }

    pub fn trim_ascii_whitespace(&self) -> &LinuxOsStr {
        let input = self.as_bytes();

        let mut first = None;
        let mut last = None;

        // Find first non-whitespace index
        for (i, &c) in input.iter().enumerate() {
            if !c.is_ascii_whitespace() {
                first = Some(i);
                break;
            }
        }

        // Find last non-whitespace index
        for (i, &c) in input.iter().enumerate().rev() {
            if !c.is_ascii_whitespace() {
                last = Some(i);
                break;
            }
        }

        if let (Some(first), Some(last)) = (first, last) {
            Self::from_bytes(&input[first..=last])
        } else {
            // string was entirely whitespace, return an empty string starting
            // at its position (so that it's still strictly a substring).
            Self::from_bytes(&input[0..0])
        }
    }
}

impl LinuxOsString {
    /// Create a new LinuxOsString from an array of bytes.
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self(vec)
    }

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn as_os_str(&self) -> &LinuxOsStr {
        self
    }
}

impl Borrow<LinuxOsStr> for LinuxOsString {
    fn borrow(&self) -> &LinuxOsStr {
        LinuxOsStr::from_bytes(&self.0)
    }
}

impl ToOwned for LinuxOsStr {
    type Owned = LinuxOsString;

    fn to_owned(&self) -> LinuxOsString {
        LinuxOsString::from_vec(self.0.to_owned())
    }
}

impl Deref for LinuxOsString {
    type Target = LinuxOsStr;

    fn deref(&self) -> &LinuxOsStr {
        LinuxOsStr::from_bytes(&self.0)
    }
}

impl Deref for LinuxOsStr {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}
