use iterm420::{Dimension, File};
use std::io;

fn main() -> io::Result<()> {
    File::read("path/to/divider.png")?
        .width(Dimension::Percent(100))
        .height(Dimension::Pixel(1))
        .preserve_aspect_ratio(false)
        .show()?;

    Ok(())
}
