use std::{env, error::Error, fs};

use arboard::Clipboard;

fn main() {
    let files = get_args();

    if files.is_empty() {
        return;
    }

    let merged_content = files
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));

    copy_to_clipboard(&merged_content).expect("Failed to copy contents to the clipboard");
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
fn copy_to_clipboard(contents: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(contents)?;

    Ok(())
}
