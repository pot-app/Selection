use arboard::Clipboard;
use log::{error, info};
use rdev::{simulate, EventType, Key};
use std::error::Error;
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL};
use windows::Win32::System::DataExchange::GetClipboardSequenceNumber;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
};
pub fn get_text() -> String {
    match get_text_by_automation() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            } else {
                info!("get_text_by_automation is empty");
            }
        }
        Err(err) => {
            error!("get_text_by_automation error:{}", err);
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
            error!("get_text_by_automation error:{}", err);
        }
    }
    // Return Empty String
    String::new()
}

// Available for Edge, Chrome and UWP
fn get_text_by_automation() -> Result<String, Box<dyn Error>> {
    // Init COM
    unsafe { CoInitialize(None) }.ok()?;
    // Create IUIAutomation instance
    let auto: IUIAutomation = unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL) }?;
    // Get Focused Element
    let el = unsafe { auto.GetFocusedElement() }?;
    // Get TextPattern
    let res: IUIAutomationTextPattern = unsafe { el.GetCurrentPatternAs(UIA_TextPatternId) }?;
    // Get TextRange Array
    let text_array = unsafe { res.GetSelection() }?;
    let length = unsafe { text_array.Length() }?;
    // Iterate TextRange Array
    let mut target = String::new();
    for i in 0..length {
        let text = unsafe { text_array.GetElement(i) }?;
        let str = unsafe { text.GetText(-1) }?;
        let str = str.to_string();
        target.push_str(&str);
    }
    Ok(target.trim().to_string())
}

// Available for almost all applications
fn get_text_by_clipboard() -> Result<String, Box<dyn Error>> {
    // Read Old Clipboard
    let old_clipboard = (Clipboard::new()?.get_text(), Clipboard::new()?.get_image());

    if let Ok(true) = copy() {
        // Read New Clipboard
        let new_text = Clipboard::new()?.get_text();

        // Create Write Clipboard
        let mut write_clipboard = Clipboard::new()?;

        match old_clipboard {
            (Ok(text), _) => {
                // Old Clipboard is Text
                write_clipboard.set_text(text)?;
                if let Ok(new) = new_text {
                    Ok(new.trim().to_string())
                } else {
                    Err("New clipboard is not Text".into())
                }
            }
            (_, Ok(image)) => {
                // Old Clipboard is Image
                write_clipboard.set_image(image)?;
                if let Ok(new) = new_text {
                    Ok(new.trim().to_string())
                } else {
                    Err("New clipboard is not Text".into())
                }
            }
            _ => {
                // Old Clipboard is Empty
                write_clipboard.clear()?;
                if let Ok(new) = new_text {
                    Ok(new.trim().to_string())
                } else {
                    Err("New clipboard is not Text".into())
                }
            }
        }
    } else {
        Err("Copy Failed".into())
    }
}

fn copy() -> Result<bool, Box<dyn Error>> {
    let num_before = unsafe { GetClipboardSequenceNumber() };
    simulate(&EventType::KeyPress(Key::ControlRight))?;
    simulate(&EventType::KeyPress(Key::KeyC))?;
    simulate(&EventType::KeyRelease(Key::ControlRight))?;
    simulate(&EventType::KeyRelease(Key::KeyC))?;
    std::thread::sleep(std::time::Duration::from_millis(20));
    let num_after = unsafe { GetClipboardSequenceNumber() };
    Ok(num_after != num_before)
}
