// Project: selection
// File: lib.rs
// Created Date: 2023-06-04
// Author: Pylogmon <pylogmon@outlook.com>

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

/// Get the text selected by the cursor
///
/// Return empty string if no text is selected or error occurred
/// # Example
///
/// ```
/// use selection::get_text;
///
/// let text = get_text();
/// println!("{}", text);
/// ```
#[cfg(target_os = "linux")]
pub use crate::linux::get_text;
#[cfg(target_os = "macos")]
pub use crate::macos::get_text;
#[cfg(target_os = "windows")]
pub use crate::windows::get_text;

#[cfg(test)]
mod tests {
    use crate::get_text;
    #[test]
    fn it_works() {
        println!("{}", get_text());
    }
}
