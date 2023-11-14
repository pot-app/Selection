use log::{error, info};
use std::env::var;
use std::error::Error;
use std::io::Read;
use std::time::Duration;
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType, Seat};
use wl_clipboard_rs::utils::is_primary_selection_supported;
use x11_clipboard::Clipboard;

pub fn get_text() -> String {
    match var("XDG_SESSION_TYPE") {
        Ok(session_type) => match session_type.as_str() {
            "x11" => match get_text_on_x11() {
                Ok(text) => return text,
                Err(err) => error!("{}", err),
            },
            "wayland" => match get_text_on_wayland() {
                Ok(text) => return text,
                Err(err) => error!("{}", err),
            },
            _ => {
                error!("Unknown Session Type: {session_type}");
            }
        },
        Err(err) => {
            error!("{}", err);
        }
    }
    // Return Empty String
    String::new()
}

fn get_text_on_x11() -> Result<String, Box<dyn Error>> {
    let clipboard = Clipboard::new()?;
    let primary = clipboard.load(
        clipboard.getter.atoms.primary,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
        Duration::from_millis(100),
    )?;
    let result = String::from_utf8_lossy(&primary)
        .trim_matches('\u{0}')
        .trim()
        .to_string();
    Ok(result)
}

fn get_text_on_wayland() -> Result<String, Box<dyn Error>> {
    if let Ok(support) = is_primary_selection_supported() {
        if !support {
            std::env::set_var("XDG_SESSION_TYPE", "x11");
            std::env::set_var("GDK_BACKEND", "x11");
            info!("Primary Selection is not supported. Fallback to use X11 Clipboard");
            return get_text_on_x11();
        }
    } else {
        std::env::set_var("XDG_SESSION_TYPE", "x11");
        std::env::set_var("GDK_BACKEND", "x11");
        info!("Primary Selection is not supported. Fallback to use X11 Clipboard");
        return get_text_on_x11();
    }

    let (mut pipe, _) = get_contents(ClipboardType::Primary, Seat::Unspecified, MimeType::Text)?;
    let mut contents = vec![];
    pipe.read_to_end(&mut contents)?;
    let contents = String::from_utf8_lossy(&contents)
        .trim_matches('\u{0}')
        .trim()
        .to_string();
    Ok(contents)
}
