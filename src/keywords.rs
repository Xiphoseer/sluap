/// Reserved Identifiers
///
/// These identifiers have a special syntactic meaning.
///
/// The [Ord] implementation sorts keywords lexicographically.
///
/// See: <https://www.lua.org/manual/5.1/manual.html#2.1>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    /// `and`
    And,
    /// `break`
    Break,
    /// `do`
    Do,
    /// `else`
    Else,
    /// `elseif`
    ElseIf,
    /// `end`
    End,
    /// `false`
    False,
    /// `for`
    For,
    /// `function`
    Function,
    /// `if`
    If,
    /// `in`
    In,
    /// `local`
    Local,
    /// `nil`
    Nil,
    /// `not`
    Not,
    /// `or`
    Or,
    /// `repeat`
    Repeat,
    /// `return`
    Return,
    /// `then`
    Then,
    /// `true`
    True,
    /// `until`
    Until,
    /// `while`
    While,
}

impl Keyword {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            b"and" => Some(Self::And),
            b"break" => Some(Self::Break),
            b"do" => Some(Self::Do),
            b"else" => Some(Self::Else),
            b"elseif" => Some(Self::ElseIf),
            b"end" => Some(Self::End),
            b"false" => Some(Self::False),
            b"for" => Some(Self::For),
            b"function" => Some(Self::Function),
            b"if" => Some(Self::If),
            b"in" => Some(Self::In),
            b"local" => Some(Self::Local),
            b"nil" => Some(Self::Nil),
            b"not" => Some(Self::Not),
            b"or" => Some(Self::Or),
            b"repeat" => Some(Self::Repeat),
            b"return" => Some(Self::Return),
            b"then" => Some(Self::Then),
            b"true" => Some(Self::True),
            b"until" => Some(Self::Until),
            b"while" => Some(Self::While),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Keyword;

    #[test]
    fn test_from_bytes() {
        assert_eq!(Keyword::from_bytes(b"and"), Some(Keyword::And));
        assert_eq!(Keyword::from_bytes(b"break"), Some(Keyword::Break));
        assert_eq!(Keyword::from_bytes(b"do"), Some(Keyword::Do));
        assert_eq!(Keyword::from_bytes(b"else"), Some(Keyword::Else));
        assert_eq!(Keyword::from_bytes(b"elseif"), Some(Keyword::ElseIf));
        assert_eq!(Keyword::from_bytes(b"end"), Some(Keyword::End));
        assert_eq!(Keyword::from_bytes(b"false"), Some(Keyword::False));
        assert_eq!(Keyword::from_bytes(b"for"), Some(Keyword::For));
        assert_eq!(Keyword::from_bytes(b"function"), Some(Keyword::Function));
        assert_eq!(Keyword::from_bytes(b"if"), Some(Keyword::If));
        assert_eq!(Keyword::from_bytes(b"in"), Some(Keyword::In));
        assert_eq!(Keyword::from_bytes(b"local"), Some(Keyword::Local));
        assert_eq!(Keyword::from_bytes(b"nil"), Some(Keyword::Nil));
        assert_eq!(Keyword::from_bytes(b"not"), Some(Keyword::Not));
        assert_eq!(Keyword::from_bytes(b"or"), Some(Keyword::Or));
        assert_eq!(Keyword::from_bytes(b"repeat"), Some(Keyword::Repeat));
        assert_eq!(Keyword::from_bytes(b"return"), Some(Keyword::Return));
        assert_eq!(Keyword::from_bytes(b"then"), Some(Keyword::Then));
        assert_eq!(Keyword::from_bytes(b"true"), Some(Keyword::True));
        assert_eq!(Keyword::from_bytes(b"until"), Some(Keyword::Until));
        assert_eq!(Keyword::from_bytes(b"while"), Some(Keyword::While));
    }
}
