pub fn get_text() -> String {
    match get_text_by_automation() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
    match get_text_by_clipboard() {
        Ok(text) => {
            if !text.is_empty() {
                return text;
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
    // Return Empty String
    String::new()
}

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

fn get_text_by_clipboard() -> Result<String, String> {
    use arboard::Clipboard;

    // Read Old Clipboard
    let old_clipboard = (
        Clipboard::new().unwrap().get_text(),
        Clipboard::new().unwrap().get_image(),
    );

    copy();

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
                Ok("".to_string())
            }
        }
        (_, Ok(image)) => {
            // Old Clipboard is Image
            write_clipboard.set_image(image).unwrap();
            if let Ok(new) = new_text {
                Ok(new)
            } else {
                Ok("".to_string())
            }
        }
        _ => {
            // Old Clipboard is Empty
            write_clipboard.clear().unwrap();
            if let Ok(new) = new_text {
                Ok(new)
            } else {
                Ok("".to_string())
            }
        }
    }
}

fn copy() {
    use windows::Win32::System::DataExchange::GetClipboardSequenceNumber;
    use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    use windows::Win32::UI::Input::KeyboardAndMouse::GetFocus;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowThreadProcessId, SendMessageW, WM_COPY,
    };

    let num_before = unsafe { GetClipboardSequenceNumber() };

    unsafe {
        let window = GetForegroundWindow(); // 获得当前激活的窗口句柄
        let self_thread_id = GetCurrentThreadId(); // 获取本身的线程ID
        let fore_thread_id = GetWindowThreadProcessId(window, None); // 根据窗口句柄获取线程ID
        AttachThreadInput(fore_thread_id, self_thread_id, true); // 附加线程
        let focused = GetFocus(); // 获取具有输入焦点的窗口句柄
        AttachThreadInput(fore_thread_id, self_thread_id, false); // 取消附加的线程
        SendMessageW(focused, WM_COPY, None, None); // 发送复制信号
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
    }
}
