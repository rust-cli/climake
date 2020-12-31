//! Input and output structures for climake which allow inputs for args along
//! with outputs passed back
//!
//! # Importing
//!
//! This module is included in [crate::prelude] by default so no extra importing
//! steps are required (unless you are importing explicit items).

use std::fmt;
use std::path::PathBuf;

/// An input type, typically given for an [Argument](crate::Argument) to descibe
/// what types are allowed to be passwed in. This is then transferred to [Data]
/// once the cli has been executed
#[derive(Debug, PartialEq, Clone)]
pub enum Input {
    /// No input allowed, will error if any is given. Maps to [Data::None]
    None,

    /// Text input allowed, this will return an empty string if no text is supplied.
    /// Maps to [Data::Text]
    Text,

    /// A single [PathBuf] given to the argument, these are not certain to exist
    /// and simply echo the user's input. Maps to [Data::Path]
    Path,

    /// Multiple [PathBuf]s given to the argument, these are not certain to exist
    /// and simply echo the user's input. Maps to [Data::Paths]
    Paths,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // formatting has a space on existing words on purpouse for help generation
        match self {
            Input::None => write!(f, ""),
            Input::Text => write!(f, "[text] "),
            Input::Path => write!(f, "[path] "),
            Input::Paths => write!(f, "[paths] "),
        }
    }
}

/// Outputted data from parsing a cli for each argument. This enumeration is based
/// upon the allowed [Input] of a given [Argument](crate::Argument) and maps
/// directly to the input
///
/// # Mappings from [Input]
///
/// If a user requested for an [Argument](crate::Argument) to be of [Input::Path],
/// once parsed this enumeration would be [Data::Path] (in corrospondance with
/// the name).
///
#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    /// No data provided, from [Input::None]
    None,

    /// Textual input provided, from [Input::Text]. This may be an empty string
    /// in the case of the user not actually providing input
    Text(String),

    /// Path input provided, from [Input::Path]. This may be an empty or invalid
    /// [PathBuf] in the case of user input being misleading or non-existant
    Path(PathBuf),

    /// Multiple path inputs provided, from [Input::Paths]. This may be an empty
    /// vector (i.e. length 0) if the user doesn't provide any paths or may be
    /// non-existant paths given from user input
    Paths(Vec<PathBuf>),
}

impl Data {
    /// Creates a new [Data] from with types mapping from [Input] using passed
    /// `data`. This may map the `data` string vec into types such as `PathBuf`
    pub(crate) fn new(input: Input, data: impl IntoIterator<Item = String>) -> Self {
        match input {
            Input::None => Data::None, // ignore passed `data` (if any)
            Input::Text => match data.into_iter().next() {
                Some(text) => Data::Text(text),
                None => Data::Text(String::new()),
            },
            Input::Path => match data.into_iter().next() {
                Some(path_string) => Data::Path(PathBuf::from(path_string)),
                None => Data::Path(PathBuf::new()),
            },
            Input::Paths => Data::Paths(
                data.into_iter()
                    .map(|path_string| PathBuf::from(path_string))
                    .collect(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the [Data::new] method works correctly
    #[test]
    fn data_new() {
        let testval = String::from("Hi!");

        // Data::None
        assert_eq!(Data::new(Input::None, vec![]), Data::None);
        assert_eq!(Data::new(Input::None, vec![testval.clone()]), Data::None);

        // Data::Text
        assert_eq!(Data::new(Input::Text, vec![]), Data::Text(String::new()));
        assert_eq!(
            Data::new(Input::Text, vec![testval.clone()]),
            Data::Text(testval.clone())
        );
        assert_eq!(
            Data::new(Input::Text, vec![testval.clone(), testval.clone()]),
            Data::Text(testval.clone())
        );

        // Data::Path
        assert_eq!(Data::new(Input::Path, vec![]), Data::Path(PathBuf::new()));
        assert_eq!(
            Data::new(Input::Path, vec![testval.clone()]),
            Data::Path(PathBuf::from(testval.clone()))
        );
        assert_eq!(
            Data::new(Input::Path, vec![testval.clone(), testval.clone()]),
            Data::Path(PathBuf::from(testval.clone()))
        );

        // Data::Paths
        assert_eq!(Data::new(Input::Paths, vec![]), Data::Paths(vec![]));
        assert_eq!(
            Data::new(Input::Paths, vec![testval.clone()]),
            Data::Paths(vec![PathBuf::from(testval.clone())])
        );
        assert_eq!(
            Data::new(Input::Paths, vec![testval.clone(), testval.clone()]),
            Data::Paths(vec![PathBuf::from(testval.clone()), PathBuf::from(testval)])
        );
    }
}
