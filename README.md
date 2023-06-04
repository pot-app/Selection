# Selection

Get the text selected by the cursor

## Example

```rust
fn main() {
    use selection::get_selected_text;
    // Return the selected text on success
    // Otherwise return the empty string
    let text = get_selected_text();
    println!("{}", text);
}
```
