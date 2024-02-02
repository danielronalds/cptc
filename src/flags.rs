// Arrays of the different flag options
const PAUSE_FLAGS: [&str; 2] = ["--pause", "-p"];
const VERBOUSE_FLAGS: [&str; 2] = ["--verbouse", "-v"];
const HELP_FLAGS: [&str; 2] = ["--help", "-h"];
const VERSION_FLAGS: [&str; 2] = ["--version", "-V"];

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// A struct for storing the flags the program is run with
pub struct Flags {
    pause: bool,
    verbouse: bool,
    help: bool,
    version: bool,
}

impl Flags {
    /// Whether the program should pause before closing
    pub fn pause(&self) -> bool {
        self.pause
    }

    /// Whether the program should print the contents of the clipboard
    pub fn verbouse(&self) -> bool {
        self.verbouse
    }

    /// Whether the program should print the help menu and exit
    pub fn help(&self) -> bool {
        self.help
    }

    /// Whether the program should print the version and exit
    pub fn version(&self) -> bool {
        self.version
    }
}

impl From<Vec<String>> for Flags {
    fn from(value: Vec<String>) -> Self {
        let mut flags = Flags::default();

        for flag in value {
            flags.pause = PAUSE_FLAGS.contains(&flag.as_str());
            flags.verbouse = VERBOUSE_FLAGS.contains(&flag.as_str());
            flags.help = HELP_FLAGS.contains(&flag.as_str());
            flags.version = VERSION_FLAGS.contains(&flag.as_str());
        }

        flags
    }
}

/// Extracts the flags the program is run with from files
///
/// # Parameters
///
/// - `args` The vec of strings to extract flags out of
///
/// # Returns
///
/// A tuple with a `Flags` struct, and the remaining strings
pub fn extract_flags(args: Vec<String>) -> (Flags, Vec<String>) {
    let (flags, files) = split_arguments(args);

    (Flags::from(flags), files)
}

/// Splits the given strings into flags and files
///
/// # Parameters
///
/// - `args` The vec of strings to split
///
/// # Returns
///
/// A tuple of two vecs of strings, with the first being flags
fn split_arguments(args: Vec<String>) -> (Vec<String>, Vec<String>) {
    let is_flag = |x: &str| x.starts_with('-');

    let flags: Vec<String> = args
        .iter()
        .filter(|x| is_flag(x))
        .map(|x| x.to_string())
        .collect();

    let files: Vec<String> = args
        .iter()
        .filter(|x| !is_flag(x))
        .map(|x| x.to_string())
        .collect();

    (flags, files)
}

#[cfg(test)]
mod tests {
    use crate::flags::{extract_flags, split_arguments, Flags};

    #[test]
    fn split_arguments_works() {
        let args: Vec<String> = ["test.txt", "test1.txt", "test2.txt", "-p"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let (flags, _) = split_arguments(args);

        assert_eq!(flags, vec!["-p".to_string()]);
    }

    #[test]
    fn extract_flags_with_flags_works() {
        let args: Vec<String> = ["test.txt", "test1.txt", "test2.txt", "-p"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();

        let (flags, files) = extract_flags(args);

        assert_eq!(
            flags,
            Flags {
                pause: true,
                verbouse: false,
                help: false,
                version: false
            }
        );

        assert_eq!(
            files,
            ["test.txt", "test1.txt", "test2.txt"]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        )
    }
}
