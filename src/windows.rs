pub fn get_text() -> String {
    match get_text_by_automation() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            }
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
    match get_text_by_clipboard() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            }
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
    // Return Empty String
    String::new()
}

// Available for Edge, Chrome and UWP
fn get_text_by_automation() -> Result<String, String> {
    use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL};
    use windows::Win32::UI::Accessibility::{
        CUIAutomation, IUIAutomation, IUIAutomationTextPattern, UIA_TextPatternId,
    };
    // Init COM
    match unsafe { CoInitialize(None) } {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    };
    // Create IUIAutomation instance
    let auto: IUIAutomation = match unsafe { CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL) } {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    // Get Focused Element
    let el = match unsafe { auto.GetFocusedElement() } {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    // Get TextPattern
    let res: IUIAutomationTextPattern = match unsafe { el.GetCurrentPatternAs(UIA_TextPatternId) } {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    // Get TextRange Array
    let text_array = match unsafe { res.GetSelection() } {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    let length = match unsafe { text_array.Length() } {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    // Iterate TextRange Array
    let mut target = String::new();
    for i in 0..length {
        let text = match unsafe { text_array.GetElement(i) } {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let str = match unsafe { text.GetText(-1) } {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let str = str.to_string();
        target.push_str(&str);
    }
    Ok(target)
}

// Available for almost all applications
fn get_text_by_clipboard() -> Result<String, String> {
    use arboard::Clipboard;

    // Read Old Clipboard
    let old_clipboard = (
        Clipboard::new().unwrap().get_text(),
        Clipboard::new().unwrap().get_image(),
    );

    if copy() {
        // Read New Clipboard
        let new_text = Clipboard::new().unwrap().get_text();

        // Create Write Clipboard
        let mut write_clipboard = Clipboard::new().unwrap();

        match old_clipboard {
            (Ok(text), _) => {
                // Old Clipboard is Text
                write_clipboard.set_text(text).unwrap();
                if let Ok(new) = new_text {
                    Ok(new)
                } else {
                    Err("New clipboard is not Text".to_string())
                }
            }
            (_, Ok(image)) => {
                // Old Clipboard is Image
                write_clipboard.set_image(image).unwrap();
                if let Ok(new) = new_text {
                    Ok(new)
                } else {
                    Err("New clipboard is not Text".to_string())
                }
            }
            _ => {
                // Old Clipboard is Empty
                write_clipboard.clear().unwrap();
                if let Ok(new) = new_text {
                    Ok(new)
                } else {
                    Err("New clipboard is not Text".to_string())
                }
            }
        }
    } else {
        Err("Copy Failed".to_string())
    }
}

fn copy() -> bool {
    use windows::Win32::System::DataExchange::GetClipboardSequenceNumber;
    use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    use windows::Win32::UI::Input::KeyboardAndMouse::GetFocus;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowThreadProcessId, SendMessageW, WM_COPY,
    };

    let num_before = unsafe { GetClipboardSequenceNumber() };

    unsafe {
        let window = GetForegroundWindow(); // Gets the currently activated window handle
        let self_thread_id = GetCurrentThreadId(); // Gets the thread ID of itself
        let fore_thread_id = GetWindowThreadProcessId(window, None); // Get the thread ID from the window handle
        AttachThreadInput(fore_thread_id, self_thread_id, true); // Attach thread
        let focused = GetFocus(); // Get the handle of the window with the input focus
        AttachThreadInput(fore_thread_id, self_thread_id, false); // Cancel attach thread
        SendMessageW(focused, WM_COPY, None, None); // Send a copy signal
    }

    let num_after = unsafe { GetClipboardSequenceNumber() };

    if num_before == num_after {
        use enigo::*;
        let mut enigo = Enigo::new();
        enigo.key_up(Key::Control);
        enigo.key_up(Key::Alt);
        enigo.key_up(Key::Shift);
        enigo.key_up(Key::Space);
        enigo.key_up(Key::Meta);
        enigo.key_up(Key::Tab);
        enigo.key_up(Key::Escape);
        enigo.key_up(Key::CapsLock);
        enigo.key_up(Key::C);
        enigo.key_sequence_parse("{+CTRL}c{-CTRL}");
        std::thread::sleep(std::time::Duration::from_millis(100));
        let num_enigo = unsafe { GetClipboardSequenceNumber() };
        num_after != num_enigo
    } else {
        true
    }
}
