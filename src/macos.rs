use accessibility_ng::{AXAttribute, AXUIElement};
use accessibility_sys_ng::{kAXFocusedUIElementAttribute, kAXSelectedTextAttribute};
use core_foundation::string::CFString;
use log::{error, info};
use std::error::Error;

pub fn get_text() -> String {
    match get_selected_text_by_ax() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            } else {
                info!("get_selected_text_by_ax is empty");
            }
        }
        Err(err) => {
            error!("get_selected_text_by_ax error:{}", err);
        }
    }
    info!("fallback to get_text_by_clipboard");
    match get_text_by_clipboard() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            } else {
                info!("get_text_by_clipboard is empty");
            }
        }
        Err(err) => {
            error!("get_text_by_clipboard error:{}", err);
        }
    }
    // Return Empty String
    String::new()
}

// Copy from https://github.com/yetone/get-selected-text/blob/main/src/macos.rs
fn get_selected_text_by_ax() -> Result<String, Box<dyn Error>> {
    let system_element = AXUIElement::system_wide();
    let Some(selected_element) = system_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXFocusedUIElementAttribute,
        )))
        .map(|element| element.downcast_into::<AXUIElement>())
        .ok()
        .flatten()
    else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No selected element",
        )));
    };
    let Some(selected_text) = selected_element
        .attribute(&AXAttribute::new(&CFString::from_static_string(
            kAXSelectedTextAttribute,
        )))
        .map(|text| text.downcast_into::<CFString>())
        .ok()
        .flatten()
    else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No selected text",
        )));
    };
    Ok(selected_text.to_string())
}

fn get_text_by_clipboard() -> Result<String, Box<dyn Error>> {
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(APPLE_SCRIPT)
        .output()?;
    // check exit code
    if output.status.success() {
        // get output content
        let content = String::from_utf8(output.stdout)?;
        Ok(content)
    } else {
        Err(format!("{output:?}").into())
    }
}

const APPLE_SCRIPT: &str = r#"
use AppleScript version "2.4"
use scripting additions
use framework "Foundation"
use framework "AppKit"

set savedAlertVolume to alert volume of (get volume settings)

-- Back up clipboard contents:
set savedClipboard to the clipboard

set thePasteboard to current application's NSPasteboard's generalPasteboard()
set theCount to thePasteboard's changeCount()

tell application "System Events"
    set volume alert volume 0
end tell

-- Copy selected text to clipboard:
tell application "System Events" to keystroke "c" using {command down}
delay 0.1 -- Without this, the clipboard may have stale data.

tell application "System Events"
    set volume alert volume savedAlertVolume
end tell

if thePasteboard's changeCount() is theCount then
    return ""
end if

set theSelectedText to the clipboard

set the clipboard to savedClipboard

theSelectedText
"#;
