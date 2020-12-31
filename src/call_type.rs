//! Contains [CallType]-related items, see specific documentation for more information

use std::fmt;

/// A single type of call for an [Argument], can be a short call or a long call
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CallType {
    /// Short, single-char call, e.g. `-h`
    Short(char),

    /// Long, multi-char call, e.g. `--hello`
    Long(String),
}

impl fmt::Display for CallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallType::Short(c) => write!(f, "{}", c),
            CallType::Long(string) => write!(f, "--{}", string),
        }
    }
}

impl From<CallType> for String {
    fn from(calltype: CallType) -> Self {
        match calltype {
            CallType::Short(c) => String::from(c),
            CallType::Long(string) => string,
        }
    }
}

impl From<char> for CallType {
    fn from(c: char) -> Self {
        CallType::Short(c)
    }
}

impl From<String> for CallType {
    fn from(string: String) -> Self {
        CallType::Long(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [From]<[CallType]> implementation for [String] works correctly
    #[test]
    fn string_from_calltype() {
        assert_eq!(String::from(CallType::Short('h')), "h".to_string());
        assert_eq!(
            String::from(CallType::Long("testing".to_string())),
            "testing".to_string()
        );
    }
}
