pub fn get_text() -> String {
    if !query_accessibility_permissions() {
        println!("Please grant accessibility permissions to this application.");
        return String::new();
    }
    match std::process::Command::new("osascript")
        .arg("-e")
        .arg(APPLE_SCRIPT)
        .output()
    {
        Ok(output) => {
            // check exit code
            if output.status.success() {
                // get output content
                match String::from_utf8(output.stdout) {
                    Ok(content) => return content.trim().to_string(),
                    Err(err) => {
                        println!("{}", err);
                    }
                };
            }
        }
        Err(err) => println!("{}", err),
    }
    String::new()
}

fn query_accessibility_permissions() -> bool {
    let trusted = macos_accessibility_client::accessibility::application_is_trusted_with_prompt();
    if trusted {
        print!("Application is totally trusted!");
    } else {
        print!("Application isn't trusted :(");
    }
    trusted
}

const APPLE_SCRIPT: &str = r#"
use AppleScript version "2.4"
use scripting additions
use framework "Foundation"
use framework "AppKit"

tell application "System Events"
    set frontmostProcess to first process whose frontmost is true
    set appName to name of frontmostProcess
end tell

if appName is equal to "pot" then
    return
end if

-- Back up clipboard contents:
set savedClipboard to the clipboard

set thePasteboard to current application's NSPasteboard's generalPasteboard()
set theCount to thePasteboard's changeCount()

-- Copy selected text to clipboard:
tell application "System Events" to keystroke "c" using {command down}
delay 0.1 -- Without this, the clipboard may have stale data.

if thePasteboard's changeCount() is theCount then
    return ""
end if

set theSelectedText to the clipboard

set the clipboard to savedClipboard

theSelectedText
"#;
