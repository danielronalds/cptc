use std::{
    env,
    error::Error,
    fs,
    io::{self, IsTerminal, Write},
};

use arboard::Clipboard;

mod flags;

use flags::extract_flags;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let (flags, files) = extract_flags(get_args());

    if flags.version() {
        println!("{} {}", NAME, VERSION);
        return;
    }

    let piped_input = get_piped_input();

    if flags.help() || files.is_empty() && piped_input.is_none() {
        print_help();
        return;
    }

    let mut merged_content = files
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .fold("".to_string(), |acc, x| {
            format!("{}{}{}", acc, if !acc.is_empty() { "\n" } else { "" }, x)
        });

    if let Some(piped_input) = piped_input {
        merged_content = format!(
            "{}{}{}",
            piped_input,
            if merged_content.is_empty() { "" } else { "\n" },
            merged_content
        );
    }

    copy_to_clipboard(&merged_content).expect("Failed to copy contents to the clipboard");

    if flags.verbouse() {
        println!("{}", merged_content)
    }

    if flags.pause() || is_wayland() {
        print!("[cptc] Pausing execution, press enter to exit! ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

/// Gets the args passed to the program
///
/// # Returns
///
/// The env args from index 1 onwards
fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    args[1..].to_vec() // 1st arg is not required
}

/// Gets the piped in content as a String
///
/// # Returns
///
/// Some(piped_input) if there is any input, otherwise None
fn get_piped_input() -> Option<String> {
    let stdin = io::stdin();

    if stdin.is_terminal() {
        return None;
    }

    let piped_input = io::read_to_string(stdin).ok()?;

    match piped_input.is_empty() {
        true => None,
        false => Some(piped_input),
    }
}

/// Determines if the user is using wayland or not
fn is_wayland() -> bool {
    let xdg_session = env::var("XDG_SESSION_TYPE");

    match xdg_session {
        Ok(value) => value == "wayland",
        Err(_) => false,
    }
}

/// Sets the given string to the user's clipboard
///
/// # Parameters
///
/// - `content` The content to set the clipboard to contain
///
/// # Returns
///
/// Whether the clipboard was successful set or not
fn copy_to_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(content)?;

    Ok(())
}

/// Prints the help menu
fn print_help() {
    let help = format!(
        "\
usage: {} [OPTIONS] [FILE]...

{}

Options:
  --help       -h      Print this menu and exit
  --version    -v      Print the version number and exit
  --verbouse   -v      Print what is copied
  --pause      -p      Pauses exiting of the program until ENTER is pressed",
        NAME, DESCRIPTION
    );

    println!("{}", help);
}
