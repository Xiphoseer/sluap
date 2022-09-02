use core::{
    fmt::{self, Write},
    str::Chars,
};

pub trait ByteLen {
    fn len(&self) -> usize;

    fn split_at(&self, mid: usize) -> (&Self, &Self);
}

impl ByteLen for [u8] {
    fn len(&self) -> usize {
        self.len()
    }

    fn split_at(&self, mid: usize) -> (&Self, &Self) {
        self.split_at(mid)
    }
}

impl ByteLen for str {
    fn len(&self) -> usize {
        self.len()
    }

    fn split_at(&self, mid: usize) -> (&Self, &Self) {
        self.split_at(mid)
    }
}

/// Infallible decoder
pub trait Decoder<'a>: Clone {
    type Slice: ?Sized + ByteLen;

    fn as_slice(&self) -> &'a Self::Slice;
    fn as_bytes(&self) -> &'a [u8];
    fn next_char(&mut self) -> Option<char>;
    fn peek_char(&mut self) -> Option<char>;
    fn skip_bytes(&mut self, count: usize);

    fn offset_from(&self, earlier: &[u8]) -> usize {
        let end = self.as_bytes().as_ptr();
        let range = earlier.as_ptr_range();
        assert!(range.start <= end && end <= range.end); // make sure end is actually contained in start
        (unsafe { end.offset_from(range.start) }) as usize
    }
}

#[derive(Clone)]
pub struct Latin1Decoder<'a> {
    rest: &'a [u8],
}

const W1252_X8: [char; 32] = [
    '\u{208c}', '\u{81}', '\u{201a}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
    '\u{02C6}', '\u{2030}', '\u{0160}', '\u{2039}', '\u{0152}', '\u{8D}', '\u{017D}', '\u{8F}',
    '\u{90}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
    '\u{02DC}', '\u{2122}', '\u{0161}', '\u{203A}', '\u{0153}', '\u{9D}', '\u{017E}', '\u{0178}',
];

#[inline]
fn win1252_char_decode(chr: u8) -> char {
    if chr & 0xE0 == 0x80 {
        unsafe { *W1252_X8.get_unchecked(chr as usize - 0x80) }
    } else {
        char::from(chr)
    }
}

pub struct Latin1Decoded<'a>(pub &'a [u8]);

impl<'a> fmt::Display for Latin1Decoded<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &chr in self.0 {
            f.write_char(win1252_char_decode(chr))?;
        }
        Ok(())
    }
}

impl<'a> Decoder<'a> for Latin1Decoder<'a> {
    type Slice = [u8];

    fn as_slice(&self) -> &'a Self::Slice {
        self.rest
    }

    fn as_bytes(&self) -> &'a [u8] {
        self.rest
    }

    fn skip_bytes(&mut self, count: usize) {
        self.rest = self.rest.split_at(count).1;
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some((&chr, rest)) = self.rest.split_first() {
            self.rest = rest;
            Some(win1252_char_decode(chr))
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.rest
            .split_first()
            .map(|(&c, _)| win1252_char_decode(c))
    }
}

#[derive(Clone)]
pub struct Utf8Decoder<'a> {
    inner: Chars<'a>,
}

impl<'a> Utf8Decoder<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: input.chars(),
        }
    }
}

impl<'a> Decoder<'a> for Utf8Decoder<'a> {
    type Slice = str;

    fn as_bytes(&self) -> &'a [u8] {
        self.inner.as_str().as_bytes()
    }

    fn as_slice(&self) -> &'a Self::Slice {
        self.inner.as_str()
    }

    // This may panic if count isn't within bounds
    fn skip_bytes(&mut self, count: usize) {
        self.inner = self.inner.as_str().split_at(count).1.chars();
    }

    fn next_char(&mut self) -> Option<char> {
        self.inner.next()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.inner.clone().next()
    }
}

#[cfg(test)]
mod tests {
    use crate::encoding::win1252_char_decode;

    #[test]
    fn test_win1252_decode() {
        // Just check that nothing segfaults
        for chr in 0u8..=255 {
            super::win1252_char_decode(chr);
        }

        assert_eq!(win1252_char_decode(0x8b), '‹');
        assert_eq!(win1252_char_decode(0x9b), '›');
        assert_eq!(win1252_char_decode(0xfb), 'û');
    }
}
impl<'a> Latin1Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { rest: bytes }
    }
}
