use std::{env, error::Error, fs, io::Write};

use arboard::Clipboard;

mod flags;

use flags::extract_flags;

fn main() {
    let (flags, files) = extract_flags(get_args());

    if files.is_empty() {
        return;
    }

    let merged_content = files
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));

    copy_to_clipboard(&merged_content).expect("Failed to copy contents to the clipboard");

    if flags.verbouse() {
        println!("{}", merged_content)
    }

    if flags.pause() {
        print!("Pausing execution, press enter to exit! ");
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

    args[1..].to_vec()
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
