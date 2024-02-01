/// An array of the pause flags options
const PAUSE_OPTIONS: [&str; 2] = ["--pause", "-p"];

/// An array of the different flags
const CONFIG_OPTIONS: [[&str; 2]; 1] = [PAUSE_OPTIONS];

#[derive(Default)]
/// A struct for storing the flags the program is run with
pub struct Flags {
    pause: bool,
}

impl Flags {
    /// Whether the program should pause before closing
    pub fn pause(&self) -> bool {
        self.pause
    }
}

impl From<Vec<String>> for Flags {
    fn from(value: Vec<String>) -> Self {
        let mut config = Flags::default();

        for option in value {
            if PAUSE_OPTIONS.contains(&option.as_str()) {
                config.pause = true;
            }
        }

        config
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
/// A tuple with a `Flags` struct, and the remaing strings
pub fn extract_flags(args: Vec<String>) -> (Flags, Vec<String>) {
    let config_options: Vec<String> = args
        .iter()
        .filter_map(|x| {
            for options in CONFIG_OPTIONS {
                if options.contains(&x.as_str()) {
                    return Some(x.to_string());
                }
            }
            None
        })
        .collect();

    let files: Vec<String> = args
        .iter()
        .filter_map(|x| {
            for options in CONFIG_OPTIONS {
                if options.contains(&x.as_str()) {
                    return None;
                }
            }
            Some(x.to_string())
        })
        .collect();

    (Flags::from(config_options), files)
}
