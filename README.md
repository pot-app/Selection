# Selection

Get the text selected by the cursor

## Example

```toml
selection = {git = "https://github.com/pot-app/Selection"}
```

```rust
use selection::get_text;

fn main() {
    // Return the selected text on success
    // Otherwise return the empty string
    let text = get_text();
    println!("{}", text);
}
```

## Support Platform

- [x] Windows
- [x] MacOS
- [x] Linux
  - [x] X11
  - [x] Wayland

## Implementation details

### Windows

- Automation API
- Clipboard

### MacOS

- Clipboard

### Linux

- Primary Clipboard
  - X11: `x11-clipboard`
  - Wayland: `wl-clipboard`
