// Prevents the terminal window from opening on Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import the necessary modules from external crates
use eframe::egui::{self, Response, Ui}; // GUI library for building applications
use sha2::{Sha256, Digest}; // SHA-256 hashing algorithm
use clipboard::{ClipboardContext, ClipboardProvider}; // Clipboard handling for copy-paste functionality

// Function to validate and parse the depth input as an integer
fn validate_depth(arg_two: &str) -> i32 {
    // Try to parse the input as an integer
    if let Ok(n) = arg_two.parse::<i32>() {
        return n;
    }
    // If parsing as an integer fails, try to parse as a float and round it to the nearest integer
    if let Ok(test_float) = arg_two.parse::<f64>() {
        return test_float.round() as i32;
    }
    // Return 0 if both parsing attempts fail
    0
}

// Function to perform SHA-256 hashing on the input string
fn crunch(mstr: &str) -> String {
    // Create a new SHA-256 hasher
    let mut hasher = Sha256::new();
    // Update the hasher with the input string after trimming whitespace
    hasher.update(mstr.trim().as_bytes());
    // Finalize the hash and format it as a hexadecimal string
    let result = hasher.finalize();
    format!("{:x}", result)
}

// Function to copy the given content to the system clipboard
fn copy_to_clipboard(content: &str) {
    // Create a new clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Failed to create clipboard context");
    // Set the clipboard contents to the given string
    ctx.set_contents(content.to_owned()).expect("Failed to set clipboard contents");
}

// Define a struct to hold the application's state
struct MyApp {
    text_input: String, // User input for the text to be hashed
    depth_input: String, // User input for the hashing depth
    show_message: bool, // Flag to indicate whether to show the clipboard message
}

// Implement the Default trait for MyApp to provide default values for the fields
impl Default for MyApp {
    fn default() -> Self {
        Self {
            text_input: String::new(), // Initialize text input as an empty string
            depth_input: String::new(), // Initialize depth input as an empty string
            show_message: false, // Initialize show_message as false
        }
    }
}

// Implement the eframe::App trait for MyApp to define the application's behavior
impl eframe::App for MyApp {
    // Function to update the application's state and handle UI interactions
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Customize the style for the UI context
        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 120, 215); // Button background color when pressed
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(0, 120, 215); // Button background color when inactive
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 120, 215); // Button background color when hovered
        style.visuals.widgets.active.fg_stroke.color = egui::Color32::WHITE; // Button text color when pressed
        style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::WHITE; // Button text color when inactive
        style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::WHITE; // Button text color when hovered
        ctx.set_style(style);

        // Create the central panel for the GUI
        egui::CentralPanel::default().show(ctx, |ui| {
            // Display the application's heading
            ui.heading("Mastergen - A SHA256 Master Password Generator");

            // Add some vertical spacing for better layout
            ui.add_space(10.0);

            // Create a vertical layout for the input fields and button
            ui.vertical(|ui| {
                // Create a horizontal layout for the text input field
                ui.horizontal(|ui| {
                    ui.label("Text:"); // Label for the text input
                    password_input(ui, &mut self.text_input); // Text input field as password
                });

                // Add some vertical spacing for better layout
                ui.add_space(10.0);

                // Create a horizontal layout for the depth input field
                ui.horizontal(|ui| {
                    ui.label("Depth:"); // Label for the depth input
                    password_input(ui, &mut self.depth_input); // Depth input field as password
                });

                // Add some vertical spacing for better layout
                ui.add_space(10.0);

                // Create a button to execute the hashing operation
                if ui.add_sized([100.0, 40.0], egui::Button::new("Execute")).clicked() {
                    // Validate the depth input
                    let depth = validate_depth(&self.depth_input);
                    // Check if the depth is greater than 0
                    if depth > 0 {
                        // Clone the text input to perform hashing
                        let mut mstr = self.text_input.clone();
                        // Perform the hashing operation 'depth' number of times
                        for _ in 1..=depth {
                            mstr = crunch(&mstr);
                        }
                        // Copy the result to the clipboard
                        copy_to_clipboard(&mstr);
                        // Set the flag to show the clipboard message
                        self.show_message = true;
                    } else {
                        // Display an error message if the depth is invalid
                        ui.label("Invalid depth supplied!");
                        // Reset the flag to hide the clipboard message
                        self.show_message = false;
                    }
                }

                // Add some vertical spacing for better layout
                ui.add_space(10.0);

                // Conditionally display the clipboard message
                if self.show_message {
                    ui.label("Result has been copied to clipboard.");
                }
            });
        });
    }
}

// Function to create a masked input field
fn password_input(ui: &mut Ui, input: &mut String) -> Response {
    let mut display_text = "*".repeat(input.len());
    let response = ui.text_edit_singleline(&mut display_text);
    if response.changed() {
        let len = display_text.len();
        input.clear();
        input.push_str(&"*".repeat(len));
    }
    response
}

// Main function to set up and run the application
fn main() {
    // Create a new instance of the MyApp struct with default values
    let app = MyApp::default();
    // Define options for the native window
    let native_options = eframe::NativeOptions {
        decorated: true, // Use standard window decorations
        initial_window_size: Some(egui::Vec2::new(400.0, 300.0)), // Set initial window size
        ..Default::default() // Use default options for the rest
    };
    // Run the application with the given title, options, and app instance
    eframe::run_native(
        "Mastergen64", // Title of the window
        native_options, // Options for the native window
        Box::new(|_cc| Box::new(app)), // Closure to create the app instance
    );
}
