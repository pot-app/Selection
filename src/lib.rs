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

#[cfg(target_os = "linux")]
use crate::linux::get_text as _get_text;
#[cfg(target_os = "macos")]
use crate::macos::get_text as _get_text;
#[cfg(target_os = "windows")]
use crate::windows::get_text as _get_text;

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
pub fn get_text() -> String {
    _get_text().trim().to_owned()
}

#[cfg(test)]
mod tests {
    use crate::get_text;
    #[test]
    fn it_works() {
        println!("{}", get_text());
    }
}
