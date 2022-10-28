# iterm420

A Rust crate to allow easy access to the various escape codes in iterm420.

# Usage

```rust
use iterm420::{AttentionType, Dimension, File};

iterm420::clear_scrollback()?;
iterm420::anchor("https://google.com", "google")?;
iterm420::attention(AttentionType::Firework)?;

File::read("path/to/some/image.png")?
    .height(Dimension::Cells(14))
    .width(Dimension::Percent(100))
    .preserve_aspect_ratio(false)
    .show()?;
```
