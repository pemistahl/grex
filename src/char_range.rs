/// A lightweight replacement for unic_char_range::CharRange
/// Represents a closed range of Unicode characters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharRange {
    start: char,
    end: char,
}

impl CharRange {
    /// Creates a closed character range from start to end (inclusive)
    pub fn closed(start: char, end: char) -> Self {
        Self { start, end }
    }

    /// Checks if the given character is within this range
    pub fn contains(&self, c: char) -> bool {
        c >= self.start && c <= self.end
    }

    /// Returns an iterator over all valid Unicode scalar values
    /// This includes U+0000 to U+D7FF and U+E000 to U+10FFFF
    /// (excludes surrogate code points U+D800 to U+DFFF)
    pub fn all() -> CharRangeIter {
        CharRangeIter {
            current: '\0',
            done: false,
        }
    }
}

/// Iterator over all valid Unicode scalar values
pub struct CharRangeIter {
    current: char,
    done: bool,
}

impl Iterator for CharRangeIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;
        
        // Get the next valid Unicode scalar value
        let mut next_code_point = self.current as u32 + 1;
        
        // Skip over surrogate code points (U+D800 to U+DFFF) and find next valid char
        loop {
            if next_code_point > 0x10FFFF {
                // We've reached the end of valid Unicode code points
                self.done = true;
                break;
            }
            
            match char::from_u32(next_code_point) {
                Some(next_char) => {
                    self.current = next_char;
                    break;
                }
                None => {
                    // Invalid code point (likely surrogate), skip to next
                    next_code_point += 1;
                }
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_range_contains() {
        let range = CharRange::closed('a', 'z');
        assert!(range.contains('a'));
        assert!(range.contains('m'));
        assert!(range.contains('z'));
        assert!(!range.contains('A'));
        assert!(!range.contains('0'));
    }

    #[test]
    fn test_char_range_all() {
        let all_chars: Vec<char> = CharRange::all().take(10).collect();
        assert_eq!(all_chars[0], '\0');
        assert_eq!(all_chars.len(), 10);
    }

    #[test]
    fn test_char_range_all_count() {
        // Valid Unicode scalar values: 0x110000 total code points - 0x800 surrogates = 0x10F800
        let count = CharRange::all().count();
        assert_eq!(count, 0x10F800);
    }
}

