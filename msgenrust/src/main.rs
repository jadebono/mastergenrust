// Import necessary crates and modules
use sha2::{Sha256, Digest};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// Function to validate and convert the input depth to an integer
fn validate_depth(arg: &str) -> i32 {
    // Try to parse the argument as an integer
    if let Ok(n) = arg.parse::<i32>() {
        return n;
    }
    // If that fails, try to parse it as a float and round to the nearest integer
    if let Ok(test_float) = arg.parse::<f64>() {
        return test_float.round() as i32;
    }
    // If both parsing attempts fail, return 0
    0
}

// Function to hash the input string using SHA-256
fn crunch(mstr: &str) -> String {
    let mut hasher = Sha256::new();
    // Remove any leading/trailing whitespace and update the hasher
    hasher.update(mstr.trim().as_bytes());
    // Finalize the hash and format it as a hexadecimal string
    let result = hasher.finalize();
    format!("{:x}", result)
}

// Function to copy the given content to the clipboard using xclip
fn copy_to_clipboard(content: &str) {
    // Create a new xclip command to copy to the clipboard
    let mut cmd = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start xclip");

    {
        // Open the stdin of the xclip command and write the content to it
        let stdin = cmd.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(content.as_bytes()).expect("Failed to write to stdin");
    }

    // Wait for the xclip command to complete
    let output = cmd.wait_with_output().expect("Failed to wait on xclip");
    // If the command did not succeed, print an error message
    if !output.status.success() {
        eprintln!("xclip command failed with status: {}", output.status);
    }
}

// Function to read input and display stars
fn read_input_with_stars(prompt: &str) -> String {
    // Print the prompt and flush stdout
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    // Enter raw mode to handle keypresses manually
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut input = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('\n') => break, // Enter key ends the input
            termion::event::Key::Char(c) => {
                input.push(c);
                write!(stdout, "*").unwrap(); // Print a star for each character
            }
            termion::event::Key::Backspace => {
                input.pop();
                write!(stdout, "\x08 \x08").unwrap(); // Handle backspace
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
    println!(); // Move to the next line after input is complete
    input
}

// Main function to execute the program
fn main() {
    // Print a message and wait for the user to input the seed phrase and depth
    println!("Please enter the seed phrase:");
    let phrase = read_input_with_stars("> ");
    println!("Please enter the depth:");
    let depth_str = read_input_with_stars("> ");
    let depth = validate_depth(&depth_str);

    // Validate the depth input
    if depth <= 0 {
        println!("Invalid depth supplied! Program will terminate here!");
        return;
    }

    // Hash the seed phrase n times
    let mut hashed_phrase = phrase;
    for _ in 1..=depth {
        hashed_phrase = crunch(&hashed_phrase);
    }

    // Copy the final hash to the clipboard
    copy_to_clipboard(&hashed_phrase);

    // Print a confirmation message to the terminal
    println!("Master password generated and copied to clipboard.");
}
