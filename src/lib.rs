// Project: selection
// File: lib.rs
// Created Date: 2023-06-04
// Author: Pylogmon <pylogmon@outlook.com>

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(test)]
mod tests {
    #[cfg(target_os = "linux")]
    use crate::linux::get_text;
    #[cfg(target_os = "macos")]
    use crate::macos::get_text;
    #[cfg(target_os = "windows")]
    use crate::windows::get_text;

    #[test]
    fn it_works() {
        println!("{}", get_text());
    }
}
