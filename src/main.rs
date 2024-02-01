use std::{env, error::Error, fs, io::Write};

use arboard::Clipboard;

mod config;

use config::extract_config;

fn main() {
    let (config, files) = extract_config(get_args());

    if files.is_empty() {
        return;
    }

    let merged_content = files
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));

    copy_to_clipboard(&merged_content).expect("Failed to copy contents to the clipboard");

    if config.pause() {
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
fn copy_to_clipboard(contents: &str) -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(contents)?;

    Ok(())
}
